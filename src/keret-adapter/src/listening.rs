use keret_controller_transmit::ActionReport;
use serialport::SerialPort;
use snafu::{ResultExt, Snafu};
use std::io::ErrorKind;
use std::time::Duration;

#[derive(Debug, Snafu)]
pub(crate) enum ListeningError {
    #[snafu(display("Could not open serial port using {device}"))]
    CouldNotOpenPort {
        device: String,
        source: serialport::Error,
    },
    #[snafu(display("Could not read data from serial port"))]
    CouldNotReadFromPort { source: std::io::Error },
    #[snafu(display("Could not deserialize the serial message"))]
    CouldNotDeserializeMessage {
        source: keret_controller_transmit::Error,
    },
}

pub(crate) struct PortListener {
    port: Box<dyn SerialPort>,
    buffer: Vec<u8>,
}

impl PortListener {
    pub(crate) fn new(path: &str) -> Result<Self, ListeningError> {
        let port = serialport::new(path, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .context(CouldNotOpenPortSnafu {
                device: path.to_string(),
            })?;

        let buffer = Vec::<u8>::new();

        Ok(Self { port, buffer })
    }

    pub(crate) fn read_next_report(&mut self) -> Result<Option<ActionReport>, ListeningError> {
        let mut read_buffer: Vec<u8> = vec![0; 10];

        match self.port.read(&mut read_buffer) {
            Ok(length) => {
                self.buffer.extend_from_slice(&read_buffer[..length]);
            }
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {
                    return Ok(None);
                }
                _ => {
                    return Err(ListeningError::CouldNotReadFromPort { source: e });
                }
            },
        };

        if let Some(index) = self.buffer.iter().position(|&x| x == b'\n') {
            if index == 0 {
                self.buffer.pop();
                return Ok(None);
            }

            let incoming_report = ActionReport::from_message(&self.buffer[..index])
                .context(CouldNotDeserializeMessageSnafu)?;
            for _ in 0..=index {
                self.buffer.pop();
            }

            Ok(Some(incoming_report))
        } else {
            Ok(None)
        }
    }
}
