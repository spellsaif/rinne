use bytes::Bytes;
use std::sync::Arc;

use crate::EventBuilder;

use crate::{
  EventId,
  EventType,
  Priority,
  Timestamp
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub timestamp: Timestamp,
    pub event_type: EventType,
    pub priority: Priority,
    pub tags: Vec<(Arc<str>, Arc<str>)>,
    pub payload: Bytes,
}

impl Event {

    pub fn new(
        id: EventId,
        timestamp: Timestamp,
        event_type: EventType,
        priority: Priority,
        tags: Vec<(Arc<str>, Arc<str>)>,
        payload: Bytes
    ) -> Self {
        Self {
            id,
            timestamp,
            event_type,
            priority,
            tags,
            payload,
        }
    }

    pub fn builder() -> EventBuilder {
        EventBuilder::default()
    }
}
