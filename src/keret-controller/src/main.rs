#![no_main]
#![no_std]

mod controls;
mod display;
mod domain;
mod error;
mod render;
mod serialize;
mod time;

use rtt_target::rtt_init_print;

use crate::error::report_error;
use crate::render::FATAL_SPRITE;
use crate::{
    controls::{InputControls, InteractionRequest},
    display::Display,
    domain::AppMode,
    error::{Error, NoControlsSnafu},
    serialize::SerialBus,
    time::RunningTimer,
};
use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    prelude::_embedded_hal_blocking_delay_DelayMs,
};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::timer::{Instance as TimerInstance, Periodic},
    hal::uarte::Instance as UarteInstance,
    hal::Timer,
    pac::{interrupt, RTC1, TIMER0, TIMER1, UARTE0},
};
use panic_rtt_target as _;

static RUNNING_TIMER: Mutex<RefCell<Option<RunningTimer<RTC1>>>> = Mutex::new(RefCell::new(None));
static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));
static CONTROLS: Mutex<RefCell<Option<InputControls>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let Some(board) = Board::take() else {
        panic!("Could not initialize board. Nothing left to do.");
    };

    let mut mode = AppMode::Idle;
    let (mut serial_bus, mut main_loop_timer) = initialize_board(board);

    loop {
        mode = next_cycle(&mode, &mut serial_bus).unwrap_or_else(handle_runtime_error);
        show_mode(&mode);

        main_loop_timer.delay_ms(500_u32);
    }
}

fn show_mode(mode: &AppMode) {
    free(|cs| {
        let mut display = DISPLAY.borrow(cs).borrow_mut();
        let display = display
            .as_mut()
            .expect("Display must be set at this point. Need restart");

        display.display_image(mode);
    });
}

fn initialize_board(board: Board) -> (SerialBus<UARTE0>, Timer<TIMER0, Periodic>) {
    let mut display = Display::new(board.TIMER1, board.display_pins);
    display.display_image(&AppMode::Idle);

    let controls = InputControls::new(board.GPIOTE, board.buttons);
    let serial_bus = SerialBus::new(board.UARTE0, board.uart);
    let main_loop_timer = Timer::new(board.TIMER0).into_periodic();

    let running_timer = match RunningTimer::new(board.CLOCK, board.RTC1) {
        Ok(timer) => timer,
        Err(e) => handle_init_error(display, e),
    };

    free(|cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        *CONTROLS.borrow(cs).borrow_mut() = Some(controls);
        *RUNNING_TIMER.borrow(cs).borrow_mut() = Some(running_timer);
    });

    (serial_bus, main_loop_timer)
}

fn next_cycle<T: UarteInstance>(
    mode: &AppMode,
    serial_bus: &mut SerialBus<T>,
) -> Result<AppMode, Error> {
    let request = get_requested_interaction()?;
    mode.handle_interaction_request(request, now(), serial_bus)
}

fn get_requested_interaction() -> Result<InteractionRequest, Error> {
    free(|cs| {
        if let Some(controls) = CONTROLS.borrow(cs).borrow_mut().as_mut() {
            Ok(controls.get_requested_interaction())
        } else {
            NoControlsSnafu.fail()
        }
    })
}

#[inline(always)]
fn now() -> Option<u64> {
    free(|cs| {
        RUNNING_TIMER
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .map(|timer| timer.now())
    })
}

fn handle_runtime_error(err: Error) -> AppMode {
    report_error(err);
    AppMode::Error
}

fn handle_init_error<T: TimerInstance>(mut display: Display<T>, err: Error) -> ! {
    display.display_image(&FATAL_SPRITE);
    report_error(err);

    loop {
        // don't let user interact if init failed
        continue;
    }
}

#[interrupt]
fn RTC1() {
    free(|cs| {
        if let Some(timer) = RUNNING_TIMER.borrow(cs).borrow_mut().as_mut() {
            timer.tick_timer();
        }
    })
}

#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}

#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(controls) = CONTROLS.borrow(cs).borrow_mut().as_mut() {
            controls.check_input();
        }
    })
}
