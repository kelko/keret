#![cfg_attr(not(test), no_std)]
mod app_mode;
mod duration;
mod error;
mod instant;
mod results;

// re-export everything relevant from the submodules as if it was directly coded here
// hides internal structure of the module
pub use app_mode::AppMode;
pub use duration::Duration;
pub use error::Error;
pub use instant::Instant;
pub use results::{StateUpdateResult, TrackResult};

/// enum to indicate the users desired interaction
/// which is calculated by which button was pressed
#[derive(Debug, Copy, Clone, Default)]
pub enum InteractionRequest {
    #[default]
    None,
    ToggleMode,
    Reset,
}
