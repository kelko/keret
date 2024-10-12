/// measure of how long an action took
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Duration(u64);

impl From<u64> for Duration {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Duration> for u64 {
    #[inline(always)]
    fn from(val: Duration) -> Self {
        val.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SOME_SECONDS: u64 = 0xDA7A_u64;

    #[test]
    fn duration_from_u64_contains_value() {
        // act
        let actual = Duration::from(SOME_SECONDS);

        // assert
        assert_eq!(actual.0, SOME_SECONDS);
    }

    #[test]
    fn u64_into_duration_contains_value() {
        // act
        let actual: Duration = SOME_SECONDS.into();

        // assert
        assert_eq!(actual.0, SOME_SECONDS);
    }

    #[test]
    fn duration_into_u64_returns_value() {
        // arrange
        let duration = Duration(SOME_SECONDS);

        // act
        let actual: u64 = duration.into();

        // assert
        assert_eq!(actual, SOME_SECONDS);
    }
}
