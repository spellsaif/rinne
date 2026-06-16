use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventType(Arc<str>);

impl EventType {
    pub fn new(value: impl Into<Arc<str>>) -> Self {
        Self(value.into())
    }
}
