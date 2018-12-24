mod payload;

use std::collections::HashMap;
use std::sync::mpsc;

pub use self::payload::Payload;

#[derive(Default)]
pub struct Tubes<T: Clone> {
    tubes: HashMap<String, Vec<mpsc::Sender<Payload<T>>>>,
}

impl<T: Clone> Tubes<T> {
    pub fn new() -> Self {
        let tubes = HashMap::new();

        Self { tubes }
    }

    pub fn add_tube<S: Into<String>>(self, name: S) -> Self {
        let mut tubes = self.tubes;

        tubes.insert(name.into(), vec![]);

        Self { tubes }
    }

    pub fn send<S: Into<String>>(&self, name: S, payload: &Payload<T>) {
        if let Some(senders) = self.tubes.get(&name.into()) {
            for sender in senders {
                sender.send(payload.clone()).expect("Failed to send")
            }
        }
    }

    pub fn subscribe<S: Into<String>>(&mut self, name: S) -> Option<mpsc::Receiver<Payload<T>>> {
        if let Some(senders) = self.tubes.get_mut(&name.into()) {
            let (sender, receiver) = mpsc::channel();

            senders.push(sender);

            Some(receiver)
        } else {
            None
        }
    }
}
