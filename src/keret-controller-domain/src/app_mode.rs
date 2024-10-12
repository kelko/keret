use crate::results::StateUpdateResult;
use crate::{error::IncoherentTimestampsSnafu, Error, Instant, InteractionRequest};

/// current state of the application logic (the "domain")
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum AppMode {
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
    pub fn handle_interaction_request(
        &self,
        request: InteractionRequest,
        timestamp: Instant,
    ) -> Result<StateUpdateResult, Error> {
        match request {
            InteractionRequest::ToggleMode => self.toggle_mode(timestamp),
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
        Ok(StateUpdateResult::with_result(
            AppMode::Idle,
            duration.into(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Duration, TrackResult};

    const SOME_TIMESTAMP: u64 = 0xDA7A_u64;
    const DIFFERENCE: u64 = 100;
    const BIGGER_TIMESTAMP: u64 = SOME_TIMESTAMP + DIFFERENCE;

    #[test]
    fn app_mode_default_is_idle() {
        // act
        let actual = AppMode::default();

        // assert
        assert_eq!(actual, AppMode::Idle);
    }

    #[test]
    fn app_mode_of_idle_handle_none_interaction_request_keeps_idle() {
        // arrange
        let mode = AppMode::Idle;
        let interaction_request = InteractionRequest::None;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(actual, Ok(StateUpdateResult::new(AppMode::Idle)));
    }

    #[test]
    fn app_mode_of_idle_handle_reset_interaction_request_keeps_idle() {
        // arrange
        let mode = AppMode::Idle;
        let interaction_request = InteractionRequest::Reset;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(actual, Ok(StateUpdateResult::new(AppMode::Idle)));
    }

    #[test]
    fn app_mode_of_idle_handle_toggle_interaction_request_returns_running() {
        // arrange
        let mode = AppMode::Idle;
        let interaction_request = InteractionRequest::ToggleMode;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(
            actual,
            Ok(StateUpdateResult::new(AppMode::Running(
                SOME_TIMESTAMP.into()
            )))
        );
    }

    #[test]
    fn app_mode_of_running_handle_none_interaction_request_keeps_running() {
        // arrange
        let mode = AppMode::Running(SOME_TIMESTAMP.into());
        let interaction_request = InteractionRequest::None;
        let timestamp: Instant = BIGGER_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(
            actual,
            Ok(StateUpdateResult::new(AppMode::Running(
                SOME_TIMESTAMP.into()
            )))
        );
    }

    #[test]
    fn app_mode_of_running_handle_reset_interaction_request_returns_idle_without_result() {
        // arrange
        let mode = AppMode::Running(SOME_TIMESTAMP.into());
        let interaction_request = InteractionRequest::Reset;
        let timestamp: Instant = BIGGER_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(actual, Ok(StateUpdateResult::new(AppMode::Idle)));
    }

    #[test]
    fn app_mode_of_running_handle_toggle_interaction_request_returns_idle_with_result() {
        // arrange
        let mode = AppMode::Running(SOME_TIMESTAMP.into());
        let interaction_request = InteractionRequest::ToggleMode;
        let timestamp: Instant = BIGGER_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(
            actual,
            Ok(StateUpdateResult::with_result(
                AppMode::Idle,
                TrackResult::from(Duration::from(DIFFERENCE))
            ))
        );
    }

    #[test]
    fn app_mode_of_running_handle_toggle_interaction_request_with_smaller_end_returns_error() {
        // arrange
        let mode = AppMode::Running(BIGGER_TIMESTAMP.into());
        let interaction_request = InteractionRequest::ToggleMode;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(
            actual,
            Err(Error::IncoherentTimestamps {
                start: BIGGER_TIMESTAMP.into(),
                end: SOME_TIMESTAMP.into()
            })
        );
    }

    #[test]
    fn app_mode_of_error_handle_none_interaction_request_keeps_error() {
        // arrange
        let mode = AppMode::Error;
        let interaction_request = InteractionRequest::None;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(actual, Ok(StateUpdateResult::new(AppMode::Error)));
    }

    #[test]
    fn app_mode_of_error_handle_toggle_interaction_request_keeps_error() {
        // arrange
        let mode = AppMode::Error;
        let interaction_request = InteractionRequest::ToggleMode;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(actual, Ok(StateUpdateResult::new(AppMode::Error)));
    }

    #[test]
    fn app_mode_of_error_handle_result_interaction_request_returns_idle() {
        // arrange
        let mode = AppMode::Error;
        let interaction_request = InteractionRequest::Reset;
        let timestamp: Instant = SOME_TIMESTAMP.into();

        // act
        let actual = mode.handle_interaction_request(interaction_request, Some(timestamp));

        // assert
        assert_eq!(actual, Ok(StateUpdateResult::new(AppMode::Idle)));
    }

    #[test]
    fn any_app_mode_handle_toggle_interaction_request_without_timestamp_returns_error() {
        // arrange
        let mode = AppMode::Error;
        let interaction_request = InteractionRequest::ToggleMode;

        // act
        let actual = mode.handle_interaction_request(interaction_request, None);

        // assert
        assert_eq!(actual, Err(Error::NoTimer));
    }
}
