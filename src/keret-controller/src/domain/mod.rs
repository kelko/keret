pub(crate) mod model;
pub(crate) mod port;
// compile tests only when testing
mod application_service;

// re-export everything relevant from the submodules as if it was directly coded here
// hides internal structure of the module
pub(crate) use application_service::ApplicationService;
