use crate::app_service::{ApplicationService, Error};
use crate::model::TrackResult;
use async_trait::async_trait;
use mockall::mock;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum TestError {
    #[snafu(display("Test Error"))]
    ErrorForTest,
}

const DURATION: u64 = 10;

// create mocks of the ports

mock! {
    MyTrackResultInput {}

    impl crate::app_service::ports::TrackResultInput for MyTrackResultInput {
        type Error = TestError;
        fn read_next_report(&mut self) -> Result<Option<TrackResult>, TestError>;
    }
}

mock! {
    MyReportMessaging {}

    #[async_trait]
    impl crate::app_service::ports::ReportMessaging for MyReportMessaging {
        type Error = TestError;
        async fn send(&self, report: TrackResult) -> Result<(), TestError>;
    }
}

// actual tests

#[tokio::test]
async fn read_and_forward_having_report_is_send() {
    // arrange
    let mut input = MockMyTrackResultInput::default();
    input
        .expect_read_next_report()
        .once()
        .returning(|| Ok(Some(TrackResult::from(DURATION))));

    let mut output = MockMyReportMessaging::default();
    output.expect_send().once().returning(|_| Ok(()));

    let mut app_service = ApplicationService::new(input, output);

    // act
    let _ = app_service.read_and_forward().await;

    // assert -> mockall
}

#[tokio::test]
async fn read_and_forward_failing_read_does_not_send() {
    // arrange
    let mut input = MockMyTrackResultInput::default();
    input
        .expect_read_next_report()
        .once()
        .returning(|| ErrorForTestSnafu.fail());

    let mut output = MockMyReportMessaging::default();
    output.expect_send().never();

    let mut app_service = ApplicationService::new(input, output);

    // act
    let _ = app_service.read_and_forward().await;

    // assert -> mockall
}

#[tokio::test]
async fn read_and_forward_failing_read_returns_error() {
    // arrange
    let mut input = MockMyTrackResultInput::default();
    input
        .expect_read_next_report()
        .once()
        .returning(|| ErrorForTestSnafu.fail());

    let mut output = MockMyReportMessaging::default();
    output.expect_send().never();

    let mut app_service = ApplicationService::new(input, output);

    // act
    let actual = app_service.read_and_forward().await;

    // assert -> mockall
    assert!(matches!(
        actual,
        Err(Error::FailedListeningForReport { .. })
    ));
}

#[tokio::test]
async fn read_and_forward_failing_send_returns_error() {
    // arrange
    let mut input = MockMyTrackResultInput::default();
    input
        .expect_read_next_report()
        .once()
        .returning(|| Ok(Some(TrackResult::from(DURATION))));

    let mut output = MockMyReportMessaging::default();
    output
        .expect_send()
        .once()
        .returning(|_| ErrorForTestSnafu.fail());

    let mut app_service = ApplicationService::new(input, output);

    // act
    let actual = app_service.read_and_forward().await;

    // assert -> mockall
    assert!(matches!(actual, Err(Error::FailedSendingToTarget { .. })));
}
