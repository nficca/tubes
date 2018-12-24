extern crate tubes;

use std::thread;
use tubes::{Payload, Tubes};

#[test]
fn it_works() {
    let mut tubes = Tubes::new().add_tube("foo").add_tube("bar");

    let foo_receiver = tubes.subscribe("foo").unwrap();
    let foo_receiver2 = tubes.subscribe("foo").unwrap();
    let bar_receiver = tubes.subscribe("bar").unwrap();

    let payload = Payload::new("thing");

    tubes.send("foo", &payload);

    assert_eq!(bar_receiver.try_recv().is_err(), true);
    assert_eq!(foo_receiver.try_recv().unwrap(), payload);
    assert_eq!(foo_receiver2.try_recv().unwrap(), payload);
}

#[test]
fn it_maintains_payload_ordering() {
    let mut tubes = Tubes::new().add_tube("foo");

    let foo_receiver = tubes.subscribe("foo").unwrap();

    let payload_a = Payload::new(1);
    let payload_b = Payload::new(2);

    tubes.send("foo", &payload_b);
    tubes.send("foo", &payload_a);

    assert_eq!(foo_receiver.try_recv().unwrap(), payload_b);
    assert_eq!(foo_receiver.try_recv().unwrap(), payload_a);
}

#[test]
fn it_works_async() {
    let mut tubes = Tubes::new().add_tube("foo");

    let base_payload = Payload::new(());

    let mut receivers = vec![];
    let mut children = vec![];

    for _ in 0..10 {
        receivers.push(tubes.subscribe("foo").unwrap());
    }

    tubes.send("foo", &base_payload);

    for receiver in receivers {
        let payload = base_payload.clone();

        children.push(thread::spawn(move || {
            assert_eq!(receiver.try_recv().unwrap(), payload);
        }));
    }

    for child in children {
        child.join().unwrap();
    }
}
