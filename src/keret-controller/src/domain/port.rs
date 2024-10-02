use crate::{
    domain::model::{Instant, InteractionRequest, ResultMessage},
    error::Error,
};
use tiny_led_matrix::Render;

pub(crate) trait RunningTimeClock {
    fn now(&mut self) -> Instant;
}

pub(crate) trait SerialBus {
    fn serialize_message(&mut self, message: ResultMessage) -> Result<(), Error>;
}

pub(crate) trait Display {
    fn display_image(&mut self, image: &impl Render);
}

pub(crate) trait UserInterface {
    fn get_requested_interaction(&mut self) -> InteractionRequest;
}
