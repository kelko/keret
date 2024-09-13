#![no_main]
#![no_std]

mod display;
mod mode;
mod time;
mod controls;
mod serialize;
mod domain;
mod error;

use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use core::cell::RefCell;
use cortex_m::{
    interrupt::{Mutex},
    prelude::_embedded_hal_blocking_delay_DelayMs
};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{Timer,
    uarte, uarte::Parity, uarte::Baudrate}
};
use snafu::Error as _;
use crate::{
    domain::switch_mode,
    display::init_display,
    controls::init_buttons,
    time::init_time,
    mode::AppMode,
    controls::{get_requested_interaction, InteractionRequest},
    display::display_image,
    domain::{reset_mode, toggle_mode},
    error::Error
};

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
            },
            InteractionRequest::Reset => reset_mode(),
            InteractionRequest::None => {},
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
