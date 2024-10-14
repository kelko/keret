use snafu::Snafu;
use std::backtrace::Backtrace;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
    #[snafu(display("An error occurred while listening on input"))]
    FailedListeningForReport {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        backtrace: Option<Backtrace>,
    },
    #[snafu(display("An error occurred while trying to send report"))]
    FailedSendingToTarget {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        backtrace: Option<Backtrace>,
    },
}
