pub mod error;
pub mod encoder;
pub mod decoder;

pub use decoder::decode;
pub use encoder::encode;
pub use error::WalError;
