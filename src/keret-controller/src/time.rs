use crate::error::{ClockInitializationFailedSnafu, Error};
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::hal::rtc::RtcCompareReg;
use microbit::{
    hal::rtc::RtcInterrupt,
    hal::{Clocks, Rtc},
    pac::{self, interrupt, CLOCK, RTC1},
};

static RTC_TIMER: Mutex<RefCell<Option<Rtc<RTC1>>>> = Mutex::new(RefCell::new(None));
static PERIOD: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));

pub(crate) fn init_time(clock: CLOCK, rtc: RTC1) -> Result<(), Error> {
    Clocks::new(clock).start_lfclk();

    let Ok(mut rtc1) = Rtc::new(rtc, 0) else {
        return ClockInitializationFailedSnafu.fail();
    };

    if rtc1.set_compare(RtcCompareReg::Compare3, 0x800000).is_err() {
        return ClockInitializationFailedSnafu.fail();
    }

    rtc1.enable_event(RtcInterrupt::Overflow);
    rtc1.enable_interrupt(RtcInterrupt::Overflow, None);
    rtc1.enable_event(RtcInterrupt::Compare3);
    rtc1.enable_interrupt(RtcInterrupt::Compare3, None);
    rtc1.clear_counter();
    rtc1.enable_counter();

    while rtc1.get_counter() != 0 {}

    free(move |cs| {
        *RTC_TIMER.borrow(cs).borrow_mut() = Some(rtc1);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::RTC1);
        }
    });

    Ok(())
}

fn construct_ticks(period: u32, counter: u32) -> u64 {
    ((period as u64) << 23) + ((counter ^ ((period & 1) << 23)) as u64)
}

pub(crate) fn now() -> Option<u64> {
    let (period, current_value) = free(|cs| {
        let p = *PERIOD.borrow(cs).borrow();

        let timer = RTC_TIMER.borrow(cs).borrow_mut();
        let v = timer.as_ref().map(|rtc| rtc.get_counter());

        (p, v)
    });

    current_value.map(|v| construct_ticks(period, v) / 32768)
}

#[interrupt]
unsafe fn RTC1() {
    free(|cs| {
        if let Some(rtc) = RTC_TIMER.borrow(cs).borrow_mut().as_mut() {
            if rtc.is_event_triggered(RtcInterrupt::Overflow) {
                rtc.reset_event(RtcInterrupt::Overflow);
            }

            if rtc.is_event_triggered(RtcInterrupt::Compare3) {
                rtc.reset_event(RtcInterrupt::Compare3);
            }
            PERIOD.borrow(cs).replace_with(|x| *x + 1);
        }
    })
}
