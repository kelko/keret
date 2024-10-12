use keret_controller_domain::AppMode;
use microbit::{
    display::nonblocking::Display as NonblockDisplay, gpio::DisplayPins, hal::timer::Instance,
};
use tiny_led_matrix::Render;

mod sprites;

use crate::infrastructure::display::sprites::{ERROR_SPRITE, IDLE_SPRITE, RUNNING_SPRITE};
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

impl<T: Instance> keret_controller_appservice::ports::Display for Display<T> {
    /// display a sprite associated with the given `AppMode`
    #[inline]
    fn show_mode(&mut self, app_mode: &AppMode) {
        let sprite = match app_mode {
            AppMode::Idle => IDLE_SPRITE,
            AppMode::Running(_) => RUNNING_SPRITE,
            AppMode::Error => ERROR_SPRITE,
        };
        self.inner.show(&sprite);
    }
}
