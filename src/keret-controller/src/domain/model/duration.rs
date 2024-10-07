/// measure of how long an action took
#[repr(transparent)]
pub(crate) struct Duration(u64);

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
