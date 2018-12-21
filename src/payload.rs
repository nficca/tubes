use chrono::prelude::*;

pub struct Payload {
    timestamp: DateTime<Utc>,
    data: serde_json::Value
}

impl Payload {
    pub fn new(data: serde_json::Value) -> Payload {
        Payload {
            timestamp: Utc::now(),
            data
        }
    }
}

impl Clone for Payload {
    fn clone(&self) -> Payload {
        Payload {
            timestamp: self.timestamp,
            data: self.data.clone()
        }
    }
}
