use crate::domain::model::Duration;
use core::{
    fmt::{Display, Formatter},
    ops::Sub,
};

/// timestamp in controller-local time
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(crate) struct Instant(u64);

// display the instant
impl Display for Instant {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

// create the instant from a u64 timer value
impl From<u64> for Instant {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

// extract the u64 timestamp from the instant
impl From<&Instant> for u64 {
    #[inline(always)]
    fn from(val: &Instant) -> Self {
        val.0
    }
}

// calculate different between 2 instances, creating a `Duration`
impl Sub for Instant {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}
