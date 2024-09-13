use crate::repository::{LockPoisonedSnafu, RepositoryError, RepositoryStorage, ToDoRepository};
use keret_service_transmit::ActionReport;
use std::sync::{Arc, RwLock};
use tracing::instrument;

#[derive(Clone)]
pub(crate) struct StorageBasedRepository {
    storage: Arc<RwLock<dyn RepositoryStorage + Send + Sync>>,
}

impl StorageBasedRepository {
    pub(crate) fn new<T>(storage: T) -> Self
    where
        T: RepositoryStorage + Send + Sync + 'static,
    {
        Self {
            storage: Arc::new(RwLock::new(storage)),
        }
    }
}

impl ToDoRepository for StorageBasedRepository {
    #[instrument(skip(self))]
    fn list(&self) -> Result<Vec<ActionReport>, RepositoryError> {
        let Ok(repo) = self.storage.read() else {
            return LockPoisonedSnafu.fail();
        };

        repo.list()
    }

    #[instrument(skip(self))]
    fn add(&self, value: ActionReport) -> Result<usize, RepositoryError> {
        let Ok(mut repo) = self.storage.write() else {
            return LockPoisonedSnafu.fail();
        };

        let mut list = repo.list()?;

        let new_index = list.len();
        list.push(value);
        repo.store(list)?;

        Ok(new_index)
    }
}
