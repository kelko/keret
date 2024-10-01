use microbit::{board::Buttons, hal::gpiote::Gpiote, pac};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum InteractionRequest {
    #[default]
    None,
    ToggleMode,
    Reset,
}

pub(crate) struct InputControls {
    gpiote: Gpiote,
    request: InteractionRequest,
}

impl InputControls {
    pub(crate) fn new(board_gpiote: pac::GPIOTE, board_buttons: Buttons) -> Self {
        let gpiote = Gpiote::new(board_gpiote);

        let channel0 = gpiote.channel0();
        channel0
            .input_pin(&board_buttons.button_a.degrade())
            .hi_to_lo()
            .enable_interrupt();
        channel0.reset_events();

        let channel1 = gpiote.channel1();
        channel1
            .input_pin(&board_buttons.button_b.degrade())
            .hi_to_lo()
            .enable_interrupt();
        channel1.reset_events();

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);

        Self {
            gpiote,
            request: InteractionRequest::None,
        }
    }

    #[inline(always)]
    pub(crate) fn get_requested_interaction(&mut self) -> InteractionRequest {
        let current = self.request;
        self.request = InteractionRequest::None;

        current
    }

    pub(crate) fn check_input(&mut self) {
        let a_pressed = self.gpiote.channel0().is_event_triggered();
        let b_pressed = self.gpiote.channel1().is_event_triggered();

        let request = if a_pressed {
            InteractionRequest::Reset
        } else if b_pressed {
            InteractionRequest::ToggleMode
        } else {
            InteractionRequest::None
        };

        self.gpiote.channel0().reset_events();
        self.gpiote.channel1().reset_events();

        self.request = request;
    }
}
