use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalError {
    #[error("unexpected end of input")]
    UnexpectedEof,

    #[error("invalid utf8")]
    InvalidUtf8,

    #[error("invalid priority")]
    InvalidPriority,

}
