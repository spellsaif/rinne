use bytes::Bytes;
use std::sync::Arc;

use crate::EventBuilder;

use crate::{
  EventId,
  EventType,
  Priority,
  Timestamp
};

#[derive(Clone, Debug)]
pub struct Event {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub event_type: EventType,
    pub priority: Priority,
    pub tags: Vec<(Arc<str>, Arc<str>)>,
    pub payload: Bytes,
}

impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder::default()
    }
}
