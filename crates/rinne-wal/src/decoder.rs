use std::sync::Arc;

use bytes::Bytes;

use rinne_core::{Event, EventId, EventType, Priority, Timestamp};

use crate::{
    WalError,
    reader::{read_bytes, read_i64, read_u8, read_u16, read_u32},
};

pub fn decode(bytes: &[u8]) -> Result<Event, WalError> {
    let mut buf = Bytes::copy_from_slice(bytes);

    // UUID
    let uuid_bytes = read_bytes(&mut buf, 16)?;

    let id = EventId::from_bytes(uuid_bytes.try_into().expect("uuid must be 16 bytes"));

    // Timestamp
    let timestamp = Timestamp::from_millis(read_i64(&mut buf)?);

    // Priority
    let priority = Priority::from_u8(read_u8(&mut buf)?).ok_or(WalError::InvalidPriority)?;

    // Event Type
    let event_type_len = read_u16(&mut buf)? as usize;

    let event_type_bytes = read_bytes(&mut buf, event_type_len)?;

    let event_type_str =
        std::str::from_utf8(&event_type_bytes).map_err(|_| WalError::InvalidUtf8)?;

    let event_type = EventType::new(event_type_str);

    // Tags
    let tag_count = read_u16(&mut buf)?;

    let mut tags = Vec::new();

    for _ in 0..tag_count {
        // Key
        let key_len = read_u16(&mut buf)? as usize;

        let key_bytes = read_bytes(&mut buf, key_len)?;

        let key = std::str::from_utf8(&key_bytes).map_err(|_| WalError::InvalidUtf8)?;

        // Value
        let value_len = read_u16(&mut buf)? as usize;

        let value_bytes = read_bytes(&mut buf, value_len)?;

        let value = std::str::from_utf8(&value_bytes).map_err(|_| WalError::InvalidUtf8)?;

        tags.push((Arc::from(key), Arc::from(value)));
    }

    // Payload
    let payload_len = read_u32(&mut buf)? as usize;

    let payload_bytes = read_bytes(&mut buf, payload_len)?;

    let payload = bytes::Bytes::from(payload_bytes);

    Ok(Event::new(
        id, timestamp, event_type, priority, tags, payload,
    ))
}
