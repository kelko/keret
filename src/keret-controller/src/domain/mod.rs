/// defines all domain layer models processing the logic
pub(crate) mod model;
/// defines all dependencies of the domain layer which are implemented outside
pub(crate) mod port;

mod application_service;

// only publish the ApplicationService itself from the application_service module, as it was defined here
pub(crate) use application_service::ApplicationService;
