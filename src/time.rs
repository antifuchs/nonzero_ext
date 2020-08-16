//! Non-zero time intervals.

use crate::{impl_nonzeroable, impl_nonzeroness, NonZero, NonZeroAble};
use std::{ops::Add, time::Duration};

/// A non-zero duration.
///
/// A `NonZeroDuration` represents a [`Duration`] that can not be the
/// "zero" duration, that is, zero nanoseconds long.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonZeroDuration(Duration);

impl NonZeroDuration {
    /// Creates a non-zero duration if the passed value is non-zero.
    pub fn new(d: Duration) -> Option<Self> {
        if d.as_nanos() == 0 {
            return None;
        }
        Some(NonZeroDuration(d))
    }

    pub const unsafe fn new_unchecked(d: Duration) -> Self {
        NonZeroDuration(d)
    }

    /// Returns the wrapped Duration.
    pub const fn get(self) -> Duration {
        self.0
    }
}

impl Add<Duration> for NonZeroDuration {
    type Output = NonZeroDuration;

    fn add(self, rhs: Duration) -> Self::Output {
        let res = self.0 + rhs;

        // Duration can not be negative, so an nz duration + a
        // potentially-zero duration are always a non-zero duration.
        NonZeroDuration(res)
    }
}

impl_nonzeroness!(NonZero, NonZeroDuration, Duration);
impl_nonzeroable!(
    NonZeroAble,
    NonZeroDuration,
    Duration,
    self,
    self.0.as_nanos().count_ones() as usize
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::nonzero;

    #[test]
    fn new_checks_for_0() {
        assert_eq!(NonZeroDuration::new(Duration::from_secs(0)), None);
    }

    #[test]
    fn new_returns_nonzeroes() {
        let d = Duration::from_nanos(823);
        let nzd = NonZeroDuration::new(d).expect("this is definitely a nonzero duration");
        assert_eq!(d, nzd.get());
    }

    #[test]
    fn macroable() {
        let d = Duration::from_nanos(823);
        let nzd = nonzero!(Duration::from_nanos(823));
        assert_eq!(d, nzd.get());
    }
}
