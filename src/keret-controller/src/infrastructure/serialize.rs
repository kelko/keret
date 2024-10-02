use crate::{
    domain::model::Duration,
    error::{DeserializeMessageFailedSnafu, Error, WritingToSerialPortFailedSnafu},
};
use keret_controller_transmit::ActionReport;
use microbit::{
    board::UartPins,
    hal::uarte::{Baudrate, Instance, Parity, Uarte},
};
use snafu::ResultExt;

/// convenience abstraction of the BSP serial bus
#[repr(transparent)]
pub(crate) struct SerialBus<T> {
    serial: Uarte<T>,
}

impl<T: Instance> SerialBus<T> {
    /// create a new instance and configure the UARTE-based serial bus
    pub(crate) fn new(board_uarte: T, pins: UartPins) -> Self {
        let serial = Uarte::new(
            board_uarte,
            pins.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );

        Self { serial }
    }
}

impl<T: Instance> crate::domain::port::SerialBus for SerialBus<T> {
    /// send the duration as message via the serial bus
    fn serialize_message(&mut self, duration: Duration) -> Result<(), Error> {
        let report = ActionReport::new(duration.into());
        let serialized_message = report.as_message().context(DeserializeMessageFailedSnafu)?;

        self.serial
            .write(&serialized_message)
            .context(WritingToSerialPortFailedSnafu)?;

        let end_of_line = [b'\n'];

        self.serial
            .write(&end_of_line)
            .context(WritingToSerialPortFailedSnafu)?;

        Ok(())
    }
}
