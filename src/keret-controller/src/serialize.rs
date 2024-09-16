use crate::error::{DeserializeMessageFailedSnafu, Error, WritingToSerialPortFailedSnafu};
use keret_controller_transmit::ActionReport;
use snafu::ResultExt;

pub(crate) fn serialize_message<T: microbit::hal::uarte::Instance>(
    serial: &mut microbit::hal::uarte::Uarte<T>,
    duration: u64,
) -> Result<(), Error> {
    let report = ActionReport::new(duration);
    let serialized_message = report.as_message().context(DeserializeMessageFailedSnafu)?;
    serial
        .write(&serialized_message)
        .context(WritingToSerialPortFailedSnafu)?;

    let end_of_line = [b'\n'];
    serial
        .write(&end_of_line)
        .context(WritingToSerialPortFailedSnafu)?;

    Ok(())
}
