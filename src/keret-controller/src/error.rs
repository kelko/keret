use crate::domain::Instant;
use core::fmt::{Debug, Display, Formatter};
use rtt_target::rprintln;
use snafu::{Error as _, Snafu};

/// compatibility wrapper until core::error is used everywhere
#[repr(transparent)]
pub struct UarteError(microbit::hal::uarte::Error);

impl UarteError {
    pub(crate) fn new(error: microbit::hal::uarte::Error) -> Self {
        Self(error)
    }
}

// Debug trait is used on errors to generate developer targeted information. Required by snafu::Error
impl Debug for UarteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

// Display trait is used on errors to generate the error message itself. Required by snafu::Error
impl Display for UarteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

// mark UarteError as compatible to snafu::Error trait
impl snafu::Error for UarteError {}

/// all errors which are generated inside this crate
/// as enum, so handling code can easily match over it (if necessary)
/// using snafu crate to auto-generate user-facing message which are sent over rtt
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
    #[snafu(display("Failed writing data to the serial port"))]
    WritingToSerialPortFailed {
        #[snafu(source(from(microbit::hal::uarte::Error, UarteError::new)))]
        source: UarteError,
    },
    #[snafu(display("Failed to deserialize message"))]
    DeserializeMessageFailed {
        source: keret_controller_transmit::Error,
    },
    #[snafu(display("Incoherent timestamps. Started at {start} & ended at {end}"))]
    IncoherentTimestamps { start: Instant, end: Instant },
    #[snafu(display("Failed to initialize the clock"))]
    ClockInitializationFailed,
    #[snafu(display("No timer initialized to read the time from"))]
    NoTimer,
    #[snafu(display("No controls initialized to read requested interaction from"))]
    NoControls,
}

/// send details of a top-level error over the rtt
pub(crate) fn report_error(err: Error) {
    rprintln!("[ERROR] {}", err);
    let mut source = err.source();

    if source.is_some() {
        rprintln!("[CAUSE]:");
    }
    while let Some(inner) = source {
        rprintln!("{}", inner);

        source = inner.source();
    }
}
