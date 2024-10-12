#![no_main]
#![no_std]
//⬆️ this code runs directly (bare-metal) on a microcontroller
//   not in a typical OS situation with a kernel
//   so the "main" method is not indicator for code entry, but below you will find #[entry]
//   also as there is no OS the Rust std lib can't be used, as it depends on libc/musl/something similar

mod error;
mod infrastructure;
// importing elements (modules, structs, traits, ...) from other modules to be used in this file

use crate::error::report_domain_error;
use crate::infrastructure::serialize::SerialBusError;
use crate::{
    error::{report_error, InitializationError},
    infrastructure::{
        controls::InputControls,
        display::{Display, FATAL_SPRITE},
        time::RunningTimer,
    },
};
use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    prelude::_embedded_hal_blocking_delay_DelayMs,
};
use cortex_m_rt::entry;
use infrastructure::serialize::SerialBus;
use keret_controller_appservice::{
    ports::Display as _, ApplicationService, Error as AppServiceError,
};
use keret_controller_domain::AppMode;
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

/// convenience type alias to make code shorter/more readable
/// meant for those static values which exist once and used from interrupts and inside domain layer
type Singleton<T> = Mutex<RefCell<Option<T>>>;

/// convenience type alias for the big, complex app service with all types
type AppService<'a> = ApplicationService<
    RunningTimer<RTC1>,
    Display<TIMER1>,
    InputControls,
    SerialBus<UARTE0>,
    fn(&AppServiceError<SerialBusError>),
>;

// the following variable is static, as it needs to be accessed
// by the main running code but also by the interrupts
// as both could happen concurrently it is wrapped in a `Mutex` allowing only one
// concurrent access at a time (using Cortex hardware features)
// and in a `RefCell` so we can call mutable methods on it (so-called "inner mutability")

static APP_SERVICE: Singleton<AppService> = Mutex::new(RefCell::new(None));

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
    let mut main_loop_timer = initialize_board(board);

    // main execution loop, should never end
    loop {
        free(|cs| {
            let mut app_service = APP_SERVICE.borrow(cs).borrow_mut();
            let Some(app_service) = app_service.as_mut() else {
                // if we don't have access on the App Service we don't have access on anything
                // thus no way to display or send messages. Just panic and require hard restart
                // this situation _should_ never arise, unless something is fatally flawed
                panic!("App Service must exist by now. Needs hard restart");
            };
            mode = app_service.next_cycle(&mode);
        });
        main_loop_timer.delay_ms(500_u32);
    }
}

/// initialize the board, creating all helper objects and put the main "app service" in the mutex
/// also initializes the timer used to sleep on the main loop, as the passed in Board object
/// needs to be used in one place only, so everything board "owning" happens here
fn initialize_board(board: Board) -> Timer<TIMER0, Periodic> {
    let mut display = Display::new(board.TIMER1, board.display_pins);
    display.show_mode(&AppMode::Idle);

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
        *APP_SERVICE.borrow(cs).borrow_mut() = Some(ApplicationService::new(
            running_timer,
            display,
            controls,
            serial_bus,
            report_domain_error,
        ));
    });

    main_loop_timer
}

/// report an error that happened during initialization, don't even go into the main loop
fn handle_init_error<T: Instance>(mut display: Display<T>, err: InitializationError) -> ! {
    display.show_sprite(&FATAL_SPRITE);
    report_error(&err);

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
        if let Some(app_service) = APP_SERVICE.borrow(cs).borrow_mut().as_mut() {
            app_service.running_timer.tick_timer();
        }
    })
}

/// refresh the display
#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(app_service) = APP_SERVICE.borrow(cs).borrow_mut().as_mut() {
            app_service.display.handle_display_event();
        }
    })
}

/// check user inputs
#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(app_service) = APP_SERVICE.borrow(cs).borrow_mut().as_mut() {
            app_service.controls.check_input();
        }
    })
}
