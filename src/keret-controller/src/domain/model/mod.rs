mod app_mode;
mod duration;
mod instant;

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
