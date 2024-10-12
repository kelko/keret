use keret_controller_domain::{AppMode, Instant, InteractionRequest, TrackResult};

/// Show domain-specific content on the display
pub trait Display {
    /// display a sprite associated with the given `AppMode`
    fn show_mode(&mut self, mode: &AppMode);
}

/// Send domain-specific messages to the outside
pub trait OutsideMessaging {
    type Error: snafu::Error + 'static;
    /// inform the outside of the time tracking result
    fn send_result(&mut self, result: TrackResult) -> Result<(), Self::Error>;
}

/// Keep track of the running time, producing an ever-increasing, never resetting timestamp
pub trait RunningTimeClock {
    /// return the current timestamp
    fn now(&mut self) -> Instant;
}

/// retrieve input from the user
pub trait UserInterface {
    /// return the requested interface (as calculated from the inputs the user made)
    fn requested_interaction(&mut self) -> InteractionRequest;
}
