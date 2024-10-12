use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error<OutsideMessagingError>
where
    OutsideMessagingError: snafu::Error + 'static,
{
    #[snafu(display("Failed writing data to the serial port"))]
    SendingMessageToOutsideFailed { source: OutsideMessagingError },
    #[snafu(display("No controls initialized to read requested interaction from"))]
    NoControls,
    #[snafu(display("Domain Error"))]
    DomainErrorOccurred {
        source: keret_controller_domain::Error,
    },
}
