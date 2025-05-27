pub const PRELOAD_SCRIPT: &str = include_str!("preload.js");
pub const IPC_SENDER: &str = "__postMessage";
pub const IPC_RECEIVER: &str = "__onMessage";

// Process messages
pub const IPC_MESSAGE: &str = "IPC";
pub const READY_MESSAGE: &str = "READY";
