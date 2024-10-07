use crate::error::Error;
use keret_controller_domain::{AppMode, Instant, InteractionRequest, TrackResult};

/// Keep track of the running time, producing an ever-increasing, never resetting timestamp
pub(crate) trait RunningTimeClock {
    /// return the current timestamp
    fn now(&mut self) -> Instant;
}

/// Send domain-specific messages to the outside
pub(crate) trait OutsideMessaging {
    /// inform the outside of the time tracking result
    fn send_result(&mut self, result: TrackResult) -> Result<(), Error>;
}

/// Show domain-specific content on the display
pub(crate) trait Display {
    /// display a sprite associated with the given `AppMode`
    fn show_mode(&mut self, mode: &AppMode);
}

/// retrieve input from the user
pub(crate) trait UserInterface {
    /// return the requested interface (as calculated from the inputs the user made)
    fn requested_interaction(&mut self) -> InteractionRequest;
}
