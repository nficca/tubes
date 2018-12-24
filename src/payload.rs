use chrono::prelude::*;

#[derive(PartialEq, Debug)]
pub struct Payload<T: Clone> {
    timestamp: DateTime<Utc>,
    data: T,
}

impl<T: Clone> Payload<T> {
    pub fn new(data: T) -> Payload<T> {
        Payload {
            timestamp: Utc::now(),
            data,
        }
    }
}

impl<T: Clone> Clone for Payload<T> {
    fn clone(&self) -> Payload<T> {
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
        let payload = Payload::new(1);

        assert_eq!(payload.data, 1);
    }

    #[test]
    fn clone() {
        let payload = Payload::new(5);
        let clone = payload.clone();

        assert_eq!(clone.data, payload.data);
        assert_eq!(clone.timestamp, payload.timestamp);
    }

    #[test]
    fn eq() {
        let a = Payload::new(1);
        let b = a.clone();
        let c = Payload::new(3);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);
    }
}
