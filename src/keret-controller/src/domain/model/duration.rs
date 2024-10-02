#[repr(transparent)]
pub(crate) struct Duration(u64);

impl From<u64> for Duration {
    #[inline(always)]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<u64> for Duration {
    #[inline(always)]
    fn into(self) -> u64 {
        self.0
    }
}
