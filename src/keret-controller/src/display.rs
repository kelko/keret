use microbit::{
    display::nonblocking::Display as NonblockDisplay, gpio::DisplayPins, hal::timer::Instance, pac,
};
use tiny_led_matrix::Render;

#[repr(transparent)]
pub(crate) struct Display<T: Instance> {
    inner: NonblockDisplay<T>,
}

impl<T: Instance> Display<T> {
    pub(crate) fn new(board_timer: T, board_display: DisplayPins) -> Self {
        let display = NonblockDisplay::new(board_timer, board_display);

        unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER1) }

        Self { inner: display }
    }

    pub(crate) fn display_image(&mut self, image: &impl Render) {
        self.inner.show(image);
    }

    pub(crate) fn handle_display_event(&mut self) {
        self.inner.handle_display_event();
    }
}
