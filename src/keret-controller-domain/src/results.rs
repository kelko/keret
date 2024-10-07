use crate::{AppMode, Duration};

/// the result of a time tracking action
#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct TrackResult(pub Duration);

// create a time tracking result using the given duration
impl From<Duration> for TrackResult {
    #[inline]
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

// extract the duration as u64
impl From<TrackResult> for u64 {
    #[inline]
    fn from(val: TrackResult) -> Self {
        val.0.into()
    }
}

/// result of calculating the next state
#[derive(Debug, PartialEq)]
pub struct StateUpdateResult {
    /// the mode the controller is next
    pub mode: AppMode,

    /// a result to send (if necessary)
    pub result: Option<TrackResult>,
}

impl StateUpdateResult {
    /// create a new updated state, without a message
    #[inline]
    pub(crate) fn new(mode: AppMode) -> Self {
        Self { mode, result: None }
    }

    /// create a new updated state and also include a message to be sent
    #[inline]
    pub(crate) fn with_result(mode: AppMode, message: TrackResult) -> Self {
        Self {
            mode,
            result: Some(message),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SOME_DURATION: u64 = 0xDA7A_u64;

    #[test]
    fn track_result_from_duration_contains_value() {
        // arrange
        let duration = Duration::from(SOME_DURATION);

        // act
        let actual = TrackResult::from(duration);

        // assert
        assert_eq!(actual.0, Duration::from(SOME_DURATION));
    }

    #[test]
    fn track_result_into_u64_returns_value() {
        // arrange
        let result = TrackResult::from(Duration::from(SOME_DURATION));

        // act
        let actual: u64 = result.into();

        // assert
        assert_eq!(actual, SOME_DURATION);
    }

    #[test]
    fn new_state_update_result_contains_no_result() {
        // arrange
        let mode = AppMode::Idle;

        // act
        let actual = StateUpdateResult::new(mode);

        // assert
        assert_eq!(actual.mode, mode);
        assert_eq!(actual.result, None);
    }

    #[test]
    fn state_update_result_with_message_contains_result() {
        // arrange
        let mode = AppMode::Idle;
        let result = TrackResult::from(Duration::from(SOME_DURATION));

        // act
        let actual = StateUpdateResult::with_result(mode, result);

        // assert
        assert_eq!(actual.mode, mode);
        assert_eq!(
            actual.result,
            Some(TrackResult::from(Duration::from(SOME_DURATION)))
        );
    }
}
