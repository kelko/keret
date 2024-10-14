use crate::ports::{Display, OutsideMessaging, RunningTimeClock, UserInterface};
use crate::{ApplicationService, Error};
use keret_controller_domain::{AppMode, Duration, Instant, InteractionRequest, TrackResult};
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

// checks that all ports are called in one go, as this is how "mockall" works
#[test]
fn next_cycle_calls_ports() {
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

    let mut service = ApplicationService::new(clock, display, ui, bus, &noop_report);
    let mode = AppMode::Idle;

    // act
    let _ = service.next_cycle(&mode);

    // assert -> automatically by mockall mocks
}

#[test]
fn next_cycle_switches_to_running_using_timestamp() {
    // arrange
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(FIRST_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::ToggleMode);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Running(Instant::from(FIRST_TIMESTAMP))))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result().never();

    let mut service = ApplicationService::new(clock, display, ui, bus, &noop_report);
    let mode = AppMode::Idle;

    // act
    let actual = service.next_cycle(&mode);

    // assert
    assert_eq!(actual, AppMode::Running(Instant::from(FIRST_TIMESTAMP)));
}

#[test]
fn next_cycle_sends_message_if_tracking_finished_and_returns_idle() {
    // arrange
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(SECOND_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::ToggleMode);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Idle))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result()
        .once()
        .with(eq(TrackResult::from(Duration::from(DURATION))))
        .returning(|_| Ok(()));

    let mut service = ApplicationService::new(clock, display, ui, bus, &noop_report);
    let mode = AppMode::Running(Instant::from(FIRST_TIMESTAMP));

    // act
    let actual = service.next_cycle(&mode);

    // assert -> + automatically by mockall mocks
    assert_eq!(actual, AppMode::Idle);
}

#[test]
fn next_cycle_reports_error_on_inconsistent_timestamps() {
    // arrange
    let mut error_was_reported = false;
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(FIRST_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::ToggleMode);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Error))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result().never();

    let mut service = ApplicationService::new(clock, display, ui, bus, |error| {
        error_was_reported = matches!(error, Error::DomainErrorOccurred { .. });
    });
    let mode = AppMode::Running(Instant::from(SECOND_TIMESTAMP));

    // act
    let _ = service.next_cycle(&mode);

    // assert
    assert_eq!(error_was_reported, true);
}

#[test]
fn next_cycle_returns_error_on_inconsistent_timestamps() {
    // arrange
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(FIRST_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::ToggleMode);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Error))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result().never();

    let mut service = ApplicationService::new(clock, display, ui, bus, &noop_report);
    let mode = AppMode::Running(Instant::from(SECOND_TIMESTAMP));

    // act
    let actual = service.next_cycle(&mode);

    // assert -> + automatically by mockall mocks
    assert_eq!(actual, AppMode::Error);
}

#[test]
fn next_cycle_returns_error_when_sending_fails() {
    // arrange
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(SECOND_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::ToggleMode);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Error))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result()
        .once()
        .with(eq(TrackResult::from(Duration::from(DURATION))))
        .returning(|_| ErrorDuringSendSnafu.fail());

    let mut service = ApplicationService::new(clock, display, ui, bus, &noop_report);
    let mode = AppMode::Running(Instant::from(FIRST_TIMESTAMP));

    // act
    let actual = service.next_cycle(&mode);

    // assert -> + automatically by mockall mocks
    assert_eq!(actual, AppMode::Error);
}

#[test]
fn next_cycle_reports_error_when_sending_fails() {
    // arrange
    let mut error_was_reported = false;
    let mut clock = MockMyClock::new();
    clock
        .expect_now()
        .once()
        .returning(|| Instant::from(SECOND_TIMESTAMP));

    let mut ui = MockMyUserInterface::new();
    ui.expect_requested_interaction()
        .once()
        .returning(|| InteractionRequest::ToggleMode);

    let mut display = MockMyDisplay::new();
    display
        .expect_show_mode()
        .once()
        .with(eq(AppMode::Error))
        .return_const(());
    let mut bus = MockMyOutsideMessaging::new();
    bus.expect_send_result()
        .once()
        .with(eq(TrackResult::from(Duration::from(DURATION))))
        .returning(|_| ErrorDuringSendSnafu.fail());

    let mut service = ApplicationService::new(clock, display, ui, bus, |error| {
        error_was_reported = matches!(error, Error::SendingMessageToOutsideFailed { .. })
    });
    let mode = AppMode::Running(Instant::from(FIRST_TIMESTAMP));

    // act
    let _ = service.next_cycle(&mode);

    // assert -> + automatically by mockall mocks
    assert_eq!(error_was_reported, true);
}
