use crate::infrastructure::serialize::SerialBusError;
use rtt_target::rprintln;
use snafu::Snafu;

/// all errors which are generated inside this crate
/// as enum, so handling code can easily match over it (if necessary)
/// using snafu crate to auto-generate user-facing message which are sent over rtt
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum InitializationError {
    #[snafu(display("Failed to initialize the clock"))]
    ClockInitializationFailed,
}

/// send details of a top-level error over the rtt
pub(crate) fn report_error(err: &dyn snafu::Error) {
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

// wrapper for app service errors
#[inline]
pub(crate) fn report_domain_error(err: &keret_controller_appservice::Error<SerialBusError>) {
    report_error(err);
}
