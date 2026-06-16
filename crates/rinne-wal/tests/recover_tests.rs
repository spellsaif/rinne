use std::fs;

use bytes::Bytes;

use rinne_core::{Event, Priority};

use rinne_wal::Segment;

#[test]
fn should_recover_events() {
    let _ = fs::remove_file("recover.wal");

    let mut segment = Segment::open("recover.wal").unwrap();

    let event1 = Event::builder()
        .event_type("anime")
        .priority(Priority::High)
        .payload(Bytes::from_static(b"Kaori"))
        .build()
        .unwrap();

    let event2 = Event::builder()
        .event_type("anime")
        .priority(Priority::Normal)
        .payload(Bytes::from_static(b"Violet"))
        .build()
        .unwrap();

    segment.append(&event1).unwrap();

    segment.append(&event2).unwrap();

    let recovered = segment.recover().unwrap();

    assert_eq!(recovered, vec![event1, event2]);
}
