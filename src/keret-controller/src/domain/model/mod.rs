mod app_mode;
mod duration;
mod instant;

// re-export everything relevant from the submodules as if it was directly coded here
// hides internal structure of the module
pub(crate) use app_mode::AppMode;
pub(crate) use duration::Duration;
pub(crate) use instant::Instant;

/// enum to indicate the users desired interaction
/// which is calculated by which button was pressed
#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum InteractionRequest {
    #[default]
    None,
    ToggleMode,
    Reset,
}

/// result of calculating the next state
pub(crate) struct StateUpdateResult {
    /// the mode the controller is next
    pub(crate) mode: AppMode,

    /// a message to send (if necessary)
    pub(crate) message: Option<TrackResult>,
}

impl StateUpdateResult {
    /// create a new updated state, without a message
    #[inline]
    fn new(mode: AppMode) -> Self {
        Self {
            mode,
            message: None,
        }
    }

    /// create a new updated state and also include a message to be sent
    #[inline]
    fn with_message(mode: AppMode, message: TrackResult) -> Self {
        Self {
            mode,
            message: Some(message),
        }
    }
}

/// the result of a time tracking action
#[repr(transparent)]
pub(crate) struct TrackResult(pub Duration);

// create a time tracking result using the given duration
impl From<Duration> for TrackResult {
    #[inline]
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

// extract the duration as u64
impl Into<u64> for TrackResult {
    #[inline]
    fn into(self) -> u64 {
        self.0.into()
    }
}
