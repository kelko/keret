mod dependencies;
mod model;
// compile tests only when testing
mod application_service;

// re-export everything relevant from the submodules as if it was directly coded here
// hides internal structure of the module
pub(crate) use application_service::ApplicationService;
pub(crate) use dependencies::*;
pub(crate) use model::{AppMode, Duration, Instant};
