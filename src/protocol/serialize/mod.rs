#[cfg(feature = "simulator-time")]
mod time_milli_seconds;

#[cfg(feature = "simulator-time")]
pub use time_milli_seconds::*;
