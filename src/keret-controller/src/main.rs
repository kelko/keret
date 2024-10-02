#![no_main]
#![no_std]
//⬆️ this code runs directly (bare-metal) on a microcontroller
//   not in a typical OS situation with a kernel
//   so the "main" method is not indicator for code entry, but below you will find #[entry]
//   also as there is no OS the Rust std lib can't be used, as it depends on libc/musl/something similar

mod domain;
mod error;
mod infrastructure;

// importing elements (modules, structs, traits, ...) from other modules to be used in this file

use crate::{
    domain::{AppMode, ApplicationService, Display as _},
    error::{report_error, Error},
    infrastructure::{
        controls::InputControls,
        display::{Display, FATAL_SPRITE},
        serialize::SerialBus,
        time::RunningTimer,
    },
};
use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    prelude::_embedded_hal_blocking_delay_DelayMs,
};
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        timer::{Instance, Periodic},
        Timer,
    },
    pac::{interrupt, Interrupt, NVIC, RTC1, TIMER0, TIMER1, UARTE0},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// the following three variables are static, as they need to be accessed
// by the main running code but also by the interrupts
// as both could happen concurrently they are wrapped in a `Mutex` allowing only one
// concurrent access at a time (using Cortex hardware features)
// and in a `RefCell` so we can call mutable methods on it (so-called "inner mutability")

/// the primary timer used to calculate the running time
static RUNNING_TIMER: Mutex<RefCell<Option<RunningTimer<RTC1>>>> = Mutex::new(RefCell::new(None));

/// the display to show something on the LED matrix
static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

/// wrapper for input controls handling
static CONTROLS: Mutex<RefCell<Option<InputControls>>> = Mutex::new(RefCell::new(None));

/// entry point for the application. Could have any name, `main` used to follow convention from C
/// Initializes the controller as well as go into the execution loop. This method should never return
/// as it drives the whole microcontroller
#[entry]
fn main() -> ! {
    // rtt -> send debug information via USB to attached debugging tool
    rtt_init_print!();

    // get the main "Board" instance from the HAL to access the hardware features
    let Some(board) = Board::take() else {
        panic!("Could not initialize board. Nothing left to do.");
    };

    let mut mode = AppMode::Idle;
    let (mut app_service, mut main_loop_timer) = initialize_app_service(board);

    // main execution loop, should never end
    loop {
        mode = app_service.next_cycle(&mode);
        main_loop_timer.delay_ms(500_u32);
    }
}

/// initialize the board, creating all helper objects and put those necessary in the Mutexes
fn initialize_app_service(
    board: Board,
) -> (
    ApplicationService<RunningTimer<RTC1>, Display<TIMER1>, InputControls, SerialBus<UARTE0>>,
    Timer<TIMER0, Periodic>,
) {
    let mut display = Display::new(board.TIMER1, board.display_pins);
    display.display_image(&AppMode::Idle);

    let controls = InputControls::new(board.GPIOTE, board.buttons);
    let serial_bus = SerialBus::new(board.UARTE0, board.uart);
    let main_loop_timer = Timer::new(board.TIMER0).into_periodic();

    let running_timer = match RunningTimer::new(board.CLOCK, board.RTC1) {
        Ok(timer) => timer,
        Err(e) => handle_init_error(display, e),
    };

    // unmaking interrupts necessary for them to fire
    // needs to be done here to keep Display and RunningTimer flexible
    // as to which RTCs & Timers are actually used
    unsafe {
        NVIC::unmask(Interrupt::TIMER1);
        NVIC::unmask(Interrupt::RTC1);
        NVIC::unmask(Interrupt::GPIOTE);
    }
    NVIC::unpend(Interrupt::GPIOTE);

    free(|cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        *CONTROLS.borrow(cs).borrow_mut() = Some(controls);
        *RUNNING_TIMER.borrow(cs).borrow_mut() = Some(running_timer);
    });

    (
        ApplicationService::new(&RUNNING_TIMER, &DISPLAY, &CONTROLS, serial_bus),
        main_loop_timer,
    )
}

/// report an error that happened during initialization, don't even go into the main loop
fn handle_init_error<T: Instance>(mut display: Display<T>, err: Error) -> ! {
    display.display_image(&FATAL_SPRITE);
    report_error(err);

    loop {
        // don't let user interact if init failed
        continue;
    }
}

// below here are the interrupt handlers
// the name of the method is the interrupt (from an enum) it handles
// in all cases we just forward the call to handle such an interrupt to the
// static object

/// tick the running timer
#[interrupt]
fn RTC1() {
    free(|cs| {
        if let Some(timer) = RUNNING_TIMER.borrow(cs).borrow_mut().as_mut() {
            timer.tick_timer();
        }
    })
}

/// refresh the display
#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    })
}

/// check user inputs
#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(controls) = CONTROLS.borrow(cs).borrow_mut().as_mut() {
            controls.check_input();
        }
    })
}
