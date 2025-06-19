use std::{
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use crossbeam_channel::{Receiver, Sender, unbounded};

use crate::config::InstanceConfig;

pub enum InstanceEvent {
    Open(String),
}

pub struct Instance {
    sender: Sender<InstanceEvent>,
    receiver: Receiver<InstanceEvent>,
    socket: Option<UnixStream>,
    config: InstanceConfig,
}

impl Instance {
    pub fn new(config: InstanceConfig) -> Self {
        let (sender, receiver) = unbounded::<InstanceEvent>();
        let socket = UnixStream::connect(&config.socket_file).ok();

        Self {
            sender,
            receiver,
            socket,
            config,
        }
    }

    pub fn running(&self) -> bool {
        self.socket.is_some()
    }

    pub fn send(&self, data: String) {
        if let Some(mut stream) = self.socket.as_ref() {
            stream
                .write_all(data.as_bytes())
                .expect("Failed to write to stream");
        }
    }

    pub fn start(&self) {
        self.config.remove_socket_file();

        let listener =
            UnixListener::bind(&self.config.socket_file).expect("Failed to create socket");

        let sender = self.sender.clone();
        thread::spawn(move || {
            for mut stream in listener.incoming().flatten() {
                let mut buffer = String::new();
                if stream.read_to_string(&mut buffer).is_ok() {
                    sender.send(InstanceEvent::Open(buffer)).ok();
                }
            }
        });
    }

    pub fn stop(&self) {
        self.config.remove_socket_file();
    }

    pub fn events<F: FnMut(InstanceEvent)>(&self, handler: F) {
        self.receiver.try_iter().for_each(handler);
    }
}
