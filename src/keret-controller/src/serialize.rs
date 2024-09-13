use snafu::ResultExt;
use keret_controller_transmit::{ActionReport};
use crate::error::{Error, WritingToSerialPortFailedSnafu};

pub(crate) fn serialize_message<T: microbit::hal::uarte::Instance>(serial: &mut microbit::hal::uarte::Uarte<T>, duration: u64) -> Result<(), Error> {
    let report = ActionReport::new(duration);
    let serialized_message = report.as_message();
    serial.write(&serialized_message).context(WritingToSerialPortFailedSnafu)?;

    let end_of_line = [b'\n'];
    serial.write(&end_of_line).context(WritingToSerialPortFailedSnafu)?;

    Ok(())
}
