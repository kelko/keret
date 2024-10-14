use keret_service_transmit::ActionReport;
use snafu::Snafu;
use std::backtrace::Backtrace;

mod storage_based_repo;
mod yaml_storage;

pub(crate) use storage_based_repo::StorageBasedRepository;
pub(crate) use yaml_storage::YamlRepositoryStorage;

#[derive(Snafu, Debug)]
#[snafu()]
pub enum RepositoryError {
    #[snafu(display("Can't open the repository file"))]
    CantOpenRepository {
        #[snafu(source(from(std::io::Error, Box::new)))]
        source: Box<std::io::Error>,
    },
    #[snafu(display("Deserialization of todos from repository failed"))]
    DeserializationFailed {
        #[snafu(source(from(serde_yaml::Error, Box::new)))]
        source: Box<serde_yaml::Error>,
    },
    #[snafu(display("Deserialization of todos into repository failed"))]
    SerializationFailed {
        #[snafu(source(from(serde_yaml::Error, Box::new)))]
        source: Box<serde_yaml::Error>,
    },
    #[snafu(display("Lock is poisoned, can't acquire"))]
    LockPoisoned { backtrace: Option<Backtrace> },
    #[cfg(test)]
    #[snafu(display("Test Error"))]
    ErrorOnTest,
}

pub(crate) trait RepositoryStorage {
    fn list(&self) -> Result<Vec<ActionReport>, RepositoryError>;
    fn store(&mut self, list: Vec<ActionReport>) -> Result<(), RepositoryError>;
}

pub(crate) trait ToDoRepository: Clone + Send + Sync {
    fn list(&self) -> Result<Vec<ActionReport>, RepositoryError>;
    fn add(&self, value: ActionReport) -> Result<usize, RepositoryError>;
}
