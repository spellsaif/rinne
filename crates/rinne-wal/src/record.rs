use bytes::{BufMut, BytesMut};
use rinne_core::Event;

use crate::{WalError, encode};

#[derive(Debug)]
pub struct Record {
    bytes: Vec<u8>,
}

impl Record {
    pub fn from_event(event: &Event) -> Result<Self, WalError> {
        let payload = encode(&event)?;

        let mut buf = BytesMut::new();

        // Length prefix
        buf.put_u32(payload.len() as u32);

        // Payload
        buf.extend_from_slice(&payload);

        Ok(Self {
            bytes: buf.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
