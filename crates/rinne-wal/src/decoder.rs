use bytes::{Buf, Bytes};
use std::sync::Arc;

use rinne_core::{Event, EventId, EventType, Priority, Timestamp};

use crate::WalError;

pub fn decode(bytes: &[u8]) -> Result<Event, WalError> {
    let mut buf = Bytes::copy_from_slice(bytes);

    // UUID
    let mut uuid_bytes = [0u8; 16];

    buf.copy_to_slice(&mut uuid_bytes);

    let id = EventId::from_bytes(uuid_bytes);

    // Timestamp
    let timestamp = Timestamp::from_millis(buf.get_i64());

    // Priority
    let priority = Priority::from_u8(buf.get_u8()).ok_or(WalError::InvalidPriority)?;

    // Event Type
    let event_type_len = buf.get_u16() as usize;

    let mut event_type_bytes = vec![0u8; event_type_len];

    buf.copy_to_slice(&mut event_type_bytes);

    let event_type = std::str::from_utf8(&event_type_bytes).map_err(|_| WalError::InvalidUtf8)?;

    let event_type = EventType::new(event_type);

    // Tag count
    let tag_count = buf.get_u16();

    let mut tags = Vec::new();

    // Tags
    for _ in 0..tag_count {
        // Key
        let key_len = buf.get_u16() as usize;

        let mut key_bytes = vec![0u8; key_len];

        buf.copy_to_slice(&mut key_bytes);

        let key = std::str::from_utf8(&key_bytes).map_err(|_| WalError::InvalidUtf8)?;

        // Value
        let value_len = buf.get_u16() as usize;

        let mut value_bytes = vec![0u8; value_len];

        buf.copy_to_slice(&mut value_bytes);

        let value = std::str::from_utf8(&value_bytes).map_err(|_| WalError::InvalidUtf8)?;

        tags.push((Arc::from(key), Arc::from(value)));
    }

    // Payload
    let payload_len = buf.get_u32() as usize;

    let mut payload_bytes = vec![0u8; payload_len];

    buf.copy_to_slice(&mut payload_bytes);

    let payload = bytes::Bytes::from(payload_bytes);

    Ok(Event::new(
        id, timestamp, event_type, priority, tags, payload,
    ))
}
