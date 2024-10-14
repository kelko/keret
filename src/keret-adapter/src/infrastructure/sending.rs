use crate::model::TrackResult;
use async_trait::async_trait;
use keret_service_transmit::ActionReport;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub(crate) enum SendingError {
    #[snafu(display("Could not send the report to URL {target}"))]
    CouldNotSendReport {
        target: String,
        source: reqwest::Error,
    },
}

pub(crate) struct ReportSender {
    target: String,
}

impl ReportSender {
    pub(crate) fn new(target: String) -> Self {
        Self { target }
    }
}

#[async_trait]
impl crate::app_service::ports::ReportMessaging for ReportSender {
    type Error = SendingError;

    async fn send(&self, report: TrackResult) -> Result<(), Self::Error> {
        // extract the actual value from the adapter value object
        let report: u64 = report.into();
        // turn the value into a sendable ActionReport for the service
        let report: ActionReport = report.into();
        let client = reqwest::Client::new();

        let _res = client
            .post(&self.target)
            .json(&report)
            .send()
            .await
            .context(CouldNotSendReportSnafu {
                target: self.target.clone(),
            })?;

        Ok(())
    }
}
