use crate::{
    error::{DomainErrorOccurredSnafu, SendingMessageToOutsideFailedSnafu},
    ports::{Display, OutsideMessaging, RunningTimeClock, UserInterface},
    Error,
};
use keret_controller_domain::{AppMode, StateUpdateResult};
use snafu::ResultExt;

#[cfg(test)]
mod test;

/// application service to orchestrate the domain logic
pub struct ApplicationService<TClock, TDisplay, TUserInterface, TSerialBus, TReportFunc>
where
    TClock: RunningTimeClock,
    TDisplay: Display,
    TUserInterface: UserInterface,
    TSerialBus: OutsideMessaging,
    TReportFunc: FnMut(&Error<TSerialBus::Error>) + Send + Sync,
{
    pub running_timer: TClock,
    pub display: TDisplay,
    pub controls: TUserInterface,
    serial_bus: TSerialBus,
    report_error: TReportFunc,
}

impl<TClock, TDisplay, TUserInterface, TSerialBus, TReportFunc>
    ApplicationService<TClock, TDisplay, TUserInterface, TSerialBus, TReportFunc>
where
    TClock: RunningTimeClock,
    TDisplay: Display,
    TUserInterface: UserInterface,
    TSerialBus: OutsideMessaging,
    TReportFunc: FnMut(&Error<TSerialBus::Error>) + Send + Sync,
{
    /// setup a new `ApplicationService` instance
    #[inline]
    pub fn new(
        running_timer: TClock,
        display: TDisplay,
        controls: TUserInterface,
        serial_bus: TSerialBus,
        report_error: TReportFunc,
    ) -> Self {
        Self {
            running_timer,
            display,
            controls,
            serial_bus,
            report_error,
        }
    }

    /// run the next cycle of the main logic loop, returning the new state
    pub fn next_cycle(&mut self, mode: &AppMode) -> AppMode {
        let next = self
            .calculate_next_state(mode)
            .unwrap_or_else(|e| self.handle_runtime_error(e));
        self.display.show_mode(&next);

        next
    }

    /// calculate the next state:
    /// check what the user requested to do (by clicking on buttons) and
    /// let domain layer calculate the next state based on this input
    fn calculate_next_state(
        &mut self,
        mode: &AppMode,
    ) -> Result<AppMode, Error<TSerialBus::Error>> {
        let request = self.controls.requested_interaction();
        let time = self.running_timer.now();

        let StateUpdateResult {
            mode,
            result: message,
        } = mode
            .handle_interaction_request(request, time)
            .context(DomainErrorOccurredSnafu)?;

        if let Some(message) = message {
            self.serial_bus
                .send_result(message)
                .context(SendingMessageToOutsideFailedSnafu)?;
        }

        Ok(mode)
    }

    /// report an error that happened while executing the main loop
    /// and switch the AppMode appropriately to indicate it's in a failure state
    fn handle_runtime_error(&mut self, err: Error<TSerialBus::Error>) -> AppMode {
        (self.report_error)(&err);
        AppMode::Error
    }
}
