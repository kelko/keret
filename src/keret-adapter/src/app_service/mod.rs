mod error;
pub(crate) mod ports;
#[cfg(test)]
mod test;

use crate::app_service::error::{FailedListeningForReportSnafu, FailedSendingToTargetSnafu};
use crate::app_service::ports::{ReportMessaging, TrackResultInput};
pub(crate) use error::Error;
use snafu::ResultExt;

pub(crate) struct ApplicationService<TInput, TOutput>
where
    TInput: TrackResultInput + 'static,
    TOutput: ReportMessaging + 'static,
{
    input: TInput,
    output: TOutput,
}

impl<TInput, TOutput> ApplicationService<TInput, TOutput>
where
    TInput: TrackResultInput + 'static,
    TOutput: ReportMessaging + 'static,
{
    pub(crate) fn new(input: TInput, output: TOutput) -> Self {
        Self { input, output }
    }

    pub(crate) async fn read_and_forward(&mut self) -> Result<(), Error> {
        let report = self
            .input
            .read_next_report()
            .boxed()
            .context(FailedListeningForReportSnafu)?;
        if let Some(report) = report {
            self.output
                .send(report)
                .await
                .boxed()
                .context(FailedSendingToTargetSnafu)?;
        }

        Ok(())
    }
}
