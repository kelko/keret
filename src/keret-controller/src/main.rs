#![no_main]
#![no_std]
//⬆️ this code runs directly (bare-metal) on a microcontroller
//   not in a typical OS situation with a kernel
//   so the "main" method is not indicator for code entry, but below you will find #[entry]
//   also as there is no OS the Rust std lib can't be used, as it depends on libc/musl/something similar

// the "modules" of this app (think "package"/"namespace") in other languages
mod controls;
mod display;
mod domain;
mod error;
mod render;
mod serialize;
mod time;

// importing elements (modules, structs, traits, ...) from other modules to be used in this file
use crate::{
    controls::{InputControls, InteractionRequest},
    display::Display,
    domain::AppMode,
    error::report_error,
    error::{Error, NoControlsSnafu},
    render::FATAL_SPRITE,
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
    let (mut serial_bus, mut main_loop_timer) = initialize_board(board);

    // main execution loop, should never end
    loop {
        mode = next_cycle(&mode, &mut serial_bus).unwrap_or_else(handle_runtime_error);
        show_mode(&mode);

        main_loop_timer.delay_ms(500_u32);
    }
}

/// initialize the board, creating all helper objects and put those necessary in the Mutexes
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

    (serial_bus, main_loop_timer)
}

/// calculate the next state in the next processing cycle:
/// check what the user requested to do (by clicking on buttons) and
/// let domain layer calculate the next state based on this input
fn next_cycle<T: UarteInstance>(
    mode: &AppMode,
    serial_bus: &mut SerialBus<T>,
) -> Result<AppMode, Error> {
    let request = get_requested_interaction()?;
    mode.handle_interaction_request(request, now(), serial_bus)
}

/// convenience method to read the current "running time" from the static timer object
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

/// convenience wrapper to read the user interaction from the static controls object
fn get_requested_interaction() -> Result<InteractionRequest, Error> {
    free(|cs| {
        if let Some(controls) = CONTROLS.borrow(cs).borrow_mut().as_mut() {
            Ok(controls.get_requested_interaction())
        } else {
            NoControlsSnafu.fail()
        }
    })
}

/// convenience method to show the correct sprite for current mode on the display
fn show_mode(mode: &AppMode) {
    free(|cs| {
        let mut display = DISPLAY.borrow(cs).borrow_mut();
        let display = display
            .as_mut()
            .expect("Display must be set at this point. Need restart");

        display.display_image(mode);
    });
}

/// report an error that happened while executing the main loop
/// and switch the AppMode appropriately to indicate it's in a failure state
fn handle_runtime_error(err: Error) -> AppMode {
    report_error(err);
    AppMode::Error
}

/// report an error that happened during initialization, don't even go into the main loop
fn handle_init_error<T: TimerInstance>(mut display: Display<T>, err: Error) -> ! {
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
