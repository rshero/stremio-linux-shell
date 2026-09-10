mod config;
mod constants;

use std::{env, ffi::CString, os::raw::c_void, rc::Rc, thread, time::Duration};

use crate::config::PlayerConfig;
use config::MpvConfig;
use constants::{
    BOOL_PROPERTIES, FLOAT_PROPERTIES, INT_PROPERTIES, NODE_PROPERTIES, STRING_PROPERTIES,
};
use crossbeam_channel::{Receiver, Sender, unbounded};
use glutin::{display::Display, prelude::GlDisplay};
use itertools::Itertools;
use libc::{LC_NUMERIC, setlocale};
use libmpv2::{
    Format, Mpv,
    events::{Event, EventContext, PropertyData},
    mpv_node::MpvNode,
    render::{OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType},
};
use rust_i18n::t;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use serde_json::{Number, Value};
use tracing::error;

pub type GLContext = Rc<Display>;

#[derive(Debug)]
pub enum MpvPropertyValue {
    Float(f64),
    Int(i64),
    Bool(bool),
    String(String),
}

impl Serialize for MpvPropertyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MpvPropertyValue::Float(value) => serializer.serialize_f64(*value),
            MpvPropertyValue::Int(value) => serializer.serialize_i64(*value),
            MpvPropertyValue::Bool(value) => serializer.serialize_bool(*value),
            MpvPropertyValue::String(value) => {
                if let Ok(json_value) = serde_json::from_str::<Value>(value) {
                    json_value.serialize(serializer)
                } else {
                    serializer.serialize_str(value)
                }
            }
        }
    }
}

fn node_to_json(node: MpvNode) -> Value {
    match node {
        MpvNode::String(value) => Value::String(value),
        MpvNode::Flag(value) => Value::Bool(value),
        MpvNode::Int64(value) => Value::Number(value.into()),
        MpvNode::Double(value) => Number::from_f64(value).map_or(Value::Null, Value::Number),
        MpvNode::ArrayIter(values) => Value::Array(values.map(node_to_json).collect()),
        MpvNode::MapIter(values) => Value::Object(
            values
                .map(|(key, value)| (key, node_to_json(value)))
                .collect(),
        ),
        MpvNode::None => Value::Null,
    }
}

#[derive(Deserialize, Debug)]
pub struct MpvProperty(pub String, pub Option<Value>);

impl MpvProperty {
    pub fn name(&self) -> &str {
        self.0.as_ref()
    }

    pub fn value(&self) -> Result<MpvPropertyValue, &'static str> {
        if let Some(value) = self.1.clone() {
            if FLOAT_PROPERTIES.contains(&self.name()) {
                return serde_json::from_value::<f64>(value)
                    .map(MpvPropertyValue::Float)
                    .map_err(|_| "Failed to get f64 from Value");
            }

            if INT_PROPERTIES.contains(&self.name()) {
                return serde_json::from_value::<i64>(value)
                    .map(MpvPropertyValue::Int)
                    .map_err(|_| "Failed to get i64 from Value");
            }

            if BOOL_PROPERTIES.contains(&self.name()) {
                return serde_json::from_value::<bool>(value)
                    .map(MpvPropertyValue::Bool)
                    .map_err(|_| "Failed to get bool from Value");
            }

            if STRING_PROPERTIES.contains(&self.name()) {
                return serde_json::from_value::<String>(value)
                    .map(MpvPropertyValue::String)
                    .map_err(|_| "Failed to get String from Value");
            }
        }

        Err("Failed to get value of MpvProperty")
    }
}

impl Serialize for MpvProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MpvProperty", 2)?;
        state.serialize_field("name", self.name())?;

        if let Ok(value) = self.value() {
            state.serialize_field("data", &value)?;
        } else if let Some(value) = &self.1 {
            state.serialize_field("data", value)?;
        }

        state.end()
    }
}

#[derive(Debug)]
pub enum PlayerEvent {
    Start,
    Stop(Option<String>),
    Update,
    PropertyChange(MpvProperty),
}

impl<'a> TryFrom<Event<'a>> for PlayerEvent {
    type Error = &'static str;

    fn try_from(value: Event<'a>) -> Result<Self, Self::Error> {
        match value {
            Event::StartFile => Ok(PlayerEvent::Start),
            Event::EndFile(code) => {
                let error = match code {
                    3 => Some(t!("player_error_quit")),
                    4 => Some(t!("player_error_general")),
                    _ => None,
                };

                Ok(PlayerEvent::Stop(error.map(String::from)))
            }
            Event::PropertyChange { name, change, .. } => {
                let property = match change {
                    PropertyData::Double(value) => MpvProperty(
                        name.to_owned(),
                        Some(Value::Number(Number::from_f64(value).unwrap())),
                    ),
                    PropertyData::Int64(value) => {
                        MpvProperty(name.to_owned(), Some(Value::Number(Number::from(value))))
                    }
                    PropertyData::Flag(value) => {
                        MpvProperty(name.to_owned(), Some(Value::Bool(value)))
                    }
                    PropertyData::Str(value) => {
                        MpvProperty(name.to_owned(), Some(Value::String(value.to_owned())))
                    }
                    PropertyData::Node(value) => {
                        MpvProperty(name.to_owned(), Some(node_to_json(value)))
                    }
                    _ => return Err("Property not supported"),
                };

                Ok(PlayerEvent::PropertyChange(property))
            }
            _ => Err("Event not supported"),
        }
    }
}

pub struct Player {
    mpv: Mpv,
    event_context: EventContext,
    render_context: Option<RenderContext>,
    sender: Sender<PlayerEvent>,
    receiver: Receiver<PlayerEvent>,
}

impl Player {
    pub fn new(player_config: PlayerConfig) -> Self {
        // Ensure C locale for MPV — GTK/CEF may have changed it since main() set it
        unsafe {
            setlocale(LC_NUMERIC, c"C".as_ptr());
        }

        // Initialize MPV config (creates config directory and installs defaults)
        let mpv_config =
            MpvConfig::new(&player_config.data_dir).expect("Failed to initialize MPV config");

        println!(
            "🎬 MPV Enhanced - Config loaded from: {}",
            mpv_config.config_dir_str()
        );

        let msg_level = match env::var("RUST_LOG") {
            Ok(scope) => format!("all={},cplayer=v,lua=v", scope),
            Err(_) => "all=warn".to_owned(),
        };

        let config_dir = mpv_config.config_dir_str();

        // Retry mpv creation — on first boot after reboot, locale can be
        // reset by GTK between our setlocale call and mpv_create().
        const MAX_RETRIES: u32 = 3;
        let mut mpv = None;
        for attempt in 0..MAX_RETRIES {
            // Re-force locale before each attempt
            unsafe {
                setlocale(LC_NUMERIC, c"C".as_ptr());
            }

            let msg = msg_level.clone();
            let dir = config_dir.clone();
            match Mpv::with_initializer(move |init| {
                init.set_property("vo", "libmpv")?;
                init.set_property("video-timing-offset", "0")?;
                init.set_property("terminal", "yes")?;
                init.set_property("msg-level", msg.as_str())?;
                // Enable config file loading from custom directory
                init.set_property("config-dir", dir.as_str())?;
                init.set_property("config", "yes")?;
                init.set_property("load-scripts", "yes")?;
                // Enable input.conf processing for keyboard shortcuts
                init.set_property("input-default-bindings", "yes")?;
                init.set_property("input-vo-keyboard", "yes")?;
                Ok(())
            }) {
                Ok(m) => {
                    mpv = Some(m);
                    break;
                }
                Err(e) => {
                    error!(
                        "Failed to create mpv (attempt {}/{}): {:?}",
                        attempt + 1,
                        MAX_RETRIES,
                        e
                    );
                    if attempt + 1 < MAX_RETRIES {
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        }
        let mpv = mpv.expect("Failed to create mpv after multiple attempts — is libmpv installed?");

        let event_context = EventContext::new(mpv.ctx);
        event_context
            .disable_deprecated_events()
            .expect("Failed to disable deprecated events");

        let (sender, receiver) = unbounded::<PlayerEvent>();

        Self {
            mpv,
            event_context,
            render_context: None,
            sender,
            receiver,
        }
    }

    pub fn setup(&mut self, context: GLContext) {
        self.render_context.take();

        fn get_proc_address(context: &GLContext, name: &str) -> *mut c_void {
            let procname = CString::new(name).unwrap();
            context.get_proc_address(procname.as_c_str()) as _
        }

        let mpv_handle = unsafe { self.mpv.ctx.as_mut() };

        let mut render_context = RenderContext::new(
            mpv_handle,
            vec![
                RenderParam::ApiType(RenderParamApiType::OpenGl),
                RenderParam::InitParams(OpenGLInitParams {
                    get_proc_address,
                    ctx: context,
                }),
                RenderParam::BlockForTargetTime(false),
                // RenderParam::AdvancedControl(true),
            ],
        )
        .expect("Failed to create render context");

        let sender = self.sender.clone();
        render_context.set_update_callback(move || {
            sender.send(PlayerEvent::Update).ok();
            crate::shared::wake_event_loop();
        });

        self.render_context = Some(render_context);
    }

    pub fn render(&self, fbo: u32, width: i32, height: i32) {
        if let Some(render_context) = self.render_context.as_ref() {
            render_context
                .render::<GLContext>(fbo as i32, width, height, false)
                .expect("Failed to draw on glutin window");
        }
    }

    pub fn report_swap(&self) {
        if let Some(render_context) = self.render_context.as_ref() {
            render_context.report_swap();
        }
    }

    pub fn events<T: FnMut(PlayerEvent)>(&mut self, mut handler: T) {
        self.receiver.try_iter().for_each(&mut handler);

        while let Some(result) = self.event_context.wait_event(0.0) {
            match result {
                Ok(event) => {
                    if let Ok(player_event) = PlayerEvent::try_from(event) {
                        handler(player_event);
                    }
                }
                Err(e) => eprintln!("Mpv error: {e}"),
            }
        }
    }

    pub fn command(&self, name: String, args: Vec<String>) {
        let args = args.iter().map(String::as_ref).collect_vec();
        if let Err(e) = self.mpv.command(&name, &args) {
            error!("Failed to use command {name} with args {:?}: {e}", args);
        }
    }

    pub fn observe_property(&self, name: String) {
        let format = match name.as_str() {
            name if FLOAT_PROPERTIES.contains(&name) => Some(Format::Double),
            name if INT_PROPERTIES.contains(&name) => Some(Format::Int64),
            name if BOOL_PROPERTIES.contains(&name) => Some(Format::Flag),
            name if STRING_PROPERTIES.contains(&name) => Some(Format::String),
            name if NODE_PROPERTIES.contains(&name) => Some(Format::Node),
            _ => None,
        };

        if let Some(format) = format
            && let Err(e) = self.event_context.observe_property(&name, format, 0)
        {
            error!("Failed to observe property {name}: {e}");
        }
    }

    pub fn set_property(&self, property: MpvProperty) {
        match property.name() {
            name if FLOAT_PROPERTIES.contains(&name) => {
                if let Ok(MpvPropertyValue::Float(value)) = property.value()
                    && let Err(e) = self.mpv.set_property(name, value)
                {
                    error!("Failed to set property {name}: {e}");
                }
            }
            name if INT_PROPERTIES.contains(&name) => {
                if let Ok(MpvPropertyValue::Int(value)) = property.value()
                    && let Err(e) = self.mpv.set_property(name, value)
                {
                    error!("Failed to set property {name}: {e}");
                }
            }
            name if BOOL_PROPERTIES.contains(&name) => {
                if let Ok(MpvPropertyValue::Bool(value)) = property.value()
                    && let Err(e) = self.mpv.set_property(name, value)
                {
                    error!("Failed to set property {name}: {e}");
                }
            }
            name if STRING_PROPERTIES.contains(&name) => {
                if let Ok(MpvPropertyValue::String(value)) = property.value()
                    && let Err(e) = self.mpv.set_property(name, value)
                {
                    error!("Failed to set property {name}: {e}");
                }
            }
            name => error!("Failed to set property {name}: Unsupported"),
        };
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.render_context.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_node_property_data_for_web_player() {
        let property = MpvProperty(
            "video-params".to_owned(),
            Some(json!({ "w": 1920, "h": 1080 })),
        );

        assert_eq!(
            serde_json::to_value(property).unwrap(),
            json!({
                "name": "video-params",
                "data": { "w": 1920, "h": 1080 },
            }),
        );
    }

    #[test]
    fn accepts_paused_for_cache_as_boolean() {
        let property = MpvProperty("paused-for-cache".to_owned(), Some(json!(false)));

        assert!(matches!(
            property.value(),
            Ok(MpvPropertyValue::Bool(false))
        ));
    }
}
