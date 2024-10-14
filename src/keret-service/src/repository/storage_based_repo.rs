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

#[cfg(test)]
mod test {
    use super::*;
    use crate::repository::*;
    use chrono::{DateTime, Utc};
    use mockall::mock;
    use mockall::predicate::eq;
    use std::time::Duration;

    mock! {
        MyRepositoryStorage {}

        impl RepositoryStorage for MyRepositoryStorage {
            fn list(&self) -> Result<Vec<ActionReport>, RepositoryError>;
            fn store(&mut self, list: Vec<ActionReport>) -> Result<(), RepositoryError>;
        }
    }

    #[test]
    fn list_with_empty_storage_returns_empty_vec() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage.expect_list().once().returning(|| Ok(vec![]));
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.list();

        // assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), vec![]);
    }

    #[test]
    fn list_with_filled_storage_returns_all_elements() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage.expect_list().once().returning(|| {
            Ok(vec![
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(10, 0).unwrap(),
                    Duration::new(1, 0),
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(20, 0).unwrap(),
                    Duration::new(1, 0),
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(30, 0).unwrap(),
                    Duration::new(1, 0),
                ),
            ])
        });
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.list();

        // assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            vec![
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(10, 0).unwrap(),
                    Duration::new(1, 0)
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(20, 0).unwrap(),
                    Duration::new(1, 0)
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(30, 0).unwrap(),
                    Duration::new(1, 0)
                ),
            ]
        );
    }

    #[test]
    fn list_failing_read_from_storage_returns_error() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage
            .expect_list()
            .once()
            .returning(|| ErrorOnTestSnafu.fail());
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.list();

        // assert
        assert!(matches!(actual, Err(RepositoryError::ErrorOnTest)));
    }

    #[test]
    fn add_on_empty_storage_writes_one_element() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage.expect_list().once().returning(|| Ok(vec![]));
        storage
            .expect_store()
            .once()
            .with(eq(vec![ActionReport::new(
                DateTime::<Utc>::from_timestamp(10, 0).unwrap(),
                Duration::new(1, 0),
            )]))
            .returning(|_| Ok(()));
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.add(ActionReport::new(
            DateTime::<Utc>::from_timestamp(10, 0).unwrap(),
            Duration::new(1, 0),
        ));

        // assert -> mockall
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 0);
    }

    #[test]
    fn add_on_filled_storage_writes_all_plus_one_elements() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage.expect_list().once().returning(|| {
            Ok(vec![
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(10, 0).unwrap(),
                    Duration::new(1, 0),
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(20, 0).unwrap(),
                    Duration::new(1, 0),
                ),
            ])
        });
        storage
            .expect_store()
            .once()
            .with(eq(vec![
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(10, 0).unwrap(),
                    Duration::new(1, 0),
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(20, 0).unwrap(),
                    Duration::new(1, 0),
                ),
                ActionReport::new(
                    DateTime::<Utc>::from_timestamp(90, 0).unwrap(),
                    Duration::new(1, 0),
                ),
            ]))
            .returning(|_| Ok(()));
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.add(ActionReport::new(
            DateTime::<Utc>::from_timestamp(90, 0).unwrap(),
            Duration::new(1, 0),
        ));

        // assert -> mockall
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 2);
    }

    #[test]
    fn add_failing_read_from_storage_does_not_store() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage
            .expect_list()
            .once()
            .returning(|| ErrorOnTestSnafu.fail());
        storage.expect_store().never().returning(|_| Ok(()));
        let repo = StorageBasedRepository::new(storage);

        // act
        let _ = repo.add(ActionReport::new(
            DateTime::<Utc>::from_timestamp(90, 0).unwrap(),
            Duration::new(1, 0),
        ));

        // assert -> mockall
    }

    #[test]
    fn add_failing_read_from_storage_returns_error() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage
            .expect_list()
            .once()
            .returning(|| ErrorOnTestSnafu.fail());
        storage.expect_store().never().returning(|_| Ok(()));
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.add(ActionReport::new(
            DateTime::<Utc>::from_timestamp(90, 0).unwrap(),
            Duration::new(1, 0),
        ));

        // assert
        assert!(matches!(actual, Err(RepositoryError::ErrorOnTest)));
    }

    #[test]
    fn add_failing_store_in_storage_returns_error() {
        // arrange
        let mut storage = MockMyRepositoryStorage::default();
        storage.expect_list().once().returning(|| Ok(vec![]));
        storage
            .expect_store()
            .once()
            .with(eq(vec![ActionReport::new(
                DateTime::<Utc>::from_timestamp(90, 0).unwrap(),
                Duration::new(1, 0),
            )]))
            .returning(|_| ErrorOnTestSnafu.fail());
        let repo = StorageBasedRepository::new(storage);

        // act
        let actual = repo.add(ActionReport::new(
            DateTime::<Utc>::from_timestamp(90, 0).unwrap(),
            Duration::new(1, 0),
        ));

        // assert
        assert!(matches!(actual, Err(RepositoryError::ErrorOnTest)));
    }
}
