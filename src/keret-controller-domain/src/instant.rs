use crate::Duration;
use core::{
    fmt::{Display, Formatter},
    ops::Sub,
};

/// timestamp in controller-local time
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Instant(u64);

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

#[cfg(test)]
mod test {
    use super::*;

    const SOME_TIMESTAMP: u64 = 0xDA7A_u64;
    const DIFFERENCE: u64 = 1;
    const BIGGER_TIMESTAMP: u64 = SOME_TIMESTAMP + DIFFERENCE;

    #[test]
    fn instant_from_u64_contains_value() {
        // act
        let actual = Instant::from(SOME_TIMESTAMP);

        // assert
        assert_eq!(actual.0, SOME_TIMESTAMP);
    }

    #[test]
    fn u64_into_instant_contains_value() {
        // act
        let actual: Instant = SOME_TIMESTAMP.into();

        // assert
        assert_eq!(actual.0, SOME_TIMESTAMP);
    }

    #[test]
    fn instant_into_u64_returns_value() {
        // arrange
        let instant = Instant(SOME_TIMESTAMP);

        // act
        let actual: u64 = (&instant).into();

        // assert
        assert_eq!(actual, SOME_TIMESTAMP);
    }

    #[test]
    fn instant_sub_instant_returns_duration_with_difference() {
        // arrange
        let from = Instant(SOME_TIMESTAMP);
        let to = Instant(BIGGER_TIMESTAMP);

        // act
        let actual = to - from;

        // assert
        assert_eq!(actual, Duration::from(DIFFERENCE));
    }

    #[test]
    #[should_panic]
    fn instant_sub_bigger_instant_panics() {
        // arrange
        let from = Instant(SOME_TIMESTAMP);
        let to = Instant(BIGGER_TIMESTAMP);

        // act
        let _actual = from - to;

        // assert -> checked by `should_panic`
    }
}
