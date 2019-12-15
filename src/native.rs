pub type Instant = std::time::Instant;
pub type SystemTime = std::time::SystemTime;

/// The current time, in milliseconds.
#[cfg(feature = "now")]
pub fn now() -> f64 {
    time::precise_time_s() * 1000.0
}
