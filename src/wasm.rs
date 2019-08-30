use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Instant(f64);

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("an instant should never be NaN or Inf.")
    }
}
impl Eq for Instant {}

impl Instant {
    #[inline]
    pub fn now() -> Self {
        Instant(now())
    }

    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        assert!(
            earlier.0 <= self.0,
            "`earlier` cannot be later than `self`."
        );
        duration_from_f64(self.0 - earlier.0)
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self {
        Instant(self.0 + duration_to_f64(rhs))
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += duration_to_f64(rhs)
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self {
        Instant(self.0 - duration_to_f64(rhs))
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= duration_to_f64(rhs)
    }
}

fn duration_from_f64(millis: f64) -> Duration {
    Duration::from_millis(millis.trunc() as u64)
        + Duration::from_nanos((millis.fract() * 1.0e6) as u64)
}

fn duration_to_f64(d: Duration) -> f64 {
    d.as_secs() as f64 * 1.0e3 + f64::from(d.subsec_nanos()) * 1.0e-6
}

#[cfg(feature = "stdweb")]
#[allow(unused_results)] // Needed because the js macro triggers it.
pub fn now() -> f64 {
    use stdweb::unstable::TryInto;

    // https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    let v = js! { return performance.now(); };
    v.try_into().unwrap()
}

#[cfg(feature = "wasm-bindgen")]
pub fn now() -> f64 {
    web_sys::window()
        .expect("should have a window in this context")
        .performance()
        .expect("performance should be available")
        .now()
}
