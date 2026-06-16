use bytes::Bytes;
use rinne_core::{Event, Priority};
use rinne_wal::{decode, encode};

#[test]
fn should_round_trip_event() {
    let event = Event::builder()
        .event_type("anime")
        .priority(Priority::High)
        .tag("character", "Kaori Miyazono")
        .tag("anime_name", "Your Lie in April")
        .payload(Bytes::from_static(b"You will cry watching this anime."))
        .build()
        .unwrap();

    let encoded = encode(&event).unwrap();
    println!("{encoded:?}");
    println!("encoded len = {}", encoded.len());
    let decoded = decode(&encoded).unwrap();

    assert_eq!(event, decoded);
}
