use bytes::{Buf, Bytes};

use crate::WalError;

pub fn read_u8(buf: &mut Bytes) -> Result<u8, WalError> {
    if buf.remaining() < 1 {
        return Err(WalError::UnexpectedEof);
    }

    Ok(buf.get_u8())
}

pub fn read_u16(buf: &mut Bytes) -> Result<u16, WalError> {
    if buf.remaining() < 2 {
        return Err(WalError::UnexpectedEof);
    }

    Ok(buf.get_u16())
}

pub fn read_u32(buf: &mut Bytes) -> Result<u32, WalError> {
    if buf.remaining() < 4 {
        return Err(WalError::UnexpectedEof);
    }

    Ok(buf.get_u32())
}

pub fn read_i64(buf: &mut Bytes) -> Result<i64, WalError> {
    if buf.remaining() < 8 {
        return Err(WalError::UnexpectedEof);
    }

    Ok(buf.get_i64())
}

pub fn read_bytes(buf: &mut Bytes, len: usize) -> Result<Vec<u8>, WalError> {
    if buf.remaining() < len {
        return Err(WalError::UnexpectedEof);
    }

    let mut bytes = vec![0u8; len];

    buf.copy_to_slice(&mut bytes);

    Ok(bytes)
}
