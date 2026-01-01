mod app;
mod config;
mod constants;
mod discord;
mod gpu;
mod instance;
mod ipc;
mod player;
mod server;
mod shared;
mod tray;
mod webview;

use app::{App, AppEvent};
use clap::Parser;
use config::Config;
use constants::{STARTUP_URL, URI_SCHEME};
use discord::Discord;
use glutin::{display::GetGlDisplay, surface::GlSurface};
use instance::{Instance, InstanceEvent};
use ipc::{IpcEvent, IpcEventMpv};
use player::{Player, PlayerEvent};
use rust_i18n::i18n;
use server::Server;
use shared::{types::UserEvent, with_gl, with_renderer_read, with_renderer_write};
use std::{num::NonZeroU32, process::ExitCode, rc::Rc, time::Duration};
use tray::Tray;
use webview::{WebView, WebViewEvent};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

i18n!("locales", fallback = "en");

#[derive(Parser, Debug)]
#[command(version, ignore_errors(true))]
struct Args {
    /// Open dev tools
    #[arg(short, long)]
    dev: bool,
    /// Startup url
    #[arg(short, long, default_value = STARTUP_URL)]
    url: String,
    /// Open a deeplink
    #[arg(short, long)]
    open: Option<String>,
    /// Disable server
    #[arg(short, long)]
    no_server: bool,
}

/// Converts KeyCode to MPV key name for input.conf bindings
fn keycode_to_mpv_key(key_code: KeyCode) -> Option<String> {
    let key_str = match key_code {
        // Letters
        KeyCode::KeyA => "a", KeyCode::KeyB => "b", KeyCode::KeyC => "c", KeyCode::KeyD => "d",
        KeyCode::KeyE => "e", KeyCode::KeyF => "f", KeyCode::KeyG => "g", KeyCode::KeyH => "h",
        KeyCode::KeyI => "i", KeyCode::KeyJ => "j", KeyCode::KeyK => "k", KeyCode::KeyL => "l",
        KeyCode::KeyM => "m", KeyCode::KeyN => "n", KeyCode::KeyO => "o", KeyCode::KeyP => "p",
        KeyCode::KeyQ => "q", KeyCode::KeyR => "r", KeyCode::KeyS => "s", KeyCode::KeyT => "t",
        KeyCode::KeyU => "u", KeyCode::KeyV => "v", KeyCode::KeyW => "w", KeyCode::KeyX => "x",
        KeyCode::KeyY => "y", KeyCode::KeyZ => "z",
        // Numbers
        KeyCode::Digit0 => "0", KeyCode::Digit1 => "1", KeyCode::Digit2 => "2", KeyCode::Digit3 => "3",
        KeyCode::Digit4 => "4", KeyCode::Digit5 => "5", KeyCode::Digit6 => "6", KeyCode::Digit7 => "7",
        KeyCode::Digit8 => "8", KeyCode::Digit9 => "9",
        // Function keys
        KeyCode::F1 => "F1", KeyCode::F2 => "F2", KeyCode::F3 => "F3", KeyCode::F4 => "F4",
        KeyCode::F5 => "F5", KeyCode::F6 => "F6", KeyCode::F7 => "F7", KeyCode::F8 => "F8",
        KeyCode::F9 => "F9", KeyCode::F10 => "F10", KeyCode::F11 => "F11", KeyCode::F12 => "F12",
        // Special keys
        KeyCode::Space => "SPACE",
        KeyCode::Enter => "ENTER",
        KeyCode::Backspace => "BS",
        KeyCode::Tab => "TAB",
        KeyCode::Escape => "ESC",
        KeyCode::Insert => "INS",
        KeyCode::Delete => "DEL",
        KeyCode::Home => "HOME",
        KeyCode::End => "END",
        KeyCode::PageUp => "PGUP",
        KeyCode::PageDown => "PGDWN",
        // Arrow keys
        KeyCode::ArrowLeft => "LEFT",
        KeyCode::ArrowRight => "RIGHT",
        KeyCode::ArrowUp => "UP",
        KeyCode::ArrowDown => "DOWN",
        // Punctuation
        KeyCode::Minus => "-",
        KeyCode::Equal => "=",
        KeyCode::BracketLeft => "[",
        KeyCode::BracketRight => "]",
        KeyCode::Backslash => "\\",
        KeyCode::Semicolon => ";",
        KeyCode::Quote => "'",
        KeyCode::Backquote => "`",
        KeyCode::Comma => ",",
        KeyCode::Period => ".",
        KeyCode::Slash => "/",
        _ => return None,
    };
    Some(key_str.to_string())
}

/// Returns the Anime4K shader command for a given key
fn get_anime4k_shader_command(key_code: KeyCode) -> Option<(&'static str, &'static str)> {
    match key_code {
        KeyCode::Digit0 => Some(("clr", "Shaders cleared")),
        KeyCode::Digit1 => Some((
            "set",
            "~~/shaders/anime4k/Restore/Anime4K_Clamp_Highlights.glsl;~~/shaders/anime4k/Restore/Anime4K_Restore_CNN_VL.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_VL.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x2.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x4.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl",
        )),
        KeyCode::Digit2 => Some((
            "set",
            "~~/shaders/anime4k/Restore/Anime4K_Clamp_Highlights.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_Denoise_CNN_x2_VL.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x2.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x4.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl",
        )),
        KeyCode::Digit3 => Some((
            "set",
            "~~/shaders/anime4k/Restore/Anime4K_Clamp_Highlights.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_VL.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x2.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x4.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl",
        )),
        KeyCode::Digit4 => Some((
            "set",
            "~~/shaders/anime4k/Restore/Anime4K_Clamp_Highlights.glsl;~~/shaders/anime4k/Restore/Anime4K_Restore_CNN_M.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x2.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x4.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl",
        )),
        KeyCode::Digit5 => Some((
            "set",
            "~~/shaders/anime4k/Restore/Anime4K_Clamp_Highlights.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_Denoise_CNN_x2_M.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x2.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x4.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl",
        )),
        KeyCode::Digit6 => Some((
            "set",
            "~~/shaders/anime4k/Restore/Anime4K_Clamp_Highlights.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x2.glsl;~~/shaders/anime4k/Restore/Anime4K_AutoDownscalePre_x4.glsl;~~/shaders/anime4k/Upscale/Anime4K_Upscale_CNN_x2_M.glsl",
        )),
        _ => None,
    }
}

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config = Config::new();

    let mut webview = WebView::new(config.webview);
    if webview.should_exit() {
        return ExitCode::SUCCESS;
    }

    let instance = Instance::new(config.instance);
    if instance.running() {
        if let Some(deeplink) = args.open {
            instance.send(deeplink);
        }

        return ExitCode::SUCCESS;
    }

    instance.start();

    let mut server = Server::new(config.server);
    if !args.no_server {
        server.start(args.dev).expect("Failed to start server");
    }

    let tray = Tray::new(config.tray);
    let mut app = App::new();
    let mut player = Player::new(config.player);
    let mut is_playing = false; // Track if video is actively playing

    // Discord needs to be in an Rc<RefCell<>> to be accessed from closures
    use std::cell::RefCell;
    let discord = Rc::new(RefCell::new(Discord::new(config.discord.enabled)));
    let discord_clone = discord.clone();
    let discord_clone2 = discord.clone();

    let mut event_loop = EventLoop::<UserEvent>::with_user_event()
        .build()
        .expect("Failed to create event loop");

    event_loop.set_control_flow(ControlFlow::Poll);

    let event_loop_proxy = event_loop.create_proxy();

    let mut needs_redraw = false;

    loop {
        let timeout = match needs_redraw {
            true => Some(Duration::ZERO),
            false => None,
        };

        let status = event_loop.pump_app_events(timeout, &mut app);

        if let PumpStatus::Exit(exit_code) = status {
            server.stop().expect("Failed to stop server");
            webview.stop();
            instance.stop();
            shared::drop_renderer();
            shared::drop_gl();

            break ExitCode::from(exit_code as u8);
        }

        if needs_redraw {
            with_gl(|surface, context| {
                with_renderer_read(|renderer| {
                    player.render(renderer.fbo, renderer.width, renderer.height);
                    renderer.draw();
                });

                surface
                    .swap_buffers(context)
                    .expect("Failed to swap buffers");

                player.report_swap();
            });

            needs_redraw = false;
        }

        instance.events(|event| match event {
            InstanceEvent::Open(deeplink) => {
                event_loop_proxy.send_event(UserEvent::Raise).ok();

                if deeplink.starts_with(URI_SCHEME) {
                    let message = ipc::create_response(IpcEvent::OpenMedia(deeplink.to_string()));
                    webview.post_message(message);
                }
            }
        });

        tray.events(|event| {
            event_loop_proxy.send_event(event).ok();
        });

        app.events(|event| match event {
            AppEvent::Init => {
                webview.start();
            }
            AppEvent::Ready => {
                shared::with_gl(|surface, _| {
                    player.setup(Rc::new(surface.display()));
                });
            }
            AppEvent::Resized(size) => {
                with_gl(|surface, context| {
                    surface.resize(
                        context,
                        NonZeroU32::new(size.0 as u32).unwrap(),
                        NonZeroU32::new(size.1 as u32).unwrap(),
                    );

                    with_renderer_write(|renderer| {
                        renderer.resize(size.0, size.1);
                    });

                    webview.update();
                    needs_redraw = true;
                });
            }
            AppEvent::Focused(state) => {
                webview.focused(state);
            }
            AppEvent::Visibility(visible) => {
                let message = ipc::create_response(IpcEvent::Visibility(visible));
                webview.post_message(message);

                tray.update(visible);
            }
            AppEvent::Minimized(minimized) => {
                let message = ipc::create_response(IpcEvent::Minimized(minimized));
                webview.post_message(message);
            }
            AppEvent::Fullscreen(fullscreen) => {
                let message = ipc::create_response(IpcEvent::Fullscreen(fullscreen));
                webview.post_message(message);
            }
            AppEvent::MouseMoved(state) => {
                webview.mouse_moved(state);
            }
            AppEvent::MouseWheel(state) => {
                webview.mouse_wheel(state);
            }
            AppEvent::MouseInput(state) => {
                webview.mouse_input(state);
            }
            AppEvent::TouchInput(touch) => {
                webview.touch_input(touch);
            }
            AppEvent::KeyboardInput((key_event, modifiers)) => {
                // Intercept Ctrl+V for clipboard paste from system
                // CEF in windowless mode doesn't automatically sync with system clipboard
                if modifiers.control_key() && key_event.state.is_pressed() {
                    if let PhysicalKey::Code(KeyCode::KeyV) = key_event.physical_key {
                        webview.paste_from_clipboard();
                        return; // Don't forward to CEF or webview
                    }
                }

                // Intercept Ctrl+0-6 for Anime4K shader switching
                if modifiers.control_key() && key_event.state.is_pressed() {
                    if let PhysicalKey::Code(key_code) = key_event.physical_key {
                        if let Some((action, _label)) = get_anime4k_shader_command(key_code) {
                            if action == "clr" {
                                println!("🎨 [ANIME4K] Clearing all shaders");
                                player.command(
                                    "change-list".to_string(),
                                    vec![
                                        "glsl-shaders".to_string(),
                                        "clr".to_string(),
                                        "".to_string(),
                                    ],
                                );
                                player.command(
                                    "show-text".to_string(),
                                    vec!["Shaders cleared".to_string()],
                                );
                            } else {
                                let shader_list = _label;
                                let mode_label = match key_code {
                                    KeyCode::Digit1 => "Anime4K: Mode A (HQ)",
                                    KeyCode::Digit2 => "Anime4K: Mode B (HQ+Denoise)",
                                    KeyCode::Digit3 => "Anime4K: Mode C (Fast)",
                                    KeyCode::Digit4 => "Anime4K: Mode A+A (HQ)",
                                    KeyCode::Digit5 => "Anime4K: Mode B+B (HQ+Denoise)",
                                    KeyCode::Digit6 => "Anime4K: Mode C+A (Fast)",
                                    _ => "Anime4K",
                                };
                                println!("🎨 [ANIME4K] Activating: {}", mode_label);
                                player.command(
                                    "change-list".to_string(),
                                    vec![
                                        "glsl-shaders".to_string(),
                                        "set".to_string(),
                                        shader_list.to_string(),
                                    ],
                                );
                                player
                                    .command("show-text".to_string(), vec![mode_label.to_string()]);
                            }
                            return; // Don't forward to webview
                        }
                    }
                }

                // Forward keypresses to MPV ONLY when video is playing
                if is_playing && key_event.state.is_pressed() {
                    if let PhysicalKey::Code(key_code) = key_event.physical_key {
                        if let Some(mut mpv_key) = keycode_to_mpv_key(key_code) {
                            // Handle modifiers (MPV format: CTRL+s, Shift+g, etc.)
                            // For letters with Shift, MPV expects uppercase letter (z -> Z)
                            // For other keys, use explicit Shift+ prefix
                            let is_letter = matches!(key_code,
                                KeyCode::KeyA | KeyCode::KeyB | KeyCode::KeyC | KeyCode::KeyD |
                                KeyCode::KeyE | KeyCode::KeyF | KeyCode::KeyG | KeyCode::KeyH |
                                KeyCode::KeyI | KeyCode::KeyJ | KeyCode::KeyK | KeyCode::KeyL |
                                KeyCode::KeyM | KeyCode::KeyN | KeyCode::KeyO | KeyCode::KeyP |
                                KeyCode::KeyQ | KeyCode::KeyR | KeyCode::KeyS | KeyCode::KeyT |
                                KeyCode::KeyU | KeyCode::KeyV | KeyCode::KeyW | KeyCode::KeyX |
                                KeyCode::KeyY | KeyCode::KeyZ
                            );

                            if modifiers.shift_key() && is_letter {
                                // For letters, Shift means uppercase (z -> Z)
                                mpv_key = mpv_key.to_uppercase();
                            } else if modifiers.shift_key() {
                                // For non-letters, use Shift+ prefix
                                mpv_key = format!("Shift+{}", mpv_key);
                            }

                            if modifiers.control_key() {
                                mpv_key = format!("CTRL+{}", mpv_key);
                            }
                            if modifiers.alt_key() {
                                mpv_key = format!("ALT+{}", mpv_key);
                            }

                            // Send keypress to MPV (will trigger input.conf bindings)
                            player.command("keypress".to_string(), vec![mpv_key]);
                        }
                    }
                }

                webview.keyboard_input(key_event, modifiers);
            }
            AppEvent::FileHover((path, state)) => {
                webview.file_hover(path, state);
            }
            AppEvent::FileDrop(state) => {
                webview.file_drop(state);
            }
            AppEvent::FileCancel => {
                webview.file_cancel();
            }
        });

        webview.events(|event| match event {
            WebViewEvent::Ready => {
                webview.navigate(&args.url);
                webview.dev_tools(args.dev);
            }
            WebViewEvent::Loaded => {
                // Proactively send Init message to tell web UI we're a shell (enables MPV)
                let init_message = ipc::create_response(IpcEvent::Init(1));
                webview.post_message(init_message);

                if let Some(deeplink) = &args.open
                    && deeplink.starts_with(URI_SCHEME)
                {
                    let message = ipc::create_response(IpcEvent::OpenMedia(deeplink.to_string()));
                    webview.post_message(message);
                }
            }
            WebViewEvent::Paint => {
                needs_redraw = true;
            }
            WebViewEvent::Resized => {
                webview.update();
                needs_redraw = true;
            }
            WebViewEvent::Cursor(cursor) => {
                app.set_cursor(cursor);
            }
            WebViewEvent::Open(url) => {
                futures::executor::block_on(app.open_url(url));
            }
            WebViewEvent::Ipc(data) => ipc::parse_request(data, |event| match event {
                IpcEvent::Init(id) => {
                    let message = ipc::create_response(IpcEvent::Init(id));
                    webview.post_message(message);
                }
                IpcEvent::Fullscreen(state) => {
                    app.set_fullscreen(state);
                }
                IpcEvent::OpenExternal(url) => {
                    futures::executor::block_on(app.open_url(url));
                }
                IpcEvent::Quit => {
                    event_loop_proxy.send_event(UserEvent::Quit).ok();
                }
                IpcEvent::Mpv(event) => match event {
                    IpcEventMpv::Observe(name) => {
                        player.observe_property(name);
                    }
                    IpcEventMpv::Command((name, args)) => {
                        player.command(name, args);
                    }
                    IpcEventMpv::Set(property) => {
                        player.set_property(property);
                    }
                    _ => {}
                },
                IpcEvent::DiscordPresence(args) => {
                    discord_clone.borrow_mut().update_presence(args);
                }
                IpcEvent::DiscordToggle(enabled) => {
                    discord_clone2.borrow_mut().set_enabled(enabled);
                    // Save to config file
                    let data_dir = dirs::data_dir()
                        .expect("Failed to get data dir")
                        .join(crate::constants::DATA_DIR);
                    let mut discord_config = config::DiscordConfig::load(&data_dir);
                    discord_config.set_enabled(enabled);
                }
                _ => {}
            }),
        });

        player.events(|event| match event {
            PlayerEvent::Start => {
                is_playing = true;
                println!("🎬 [PLAYER] Video started - MPV shortcuts enabled");
                futures::executor::block_on(app.disable_idling());
            }
            PlayerEvent::Stop(error) => {
                is_playing = false;
                println!("⏹️  [PLAYER] Video stopped - MPV shortcuts disabled");
                futures::executor::block_on(app.enable_idling());

                let message = ipc::create_response(IpcEvent::Mpv(IpcEventMpv::Ended(error)));
                webview.post_message(message);
            }
            PlayerEvent::Update => {
                needs_redraw = true;
            }
            PlayerEvent::PropertyChange(property) => {
                let message = ipc::create_response(IpcEvent::Mpv(IpcEventMpv::Change(property)));
                webview.post_message(message);
            }
        });
    }
}
