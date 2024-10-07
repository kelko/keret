use crate::Instant;
use snafu::Snafu;

#[derive(Debug, Snafu, PartialEq)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Incoherent timestamps. Started at {start} & ended at {end}"))]
    IncoherentTimestamps { start: Instant, end: Instant },
    #[snafu(display("No timer initialized to read the time from"))]
    NoTimer,
}
