use crate::domain::model::Duration;
use core::fmt::{Display, Formatter};
use core::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(crate) struct Instant(u64);

impl Display for Instant {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for Instant {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<u64> for &Instant {
    #[inline(always)]
    fn into(self) -> u64 {
        self.0
    }
}

impl Sub for Instant {
    type Output = Duration;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}
