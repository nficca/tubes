mod payload;

use std::collections::HashMap;
use std::sync::mpsc;

pub use self::payload::Payload;

pub struct Tubes {
    tubes: HashMap<String, Vec<mpsc::Sender<Payload>>>,
}

impl Tubes {
    pub fn new() -> Self {
        let tubes: HashMap<String, Vec<mpsc::Sender<Payload>>> = HashMap::new();

        Self { tubes }
    }

    pub fn add_tube<T: Into<String>>(self, name: T) -> Self {
        let mut tubes = self.tubes;

        tubes.insert(name.into(), vec![]);

        Self { tubes }
    }

    pub fn send<T: Into<String>>(&self, name: T, payload: &Payload) {
        if let Some(senders) = self.tubes.get(&name.into()) {
            for sender in senders {
                sender.send(payload.clone()).expect("Failed to send")
            }
        }
    }

    pub fn subscribe<T: Into<String>>(&mut self, name: T) -> Option<mpsc::Receiver<Payload>> {
        if let Some(senders) = self.tubes.get_mut(&name.into()) {
            let (sender, receiver) = mpsc::channel();

            senders.push(sender);

            Some(receiver)
        } else {
            None
        }
    }
}
