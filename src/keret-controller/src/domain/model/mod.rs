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

pub(crate) struct StateUpdateResult {
    pub(crate) mode: AppMode,
    pub(crate) message: Option<ResultMessage>,
}

impl StateUpdateResult {
    #[inline]
    fn new(mode: AppMode) -> Self {
        Self {
            mode,
            message: None,
        }
    }

    #[inline]
    fn with_message(mode: AppMode, message: ResultMessage) -> Self {
        Self {
            mode,
            message: Some(message),
        }
    }
}

#[repr(transparent)]
pub(crate) struct ResultMessage(pub Duration);

impl From<Duration> for ResultMessage {
    #[inline]
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl Into<u64> for ResultMessage {
    #[inline]
    fn into(self) -> u64 {
        self.0.into()
    }
}
