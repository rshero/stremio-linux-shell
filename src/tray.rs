use std::{
    sync::mpsc::{Receiver, channel},
    thread,
};

use rust_i18n::t;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};

const ICON: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/icons/symbolic.png"
));

pub enum TrayEvent {
    Quit,
}

pub struct Tray {
    receiver: Receiver<TrayEvent>,
}

impl Tray {
    pub fn new() -> Self {
        let (sender, receiver) = channel::<TrayEvent>();

        thread::spawn(|| {
            gtk::init().expect("Failed to initialize gtk");

            let language = gtk::default_language();
            if let Some(language) = language {
                rust_i18n::set_locale(&language.to_str());
            }

            let _tray = Tray::create();

            gtk::main();
        });

        let menu_sender = sender.clone();
        MenuEvent::set_event_handler(Some(move |_| {
            menu_sender.send(TrayEvent::Quit).ok();
        }));

        Self { receiver }
    }

    fn create() -> TrayIcon {
        let quit_item = MenuItem::new(t!("quit"), true, None);

        let version_label = format!("v{}", env!("CARGO_PKG_VERSION"));
        let version_item = MenuItem::new(version_label.as_str(), false, None);

        let menu = Menu::new();
        menu.append_items(&[&quit_item, &version_item])
            .expect("Failed to append menu items");

        let icon = load_icon(ICON);

        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_icon(icon)
            .build()
            .expect("Failed to build tray icon")
    }

    pub fn events<F: FnMut(TrayEvent)>(&self, handler: F) {
        self.receiver.try_iter().for_each(handler);
    }
}

fn load_icon(buffer: &[u8]) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(buffer)
            .expect("Failed to open icon path")
            .into_rgba8();

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();

        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
