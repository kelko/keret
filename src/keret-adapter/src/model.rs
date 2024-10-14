#[repr(transparent)]
pub(crate) struct TrackResult(u64);

impl From<u64> for TrackResult {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<TrackResult> for u64 {
    fn from(value: TrackResult) -> Self {
        value.0
    }
}
