use core::fmt::{Debug, Display, Formatter};
use snafu::Snafu;

/// compatibility wrapper until core::error is used everywhere
#[repr(transparent)]
pub(crate) struct UarteError(microbit::hal::uarte::Error);

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

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum SerialBusError {
    #[snafu(display("Failed writing data to the serial port"))]
    WritingToSerialPortFailed {
        #[snafu(source(from(microbit::hal::uarte::Error, UarteError::new)))]
        source: UarteError,
    },
    #[snafu(display("Failed to deserialize message"))]
    DeserializeMessageFailed {
        source: keret_controller_transmit::Error,
    },
}
