#[cfg(feature = "timetable")]
mod time_hours_minutes;
#[cfg(feature = "simulator-time")]
mod time_milli_seconds;

#[cfg(feature = "timetable")]
pub use time_hours_minutes::*;
#[cfg(feature = "simulator-time")]
pub use time_milli_seconds::*;
