mod browser_process_handler;
mod client;
mod render_process_handler;
mod utils;
mod v8_handler;

use browser_process_handler::WebViewBrowserProcessHandler;
use render_process_handler::WebViewRenderProcessHandler;

use crate::{cef_impl, constants::CMD_SWITCHES};

cef_impl!(
    prefix = "WebView",
    name = App,
    sys_type = cef_dll_sys::cef_app_t,
    {
        fn on_before_command_line_processing(
            &self,
            _process_type: Option<&CefString>,
            command_line: Option<&mut CommandLine>,
        ) {
            if let Some(line) = command_line {
                CMD_SWITCHES.iter().for_each(|switch| {
                    line.append_switch(Some(&CefString::from(switch.to_owned())));
                });
            }
        }

        fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
            Some(WebViewBrowserProcessHandler::new())
        }

        fn render_process_handler(&self) -> Option<RenderProcessHandler> {
            Some(WebViewRenderProcessHandler::new())
        }
    }
);
