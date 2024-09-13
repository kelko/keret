use std::backtrace::Backtrace;
use snafu::Snafu;
use keret_service_transmit::ActionReport;

mod yaml_storage;
mod storage_based_repo;

pub(crate) use yaml_storage::YamlRepositoryStorage;
pub(crate) use storage_based_repo::StorageBasedRepository;

#[derive(Snafu, Debug)]
#[snafu()]
pub enum RepositoryError {
    #[snafu(display("Can't open the repository file"))]
    CantOpenRepository {
        #[snafu(source(from(std::io::Error, Box::new)))]
        source: Box<std::io::Error>
    },
    #[snafu(display("Deserialization of todos from repository failed"))]
    DeserializationFailed {
        #[snafu(source(from(serde_yaml::Error, Box::new)))]
        source: Box<serde_yaml::Error>
    },
    #[snafu(display("Deserialization of todos into repository failed"))]
    SerializationFailed {
        #[snafu(source(from(serde_yaml::Error, Box::new)))]
        source: Box<serde_yaml::Error>
    },
    #[snafu(display("Lock is poisoned, can't acquire"))]
    LockPoisoned {
        backtrace: Option<Backtrace>
    },
}

pub(crate) trait RepositoryStorage {
    fn list(&self) -> Result<Vec<ActionReport>, RepositoryError>;
    fn store(&mut self, list: Vec<ActionReport>) -> Result<(), RepositoryError>;
}

pub(crate) trait ToDoRepository : Clone + Send + Sync {
    fn list(&self) -> Result<Vec<ActionReport>, RepositoryError>;
    fn add(&self, value: ActionReport) -> Result<usize, RepositoryError>;
}
