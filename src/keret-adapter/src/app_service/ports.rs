use crate::model::TrackResult;
use async_trait::async_trait;

pub(crate) trait TrackResultInput {
    type Error: std::error::Error + Send + Sync + 'static;
    fn read_next_report(&mut self) -> Result<Option<TrackResult>, Self::Error>;
}

#[async_trait]
pub(crate) trait ReportMessaging {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn send(&self, report: TrackResult) -> Result<(), Self::Error>;
}
