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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let payload = Payload::new(json!({"foo": "bar"}));

        assert_eq!(payload.data, json!({"foo": "bar"}));
    }

    #[test]
    fn clone() {
        let payload = Payload::new(json!({"foo": "bar"}));
        let clone = payload.clone();

        assert_eq!(clone.data, payload.data);
        assert_eq!(clone.timestamp, payload.timestamp);
    }
}
