pub use crate::error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;