use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_bytes(&self) -> &[u8;16] {
        self.0.as_bytes()
    }

    pub fn from_bytes(
        bytes: [u8;16]
    ) -> Self {
        Self(uuid::Uuid::from_bytes(bytes))
    }
}
