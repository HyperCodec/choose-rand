pub use crate::error::*;

/// The Result type for this crate that can contain the `Error` variant.
pub type Result<T> = std::result::Result<T, Error>;

pub use crate::rand::*;
