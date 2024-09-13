use crate::repository::{
    CantOpenRepositorySnafu, DeserializationFailedSnafu, RepositoryError, RepositoryStorage,
    SerializationFailedSnafu,
};
use keret_service_transmit::ActionReport;
use snafu::ResultExt;
use tracing::instrument;

#[derive(Debug)]
pub(crate) struct YamlRepositoryStorage {
    filename: String,
}

impl YamlRepositoryStorage {
    pub(crate) fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
        }
    }
}

impl RepositoryStorage for YamlRepositoryStorage {
    #[instrument(skip(self))]
    fn list(&self) -> Result<Vec<ActionReport>, RepositoryError> {
        if std::fs::metadata(&self.filename).is_err() {
            return Ok(vec![]);
        }

        let file = std::fs::File::open(&self.filename).context(CantOpenRepositorySnafu)?;
        let content = serde_yaml::from_reader(file).context(DeserializationFailedSnafu)?;

        Ok(content)
    }

    #[instrument(skip(self))]
    fn store(&mut self, list: Vec<ActionReport>) -> Result<(), RepositoryError> {
        let file = std::fs::File::create(&self.filename).context(CantOpenRepositorySnafu)?;
        serde_yaml::to_writer(file, &list).context(SerializationFailedSnafu)?;

        Ok(())
    }
}
