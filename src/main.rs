mod app;
mod constants;
mod instance;
mod ipc;
mod player;
mod server;
mod shared;
mod tray;
mod webview;

use app::{App, AppEvent};
use clap::Parser;
use constants::{DATA_DIR, STARTUP_URL, URI_SCHEME, WINDOW_SIZE};
use glutin::{
    display::GetGlDisplay,
    surface::{GlSurface, SwapInterval},
};
use instance::{Instance, InstanceEvent};
use ipc::{IpcEvent, IpcEventMpv};
use player::{Player, PlayerEvent};
use rust_i18n::i18n;
use server::Server;
use shared::{drop_gl, drop_renderer, with_gl, with_renderer_read, with_renderer_write};
use std::{fs, num::NonZeroU32, process::ExitCode, rc::Rc, time::Duration};
use tracing::warn;
use tray::{Tray, TrayEvent};
use webview::{WebView, WebViewEvent};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
};

i18n!("locales", fallback = "en");

enum UserEvent {
    Quit,
}

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

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let data_path = dirs::data_dir()
        .expect("Failed to get data dir")
        .join(DATA_DIR);
    fs::create_dir_all(&data_path).expect("Failed to create data directory");

    let mut webview = WebView::new(&data_path);
    if webview.should_exit() {
        return ExitCode::SUCCESS;
    }

    let instance = Instance::new();
    if instance.running() {
        if let Some(deeplink) = args.open {
            instance.send(deeplink);
        }

        return ExitCode::SUCCESS;
    }

    instance.start();

    let mut server = Server::new(&data_path);
    if !args.no_server {
        server.setup().expect("Failed to setup server");
        server.start(args.dev).expect("Failed to start server");
    }

    let tray = Tray::new();
    let mut app = App::new();
    let mut player = Player::new();

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
            drop_renderer();
            drop_gl();

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
                if deeplink.starts_with(URI_SCHEME) {
                    let message = ipc::create_response(IpcEvent::OpenMedia(deeplink.to_string()));
                    webview.post_message(message);
                }
            }
        });

        app.events(|event| match event {
            AppEvent::Ready => {
                let refresh_rate = app.get_refresh_rate();

                with_gl(|surface, context| {
                    surface
                        .set_swap_interval(context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                        .map_err(|e| warn!("Failed to enable VSync: {e}"))
                        .ok();

                    shared::create_renderer(WINDOW_SIZE, refresh_rate);
                    player.setup(Rc::new(surface.display()));
                });

                webview.start();
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
                webview.keyboard_input(key_event, modifiers);
            }
        });

        tray.events(|event| match event {
            TrayEvent::Quit => {
                event_loop_proxy.send_event(UserEvent::Quit).ok();
            }
        });

        webview.events(|event| match event {
            WebViewEvent::Ready => {
                webview.navigate(&args.url);
                webview.dev_tools(args.dev);
            }
            WebViewEvent::Loaded => {
                if let Some(deeplink) = &args.open {
                    if deeplink.starts_with(URI_SCHEME) {
                        let message =
                            ipc::create_response(IpcEvent::OpenMedia(deeplink.to_string()));
                        webview.post_message(message);
                    }
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
                _ => {}
            }),
        });

        player.events(|event| match event {
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
