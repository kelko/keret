use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::pac::interrupt;
use microbit::{board::Buttons, hal::gpiote::Gpiote, pac};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static INTERACTION_REQUEST: Mutex<RefCell<InteractionRequest>> =
    Mutex::new(RefCell::new(InteractionRequest::None));

#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum InteractionRequest {
    #[default]
    None,
    ToggleMode,
    Reset,
}

/// Initialise the buttons and enable interrupts.
pub(crate) fn init_buttons(board_gpiote: pac::GPIOTE, board_buttons: Buttons) {
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

    free(move |cs| {
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    });
}

#[inline(always)]
pub(crate) fn get_requested_interaction() -> InteractionRequest {
    free(|cs| {
        let next = *INTERACTION_REQUEST.borrow(cs).borrow();
        *INTERACTION_REQUEST.borrow(cs).borrow_mut() = InteractionRequest::None;

        next
    })
}

#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let a_pressed = gpiote.channel0().is_event_triggered();
            let b_pressed = gpiote.channel1().is_event_triggered();

            let request = if a_pressed {
                InteractionRequest::Reset
            } else if b_pressed {
                InteractionRequest::ToggleMode
            } else {
                InteractionRequest::None
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();

            INTERACTION_REQUEST.borrow(cs).replace(request);
        }
    });
}
