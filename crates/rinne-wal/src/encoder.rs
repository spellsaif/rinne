use bytes:: {BufMut, BytesMut};

use rinne_core::Event;

use crate::WalError;

pub fn encode(
    event: &Event
) -> Result<Vec<u8>, WalError> {
    let mut buf = BytesMut::new();

    //UUID
    buf.extend_from_slice(
        event.id.as_bytes()
    );

    // timestamp
    buf.put_i64(
        event.timestamp.value()
    );

    // priority
    buf.put_u8(
        event.priority.to_u8()
    );

    // event type
    let event_type = event.event_type.as_str();

    buf.put_u16(
        event_type.len() as u16
    );

    buf.extend_from_slice(
        event_type.as_bytes()
    );

    // Tags count
    buf.put_u16(
      event.tags.len() as u16
    );

    // Tags
    for (key, value) in &event.tags {
        // Key
        buf.put_u16(
            key.len() as u16
        );

        buf.extend_from_slice(
            key.as_bytes()
        );

        // value
        buf.put_u16(
            value.len() as u16
        );

        buf.extend_from_slice(
            key.as_bytes()
        );
    }

    // Payload
    buf.put_u32(
        event.payload.len() as u32
    );

    buf.extend_from_slice(
        &event.payload
    );

    Ok(buf.to_vec())
}
