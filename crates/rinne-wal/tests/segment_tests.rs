use std::fs;

use bytes::Bytes;
use rinne_core::{Event, Priority};
use rinne_wal::Segment;

#[test]
fn should_append_event() {
    let _ = fs::remove_file("test.wal");
    let mut segment = Segment::open("test.wal").unwrap();

    let event = Event::builder()
        .event_type("anime")
        .priority(Priority::High)
        .tag("character", "Kaori Miyazono")
        .payload(Bytes::from_static(b"You will cry."))
        .build()
        .unwrap();

    segment.append(&event).unwrap();
}
