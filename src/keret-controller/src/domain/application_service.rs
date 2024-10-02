use crate::{
    domain::{AppMode, InteractionRequest},
    domain::{Display, Instant, RunningTimeClock, SerialBus, UserInterface},
    error::{report_error, Error, NoControlsSnafu},
};
use core::cell::RefCell;
use cortex_m::interrupt::{free, CriticalSection, Mutex};

pub(crate) struct ApplicationService<TClock, TDisplay, TUserInterface, TSerialBus>
where
    TClock: RunningTimeClock + 'static,
    TDisplay: Display + 'static,
    TUserInterface: UserInterface + 'static,
    TSerialBus: SerialBus + 'static,
{
    running_timer: &'static Mutex<RefCell<Option<TClock>>>,
    display: &'static Mutex<RefCell<Option<TDisplay>>>,
    controls: &'static Mutex<RefCell<Option<TUserInterface>>>,
    serial_bus: TSerialBus,
}

impl<TClock, TDisplay, TUserInterface, TSerialBus>
    ApplicationService<TClock, TDisplay, TUserInterface, TSerialBus>
where
    TClock: RunningTimeClock + 'static,
    TDisplay: Display + 'static,
    TUserInterface: UserInterface + 'static,
    TSerialBus: SerialBus + 'static,
{
    pub(crate) fn new(
        running_timer: &'static Mutex<RefCell<Option<TClock>>>,
        display: &'static Mutex<RefCell<Option<TDisplay>>>,
        controls: &'static Mutex<RefCell<Option<TUserInterface>>>,
        serial_bus: TSerialBus,
    ) -> Self {
        Self {
            running_timer,
            display,
            controls,
            serial_bus,
        }
    }

    pub(crate) fn next_cycle(&mut self, mode: &AppMode) -> AppMode {
        let next = self
            .calculate_next_state(&mode)
            .unwrap_or_else(handle_runtime_error);
        self.show_mode(&next);

        next
    }

    /// calculate the next state in the next processing cycle:
    /// check what the user requested to do (by clicking on buttons) and
    /// let domain layer calculate the next state based on this input
    fn calculate_next_state(&mut self, mode: &AppMode) -> Result<AppMode, Error> {
        let (request, time) = free(|cs| (self.get_requested_interaction(cs), self.now(cs)));
        mode.handle_interaction_request(request?, time, &mut self.serial_bus)
    }

    /// convenience method to read the current "running time" from the static timer object
    fn now(&self, cs: &CriticalSection) -> Option<Instant> {
        self.running_timer
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .map(|timer| timer.now())
    }

    /// convenience wrapper to read the user interaction from the static controls object
    fn get_requested_interaction(&self, cs: &CriticalSection) -> Result<InteractionRequest, Error> {
        if let Some(controls) = self.controls.borrow(cs).borrow_mut().as_mut() {
            Ok(controls.get_requested_interaction())
        } else {
            NoControlsSnafu.fail()
        }
    }

    /// convenience method to show the correct sprite for current mode on the display
    fn show_mode(&self, mode: &AppMode) {
        free(|cs| {
            let mut display = self.display.borrow(cs).borrow_mut();
            let display = display
                .as_mut()
                .expect("Display must be set at this point. Need restart");

            display.display_image(mode);
        });
    }
}

/// report an error that happened while executing the main loop
/// and switch the AppMode appropriately to indicate it's in a failure state
fn handle_runtime_error(err: Error) -> AppMode {
    report_error(err);
    AppMode::Error
}
