use snafu::{ResultExt, Snafu};
use keret_service_transmit::ActionReport;

#[derive(Debug, Snafu)]
pub(crate) enum SendingError {
    #[snafu(display("Could not send the report to URL {target}"))]
    CouldNotSendReport {
        target: String,
        source: reqwest::Error
    }

}

pub(crate) struct ReportSender {
    target: String
}

impl ReportSender {
    pub(crate) fn new(target: String) -> Self {
        Self { target }
    }

    pub(crate) async fn send(&self, report: impl Into<ActionReport>) -> Result<(), SendingError> {
        let report = report.into();
        let client = reqwest::Client::new();

        let _res = client.post(&self.target)
            .json(&report)
            .send()
            .await
            .context(CouldNotSendReportSnafu { target: self.target.clone() })?;

        Ok(())
    }
}
