use crate::error::NoTimerSnafu;
use crate::{
    controls::InteractionRequest,
    error::{Error, IncoherentTimestampsSnafu},
    serialize::SerialBus,
};
use microbit::hal::uarte::Instance;

type Instant = u64;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub(crate) enum AppMode {
    #[default]
    Idle,
    Running(Instant),
    Error,
}

impl AppMode {
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

    #[inline(always)]
    pub(crate) fn toggle_mode<T: Instance>(
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
