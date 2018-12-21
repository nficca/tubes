extern crate chrono;
#[macro_use] extern crate serde_json;

mod payload;
mod tube;

use std::collections::HashMap;
use std::sync::mpsc;

use self::payload::Payload;
use self::tube::Tube;

struct Tubes {
    tubes: HashMap<String, Tube<Payload>>
}

impl Tubes {
    pub fn new() -> Self {
        let tubes: HashMap<String, Tube<Payload>> = HashMap::new();

        Self { tubes }
    }

    pub fn add_tube<T: Into<String>>(self, name: T) -> Self {
        let mut tubes = self.tubes;

        tubes.insert(name.into(), Tube::Inactive);

        Self { tubes }
    }

    pub fn start(self) -> Self {
        let tubes = self.tubes.into_iter().map(|(name, _)| {
            (name, Tube::Active(vec![]))
        }).collect();

        Self { tubes }
    }

    pub fn send<T: Into<String>>(&self, name: T, payload: Payload) {
        if let Some(Tube::Active(senders)) = self.tubes.get(&name.into()) {
            for sender in senders {
                sender.send(payload.clone()).expect("Failed to send")
            }
        }
    }

    pub fn subscribe<T: Into<String>>(&mut self, name: T) -> Option<mpsc::Receiver<Payload>> {
        if let Some(Tube::Active(senders)) = self.tubes.get_mut(&name.into()) {
            let (sender, receiver) = mpsc::channel();

            senders.push(sender);

            Some(receiver)
        } else {
            None
        }
    }
}
