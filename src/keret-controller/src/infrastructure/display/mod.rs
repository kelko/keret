use microbit::{
    display::nonblocking::Display as NonblockDisplay, gpio::DisplayPins, hal::timer::Instance,
};
use tiny_led_matrix::Render;

mod sprites;

pub(crate) use sprites::FATAL_SPRITE;

/// convenience abstraction of the BSP display module
#[repr(transparent)]
pub(crate) struct Display<T: Instance> {
    inner: NonblockDisplay<T>,
}

impl<T: Instance> Display<T> {
    /// create a new instance, configuring the underlying Display element and unlocking the timer
    pub(crate) fn new(board_timer: T, board_display: DisplayPins) -> Self {
        let display = NonblockDisplay::new(board_timer, board_display);

        Self { inner: display }
    }

    pub(crate) fn handle_display_event(&mut self) {
        self.inner.handle_display_event();
    }
}

impl<T: Instance> crate::domain::Display for Display<T> {
    fn display_image(&mut self, image: &impl Render) {
        self.inner.show(image);
    }
}
