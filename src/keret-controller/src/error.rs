use core::fmt::{Debug, Display, Formatter};
use snafu::Snafu;

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
    #[snafu(display("Incoherent timestamps. Started at {start} & ended at {end}"))]
    IncoherentTimestamps { start: u64, end: u64 },
}
