use microbit::{
    display::nonblocking::Display as NonblockDisplay, gpio::DisplayPins, hal::timer::Instance,
};
use tiny_led_matrix::Render;

mod sprites;

use crate::domain::model::AppMode;
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

    /// interrupt-triggered event handling inside the display
    pub(crate) fn handle_display_event(&mut self) {
        self.inner.handle_display_event();
    }

    /// display any kind of sprite
    #[inline]
    pub(crate) fn show_sprite(&mut self, sprite: &impl Render) {
        self.inner.show(sprite);
    }
}

impl<T: Instance> crate::domain::port::Display for Display<T> {
    /// display a sprite associated with the given `AppMode`
    #[inline]
    fn show_mode(&mut self, app_mode: &AppMode) {
        self.inner.show(app_mode);
    }
}
