use crate::error::NoTimerSnafu;
use crate::{
    controls::InteractionRequest,
    error::{Error, IncoherentTimestampsSnafu},
    serialize::SerialBus,
};
use microbit::hal::uarte::Instance;

type Instant = u64;

/// current state of the application logic (the "domain")
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub(crate) enum AppMode {
    #[default]
    Idle,
    Running(Instant),
    Error,
}

impl AppMode {
    /// check what interaction the user requested to perform and calculate next state from that
    pub(crate) fn handle_interaction_request<T: Instance>(
        &self,
        request: InteractionRequest,
        now: Option<Instant>,
        serial_bus: &mut SerialBus<T>,
    ) -> Result<Self, Error> {
        match request {
            InteractionRequest::ToggleMode => {
                let Some(timestamp) = now else {
                    return NoTimerSnafu.fail();
                };

                self.toggle_mode(serial_bus, timestamp)
            }
            InteractionRequest::Reset => Ok(AppMode::Idle),
            InteractionRequest::None => Ok(*self),
        }
    }

    /// user hit right button -> toggle between idle & running if possible
    /// sending the report over the serial bus if necessary
    #[inline(always)]
    fn toggle_mode<T: Instance>(
        &self,
        serial_bus: &mut SerialBus<T>,
        timestamp: u64,
    ) -> Result<AppMode, Error> {
        match self {
            AppMode::Idle => Ok(AppMode::Running(timestamp)),
            AppMode::Running(start) => self.finish_report(serial_bus, *start, timestamp),
            AppMode::Error => Ok(*self),
        }
    }

    /// user ended the timer, calculate duration and send it over the wire
    fn finish_report<T: Instance>(
        &self,
        serial_bus: &mut SerialBus<T>,
        start_timestamp: u64,
        end_timestamp: u64,
    ) -> Result<AppMode, Error> {
        if start_timestamp > end_timestamp {
            return IncoherentTimestampsSnafu {
                start: start_timestamp,
                end: end_timestamp,
            }
            .fail();
        }

        let duration = end_timestamp - start_timestamp;
        serial_bus.serialize_message(duration)?;

        Ok(AppMode::Idle)
    }
}
