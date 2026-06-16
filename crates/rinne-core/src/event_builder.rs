use bytes::Bytes;
use std::sync::Arc;

use crate:: {
    Event,
    EventError,
    EventId,
    EventType,
    Priority,
    Timestamp
};

#[derive(Default)]
pub struct EventBuilder {
    event_type: Option<EventType>,
    priority: Option<Priority>,
    tags: Vec<(Arc<str>, Arc<str>)>,
    payload: Option<Bytes>
}

impl EventBuilder {
    pub fn event_type(
        mut self,
        event_type: impl Into<Arc<str>>
    ) -> Self {
        self.event_type = Some(EventType::new(event_type));

        self
    }

    pub fn priority(
        mut self,
        priority: Priority
    ) -> Self {
        self.priority = Some(priority);

        self
    }

    pub fn tag(
        mut self,
        key: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>
    ) -> Self {
        self.tags.push((key.into(), value.into()));

        self
    }

    pub fn payload(
        mut self,
        payload: Bytes
    ) -> Self {
        self.payload = Some(payload);

        self
    }

    pub fn build(self) -> Result<Event, EventError> {
        let event_type = self.event_type.ok_or(EventError::MissingEventType)?;

        Ok(Event {
            id: EventId::new(),
            timestamp: Timestamp::now(),
            event_type,
            priority: self.priority.unwrap_or(Priority::Normal),
            tags: self.tags,
            payload: self.payload.unwrap_or_default(),
        })
    }


}
