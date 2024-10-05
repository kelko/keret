use crate::{
    domain::model::{Instant, InteractionRequest, StateUpdateResult},
    error::{Error, IncoherentTimestampsSnafu, NoTimerSnafu},
};

/// current state of the application logic (the "domain")
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub(crate) enum AppMode {
    /// app currently does nothing except idling
    #[default]
    Idle,
    /// the app has marked when time tracking started, waiting to finish it
    Running(Instant),
    /// the app ran into a (recoverable) error in the main loop
    Error,
}

impl AppMode {
    /// check what interaction the user requested to perform and calculate next state from that
    pub(crate) fn handle_interaction_request(
        &self,
        request: InteractionRequest,
        now: Option<Instant>,
    ) -> Result<StateUpdateResult, Error> {
        match request {
            InteractionRequest::ToggleMode => {
                let Some(timestamp) = now else {
                    return NoTimerSnafu.fail();
                };

                self.toggle_mode(timestamp)
            }
            InteractionRequest::Reset => Ok(StateUpdateResult::new(AppMode::Idle)),
            InteractionRequest::None => Ok(StateUpdateResult::new(*self)),
        }
    }

    /// user hit right button -> toggle between idle & running if possible
    /// sending the report over the serial bus if necessary
    #[inline(always)]
    fn toggle_mode(&self, timestamp: Instant) -> Result<StateUpdateResult, Error> {
        match self {
            AppMode::Idle => Ok(StateUpdateResult::new(AppMode::Running(timestamp))),
            AppMode::Running(start) => self.finish_report(start, timestamp),
            AppMode::Error => Ok(StateUpdateResult::new(*self)),
        }
    }

    /// user ended the timer, calculate duration and send it over the wire
    fn finish_report(
        &self,
        start_timestamp: &Instant,
        end_timestamp: Instant,
    ) -> Result<StateUpdateResult, Error> {
        if start_timestamp > &end_timestamp {
            return IncoherentTimestampsSnafu {
                start: *start_timestamp,
                end: end_timestamp,
            }
            .fail();
        }

        let duration = end_timestamp - *start_timestamp;
        Ok(StateUpdateResult::with_message(
            AppMode::Idle,
            duration.into(),
        ))
    }
}
