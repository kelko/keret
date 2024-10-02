use crate::{
    domain::{Duration, Instant},
    error::Error,
};
use tiny_led_matrix::Render;

/// enum to indicate the users desired interaction
/// which is calculated by which button was pressed
#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum InteractionRequest {
    #[default]
    None,
    ToggleMode,
    Reset,
}

pub(crate) trait RunningTimeClock {
    fn now(&mut self) -> Instant;
}

pub(crate) trait SerialBus {
    fn serialize_message(&mut self, duration: Duration) -> Result<(), Error>;
}

pub(crate) trait Display {
    fn display_image(&mut self, image: &impl Render);
}

pub(crate) trait UserInterface {
    fn get_requested_interaction(&mut self) -> InteractionRequest;
}
