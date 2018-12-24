use chrono::prelude::*;
use serde_json;

#[derive(PartialEq, Debug)]
pub struct Payload {
    timestamp: DateTime<Utc>,
    data: serde_json::Value,
}

impl Payload {
    pub fn new(data: serde_json::Value) -> Payload {
        Payload {
            timestamp: Utc::now(),
            data,
        }
    }
}

impl Clone for Payload {
    fn clone(&self) -> Payload {
        Payload {
            timestamp: self.timestamp,
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let payload = Payload::new(serde_json::json!({"foo": "bar"}));

        assert_eq!(payload.data, serde_json::json!({"foo": "bar"}));
    }

    #[test]
    fn clone() {
        let payload = Payload::new(serde_json::json!({"foo": "bar"}));
        let clone = payload.clone();

        assert_eq!(clone.data, payload.data);
        assert_eq!(clone.timestamp, payload.timestamp);
    }

    #[test]
    fn eq() {
        let a = Payload::new(serde_json::json!({"foo": "bar"}));
        let b = a.clone();
        let c = Payload::new(serde_json::json!({"fizz": "buzz"}));

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
}
