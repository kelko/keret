use crate::{
    domain::{
        model::{Instant, InteractionRequest},
        port::SerialBus,
    },
    error::{Error, IncoherentTimestampsSnafu, NoTimerSnafu},
};

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
    pub(crate) fn handle_interaction_request<TSerialBus: SerialBus>(
        &self,
        request: InteractionRequest,
        now: Option<impl Into<Instant>>,
        serial_bus: &mut TSerialBus,
    ) -> Result<Self, Error> {
        match request {
            InteractionRequest::ToggleMode => {
                let Some(timestamp) = now else {
                    return NoTimerSnafu.fail();
                };

                self.toggle_mode(serial_bus, timestamp.into())
            }
            InteractionRequest::Reset => Ok(AppMode::Idle),
            InteractionRequest::None => Ok(*self),
        }
    }

    /// user hit right button -> toggle between idle & running if possible
    /// sending the report over the serial bus if necessary
    #[inline(always)]
    fn toggle_mode<TSerialBus: SerialBus>(
        &self,
        serial_bus: &mut TSerialBus,
        timestamp: Instant,
    ) -> Result<AppMode, Error> {
        match self {
            AppMode::Idle => Ok(AppMode::Running(timestamp)),
            AppMode::Running(start) => self.finish_report(serial_bus, start, timestamp),
            AppMode::Error => Ok(*self),
        }
    }

    /// user ended the timer, calculate duration and send it over the wire
    fn finish_report<TSerialBus: SerialBus>(
        &self,
        serial_bus: &mut TSerialBus,
        start_timestamp: &Instant,
        end_timestamp: Instant,
    ) -> Result<AppMode, Error> {
        if start_timestamp > &end_timestamp {
            return IncoherentTimestampsSnafu {
                start: *start_timestamp,
                end: end_timestamp,
            }
            .fail();
        }

        let duration = end_timestamp - *start_timestamp;
        serial_bus.serialize_message(duration)?;

        Ok(AppMode::Idle)
    }
}
