use cortex_m::interrupt::free;
use microbit::hal::uarte;

use crate::error::{Error, IncoherentTimestampsSnafu, NoTimerSnafu};
use crate::{
    display::display_image, mode::AppMode, serialize::serialize_message, time::now, CURRENT_MODE,
};

pub(crate) fn toggle_mode<T: uarte::Instance>(
    serial: &mut microbit::hal::uarte::Uarte<T>,
) -> Result<(), Error> {
    let current = current_mode();

    match current {
        AppMode::Idle => {
            start_report()?;
        }
        AppMode::Running(start) => {
            finish_report(serial, start)?;
        }
        AppMode::Error | AppMode::Sending | AppMode::Fatal => {}
    };

    Ok(())
}

fn finish_report<T: uarte::Instance>(
    serial: &mut microbit::hal::uarte::Uarte<T>,
    start_timestamp: u64,
) -> Result<(), Error> {
    switch_mode(AppMode::Sending);

    let Some(end_timestamp) = now() else {
        return NoTimerSnafu.fail();
    };

    if start_timestamp > end_timestamp {
        return IncoherentTimestampsSnafu {
            start: start_timestamp,
            end: end_timestamp,
        }
        .fail();
    }

    let duration = end_timestamp - start_timestamp;
    serialize_message(serial, duration)?;

    switch_mode(AppMode::Idle);

    Ok(())
}

fn start_report() -> Result<(), Error> {
    let Some(start_timestamp) = now() else {
        return NoTimerSnafu.fail();
    };

    switch_mode(AppMode::Running(start_timestamp));

    Ok(())
}

pub(crate) fn reset_mode() {
    let current = current_mode();
    if current == AppMode::Fatal {
        return;
    }
    switch_mode(AppMode::Idle);
}

pub(crate) fn switch_mode(next: AppMode) {
    free(|cs| {
        CURRENT_MODE.borrow(cs).replace(next);
    });
    display_image(&next);
}

#[inline(always)]
fn current_mode() -> AppMode {
    free(|cs| *CURRENT_MODE.borrow(cs).borrow())
}
