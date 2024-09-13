use cortex_m::interrupt::free;
use microbit::hal::uarte;

use crate::{
    CURRENT_MODE,
    display::display_image,
    mode::AppMode,
    serialize::serialize_message,
    time::now
};
use crate::error::{Error, IncoherentTimestampsSnafu};

pub(crate) fn toggle_mode<T: uarte::Instance>(serial: &mut microbit::hal::uarte::Uarte<T>) -> Result<(), Error> {
    let current = free(|cs| {
        *CURRENT_MODE.borrow(cs).borrow()
    });

    match current {
        AppMode::Idle => {
            start_report();
        },
        AppMode::Running(start) => {
            finish_report(serial, start)?;
        },
        AppMode::Error | AppMode::Sending => {},
    };

    Ok(())
}

fn finish_report<T: uarte::Instance>(serial: &mut microbit::hal::uarte::Uarte<T>, start_timestamp: u64) -> Result<(), Error>{
    switch_mode(AppMode::Sending);

    let end_timestamp = now();
    if start_timestamp > end_timestamp {
        return IncoherentTimestampsSnafu { start: start_timestamp, end: end_timestamp}.fail()
    }

    let duration = end_timestamp - start_timestamp;
    serialize_message(serial, duration)?;

    switch_mode(AppMode::Idle);

    Ok(())
}

fn start_report() {
    let start_timestamp = now();
    switch_mode(AppMode::Running(start_timestamp));
}

#[inline(always)]
pub(crate) fn reset_mode(){
    switch_mode(AppMode::Idle);
}

#[inline(always)]
pub(crate) fn switch_mode(next: AppMode) {
    free(|cs| {
        CURRENT_MODE.borrow(cs).replace(next);
    });
    display_image(&next);
}
