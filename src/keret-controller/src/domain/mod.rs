mod dependencies;
mod model;

// re-export everything relevant from the submodules as if it was directly coded here
// hides internal structure of the module
pub(crate) use dependencies::*;
pub(crate) use model::{AppMode, Duration, Instant};
