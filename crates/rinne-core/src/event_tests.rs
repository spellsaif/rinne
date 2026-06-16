use bytes::Bytes;

use crate:: {
    Event,
    Priority
};

#[test]
fn should_build_event() {
    let event = Event::builder()
        .event_type("payment")
        .priority(Priority::High)
        .tag("customer", "786")
        .payload(Bytes::from_static(b"Konbanwa"))
        .build()
        .unwrap();

    assert_eq!(event.priority, Priority::High);
}

#[test]
fn should_fail_without_event_type(){
    let result = Event::builder()
        .payload(Bytes::from_static(b"Arigato"))
        .build();

    assert!(result.is_err());
}

#[test]
fn should_default_priority_to_normal() {
    let event = Event::builder()
        .event_type("payment")
        .build()
        .unwrap();

    assert_eq!(event.priority, Priority::Normal);
}
