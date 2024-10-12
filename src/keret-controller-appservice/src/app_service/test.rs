use crate::ports::{Display, OutsideMessaging, RunningTimeClock, UserInterface};
use crate::ApplicationService;
use core::cell::RefCell;
use keret_controller_domain::{AppMode, Instant, InteractionRequest, TrackResult};
use mockall::mock;
use mockall::predicate::*;
use snafu::Snafu;

const FIRST_TIMESTAMP: u64 = 0xDA7A;
const DURATION: u64 = 10;
const SECOND_TIMESTAMP: u64 = FIRST_TIMESTAMP + DURATION;

// errors used by the mocks

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum TestError {
    #[snafu(display("Test Error during Send"))]
    ErrorDuringSend,
}

// create mocks of the ports
mock! {
    MyUserInterface {}

    impl UserInterface for MyUserInterface {
        fn requested_interaction(&mut self) -> InteractionRequest;
    }
}

mock! {
    MyDisplay {}

    impl Display for MyDisplay {
        fn show_mode(&mut self, mode: &AppMode);
    }
}

mock! {
    MyOutsideMessaging {}

    impl OutsideMessaging for MyOutsideMessaging {
        type Error = TestError;
        fn send_result(&mut self, result: TrackResult) -> Result<(), TestError>;
    }
}

mock! {
    MyClock {}

    impl RunningTimeClock for MyClock {
        fn now(&mut self) -> Instant;
    }
}

fn noop_report(_err: &crate::Error<TestError>) {}

// tests

#[test]
fn next_cycle_checks_user_input() {
    // arrange
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(FIRST_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::None);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Idle))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result().never();

    let mut service = ApplicationService::new(&mut clock, &mut display, &mut ui, bus, &noop_report);
    let mode = AppMode::Idle;

    // act
    let _ = service.next_cycle(&mode);
}

#[test]
fn next_cycle_shows_mode() {}

#[test]
fn next_cycle_switches_to_running_using_timestamp() {}

#[test]
fn next_cycle_sends_message_if_tracking_finished() {}

#[test]
fn next_cycle_reports_error_on_no_timer() {}

#[test]
fn next_cycle_reports_error_on_no_controls() {}

#[test]
fn next_cycle_reports_error_on_inconsistent_timestamps() {}
