use core::fmt::{Debug, Display, Formatter};
use rtt_target::rprintln;
use snafu::{Error as _, Snafu};

/// compatibility wrapper until core::error is used everywhere
pub struct UarteError(microbit::hal::uarte::Error);

impl UarteError {
    pub(crate) fn new(error: microbit::hal::uarte::Error) -> Self {
        Self(error)
    }
}

impl Debug for UarteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for UarteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl snafu::Error for UarteError {}

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
    IncoherentTimestamps { start: u64, end: u64 },
    #[snafu(display("Failed to initialize the clock"))]
    ClockInitializationFailed,
    #[snafu(display("No timer initialized to read the time from"))]
    NoTimer,
    #[snafu(display("No controls initialized to read requested interaction from"))]
    NoControls,
}

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
