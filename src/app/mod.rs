mod adapters;
mod utils;

use std::{ffi::CString, path::PathBuf};

use ashpd::{
    WindowIdentifier,
    desktop::{
        Request,
        background::Background,
        inhibit::{InhibitFlags, InhibitProxy},
        open_uri::OpenFileRequest,
    },
    enumflags2::BitFlags,
};
use crossbeam_channel::{Receiver, Sender, unbounded};
use glutin::{
    context::{ContextApi, Version},
    display::GetGlDisplay,
    prelude::GlDisplay,
};
use tracing::error;
use url::Url;
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, Touch, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::ModifiersState,
    platform::wayland::WindowAttributesExtWayland,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::{
        CursorIcon, Fullscreen, Icon as WindowIcon, UserAttentionType, Window, WindowAttributes,
    },
};

use crate::{
    config,
    config::AppConfig,
    constants::{APP_ID, APP_NAME, WINDOW_SIZE},
    shared::{
        self,
        types::{Cursor, MouseState, UserEvent, WindowSize},
    },
};

const CONTEXT_API: ContextApi = ContextApi::OpenGl(Some(Version::new(3, 3)));

// Embed window icon at compile time (128x128 for better compatibility)
const WINDOW_ICON: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/icons/window-icon-128.png"
));

#[derive(Debug)]
pub enum AppEvent {
    Init,
    Ready,
    Resized(WindowSize),
    Focused(bool),
    Visibility(bool),
    Minimized(bool),
    Fullscreen(bool),
    MouseMoved(MouseState),
    MouseWheel(MouseState),
    MouseInput(MouseState),
    TouchInput(Touch),
    KeyboardInput((KeyEvent, ModifiersState)),
    FileHover((PathBuf, MouseState)),
    FileDrop(MouseState),
    FileCancel,
}

pub struct App {
    window: Option<Window>,
    sender: Sender<AppEvent>,
    receiver: Receiver<AppEvent>,
    maximized: bool,
    modifiers_state: ModifiersState,
    mouse_state: MouseState,
    inhibit_request: Option<Request<()>>,
    config: AppConfig,
}

impl App {
    pub fn new(config: &AppConfig) -> Self {
        let (sender, receiver) = unbounded::<AppEvent>();

        Self {
            window: None,
            sender,
            receiver,
            maximized: false,
            modifiers_state: ModifiersState::empty(),
            mouse_state: MouseState::default(),
            inhibit_request: None,
            config: config.clone(),
        }
    }

    pub fn events<T: FnMut(AppEvent)>(&self, handler: T) {
        self.receiver.try_iter().for_each(handler);
    }

    pub fn save_window_state(&mut self) {
        if let Some(window) = self.window.as_ref() {
            let position = window.outer_position().unwrap_or_default();
            let size = window.inner_size(); // Use inner_size instead of outer_size
            let maximized = window.is_maximized();

            let window_config = config::WindowConfig {
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
                maximized,
            };

            self.config.set_window(window_config);
        }
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        // Load window icon
        let window_icon = Self::load_window_icon();

        // Get window config with validation
        let window_config = &self.config.window;
        
        // Validate window config - fall back to defaults if invalid
        let (x, y, width, height, maximized) = Self::validate_window_config(
            window_config.x,
            window_config.y,
            window_config.width,
            window_config.height,
            window_config.maximized,
            event_loop,
        );

        // Update maximized state
        self.maximized = maximized;

        let mut window_attributes = WindowAttributes::default()
            .with_title(APP_NAME)
            .with_name(APP_ID, APP_ID)
            .with_decorations(true)
            .with_resizable(true)
            .with_maximized(maximized)
            .with_min_inner_size(PhysicalSize::new(900, 600))
            .with_inner_size(PhysicalSize::new(width, height));

        // Set position (centered or explicit)
        if x >= 0 && y >= 0 {
            window_attributes = window_attributes.with_position(PhysicalPosition::new(x, y));
        } else {
            // Center on primary monitor - winit centers by default when no position is set
            // No need to explicitly set position, let the window manager handle it
        }

        // Set window icon if loaded successfully
        if let Some(icon) = window_icon {
            window_attributes = window_attributes.with_window_icon(Some(icon));
        }

        let (window, config) = utils::create_window(event_loop, window_attributes);
        let surface = utils::create_surface(&config, &window);
        let context = utils::create_context(&config, CONTEXT_API);

        gl::load_with(|name| {
            let name = CString::new(name).unwrap();
            context.display().get_proc_address(&name) as _
        });

        self.window = window;
        
        // Update maximized state after window creation (might have changed due to WM)
        if let Some(window) = self.window.as_ref() {
            self.maximized = window.is_maximized();
        }
        
        self.sender.send(AppEvent::Visibility(true)).ok();

        shared::create_gl(surface, context);
        shared::with_gl(|_, _| {
            let refresh_rate = self.get_refresh_rate();
            shared::create_renderer((width as i32, height as i32), refresh_rate);
        });

        self.sender.send(AppEvent::Ready).ok();
    }

    fn validate_window_config(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        maximized: bool,
        event_loop: &ActiveEventLoop,
    ) -> (i32, i32, u32, u32, bool) {
        // Validate values
        let valid_position = x >= 0 && y >= 0;
        let valid_size = width >= 900 && height >= 600;
        
        // Check reasonable bounds - window should not be larger than primary monitor
        let mut reasonable_bounds = true;
        if let Some(monitor) = event_loop.primary_monitor() {
            let monitor_size = monitor.size();
            if width > monitor_size.width || height > monitor_size.height {
                reasonable_bounds = false;
            }
        }

        if valid_position && valid_size && reasonable_bounds {
            (x, y, width, height, maximized)
        } else {
            // Return defaults from constants - will be centered
            (-1, -1, width.max(900).min(WINDOW_SIZE.0 as u32), height.max(600).min(WINDOW_SIZE.1 as u32), maximized)
        }
    }

    pub fn destroy_window(&mut self) {
        shared::drop_renderer();
        shared::drop_gl();

        self.window.take();
        self.sender.send(AppEvent::Visibility(false)).ok();
    }

    pub fn notify(&self) {
        if let Some(window) = self.window.as_ref() {
            window.request_user_attention(Some(UserAttentionType::Informational));
        }
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        if let Some(window) = self.window.as_ref() {
            if let Ok(icon) = TryInto::<CursorIcon>::try_into(cursor) {
                window.set_cursor(icon);
                window.set_cursor_visible(true);
            } else {
                window.set_cursor_visible(false);
            }
        }
    }

    pub fn set_fullscreen(&self, state: bool) {
        if let Some(window) = self.window.as_ref() {
            let fullscreen = match state {
                true => Some(Fullscreen::Borderless(None)),
                false => None,
            };

            window.set_fullscreen(fullscreen);
        }
    }

    pub fn toggle_fullscreen(&self) {
        if let Some(window) = self.window.as_ref() {
            let is_fullscreen = window.fullscreen().is_some();
            self.set_fullscreen(!is_fullscreen);
        }
    }

    pub fn get_refresh_rate(&self) -> u32 {
        if let Some(window) = self.window.as_ref() {
            for monitor in window.available_monitors() {
                if let Some(m_hz) = monitor.refresh_rate_millihertz() {
                    return m_hz / 1000;
                }
            }
        }

        30
    }

    pub async fn disable_idling(&mut self) {
        if let Some(identifier) = self.window_identifier().await
            && let Ok(proxy) = InhibitProxy::new().await
        {
            let mut flags = BitFlags::empty();
            flags.insert(InhibitFlags::Idle);

            let reason = "Prevent screen from going blank during media playback";

            self.inhibit_request = proxy
                .inhibit(Some(&identifier), flags, reason)
                .await
                .map_err(|e| error!("Failed to prevent idling: {e}"))
                .ok();
        }
    }

    pub async fn enable_idling(&mut self) {
        if let Some(request) = self.inhibit_request.take() {
            request
                .close()
                .await
                .map_err(|e| error!("Failed to allow idling: {e}"))
                .ok();
        }
    }

    pub async fn open_url<T: Into<String>>(&self, input: T) {
        if let Ok(url) = Url::parse(&input.into())
            && let Some(identifier) = self.window_identifier().await
        {
            let request = OpenFileRequest::default().identifier(identifier);

            request
                .send_uri(&url)
                .await
                .map_err(|e| error!("Failed to open uri: {e}"))
                .ok();
        }
    }

    async fn request_background(&self) {
        if let Some(identifier) = self.window_identifier().await {
            let request = Background::request().identifier(identifier);

            request
                .send()
                .await
                .map_err(|e| error!("Failed to set background mode: {e}"))
                .ok();
        }
    }

    async fn window_identifier(&self) -> Option<WindowIdentifier> {
        if let Some(window) = self.window.as_ref()
            && let (Ok(window), Ok(display)) = (window.window_handle(), window.display_handle())
        {
            let window_handle = window.as_raw();
            let display_handle = display.as_raw();

            return WindowIdentifier::from_raw_handle(&window_handle, Some(&display_handle)).await;
        }

        None
    }

    fn load_window_icon() -> Option<WindowIcon> {
        use image::ImageReader;
        use std::io::Cursor;

        println!("🔍 Loading window icon ({} bytes)...", WINDOW_ICON.len());

        // Load embedded PNG icon
        match ImageReader::new(Cursor::new(WINDOW_ICON)).with_guessed_format() {
            Ok(reader) => match reader.decode() {
                Ok(img) => {
                    let rgba = img.to_rgba8();
                    let (width, height) = rgba.dimensions();
                    match WindowIcon::from_rgba(rgba.into_raw(), width, height) {
                        Ok(icon) => {
                            println!("✅ Window icon loaded: {}x{}", width, height);
                            Some(icon)
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to create window icon: {:?}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to decode icon: {:?}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("❌ Failed to read icon: {:?}", e);
                None
            }
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.window.take();
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);

        futures::executor::block_on(self.request_background());

        self.sender.send(AppEvent::Init).ok();
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers_state = modifiers.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.sender
                    .send(AppEvent::KeyboardInput((event, self.modifiers_state)))
                    .ok();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_state.over = true;
                self.mouse_state.position = position.into();
                self.sender
                    .send(AppEvent::MouseMoved(self.mouse_state))
                    .ok();
            }
            WindowEvent::CursorLeft { .. } => {
                self.mouse_state.over = false;
                self.sender
                    .send(AppEvent::MouseMoved(self.mouse_state))
                    .ok();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.mouse_state.delta = delta.into();
                self.sender
                    .send(AppEvent::MouseWheel(self.mouse_state))
                    .ok();
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.mouse_state.button = button;
                self.mouse_state.pressed = match state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };

                self.sender
                    .send(AppEvent::MouseInput(self.mouse_state))
                    .ok();
            }
            WindowEvent::Touch(touch) => {
                self.sender.send(AppEvent::TouchInput(touch)).ok();
            }
            WindowEvent::HoveredFile(path) => {
                self.sender
                    .send(AppEvent::FileHover((path, self.mouse_state)))
                    .ok();
            }
            WindowEvent::DroppedFile(_) => {
                self.sender.send(AppEvent::FileDrop(self.mouse_state)).ok();
            }
            WindowEvent::HoveredFileCancelled => {
                self.sender.send(AppEvent::FileCancel).ok();
            }
            WindowEvent::Resized(size) => {
                self.sender.send(AppEvent::Resized(size.into())).ok();
            }
            WindowEvent::Focused(state) => {
                self.sender.send(AppEvent::Focused(state)).ok();

                if let Some(window) = self.window.as_ref() {
                    self.maximized = window.is_maximized();

                    let minimized = window.is_minimized().unwrap_or(false);
                    self.sender.send(AppEvent::Minimized(minimized)).ok();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    let fullscreen = window.fullscreen().is_some();
                    self.sender.send(AppEvent::Fullscreen(fullscreen)).ok();
                }
            }
            WindowEvent::CloseRequested => {
                self.save_window_state();
                self.destroy_window();
            }
            _ => (),
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Raise => match self.window.is_some() {
                true => self.notify(),
                false => self.create_window(event_loop),
            },
            UserEvent::Show => {
                self.create_window(event_loop);
            }
            UserEvent::Hide => {
                self.destroy_window();
            }
            UserEvent::Quit => {
                event_loop.exit();
            }
            UserEvent::Redraw => {}
        }
    }
}
