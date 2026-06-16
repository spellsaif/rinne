use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventError {
    #[error("event type cannot be empty")]
    EmptyEventType,

    #[error("payload cannot be empty")]
    EmptyPayload,

    #[error("event type is required")]
    MissingEventType,

}
