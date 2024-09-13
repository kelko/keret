#![no_main]
#![no_std]

mod controls;
mod display;
mod domain;
mod error;
mod mode;
mod serialize;
mod time;

use rtt_target::{rprintln, rtt_init_print};

use crate::{
    controls::init_buttons,
    controls::{get_requested_interaction, InteractionRequest},
    display::display_image,
    display::init_display,
    domain::switch_mode,
    domain::{reset_mode, toggle_mode},
    error::Error,
    mode::AppMode,
    time::init_time,
};
use core::cell::RefCell;
use cortex_m::{interrupt::Mutex, prelude::_embedded_hal_blocking_delay_DelayMs};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{uarte, uarte::Baudrate, uarte::Parity, Timer},
};
use snafu::Error as _;

static CURRENT_MODE: Mutex<RefCell<AppMode>> = Mutex::new(RefCell::new(AppMode::Idle));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let Some(board) = Board::take() else {
        panic!("Could not initialize board. Nothing left to do.");
    };

    let mut timer = Timer::new(board.TIMER0).into_periodic();

    init_display(board.TIMER1, board.display_pins);
    init_time(board.SYST);
    init_buttons(board.GPIOTE, board.buttons);

    let mut serial = uarte::Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    display_image(&AppMode::Idle);

    loop {
        let request = get_requested_interaction();
        match request {
            InteractionRequest::ToggleMode => {
                if let Err(error) = toggle_mode(&mut serial) {
                    handle_error(error)
                }
            }
            InteractionRequest::Reset => reset_mode(),
            InteractionRequest::None => {}
        }

        timer.delay_ms(500_u32);
    }
}

fn handle_error(err: Error) {
    switch_mode(AppMode::Error);
    rprintln!("[ERROR] {}", err);
    let mut source = err.source();

    if source.is_some() {
        rprintln!("[CAUSE]:");
    }
    while let Some(inner) = source {
        rprintln!("{}", inner);

        source = inner.source();
    }
}
