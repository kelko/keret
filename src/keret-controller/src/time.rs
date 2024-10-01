use crate::error::{ClockInitializationFailedSnafu, Error};
use microbit::{
    hal::rtc::RtcInterrupt,
    hal::rtc::{Instance, RtcCompareReg},
    hal::{Clocks, Rtc},
    pac::{self, CLOCK},
};

pub(crate) struct RunningTimer<T> {
    rtc_timer: Rtc<T>,
    period: u32,
}

impl<T: Instance> RunningTimer<T> {
    pub(crate) fn new(clock: CLOCK, rtc_component: T) -> Result<Self, Error> {
        Clocks::new(clock).start_lfclk();

        let Ok(mut rtc) = Rtc::new(rtc_component, 0) else {
            return ClockInitializationFailedSnafu.fail();
        };

        if rtc.set_compare(RtcCompareReg::Compare3, 0x800000).is_err() {
            return ClockInitializationFailedSnafu.fail();
        }

        rtc.enable_event(RtcInterrupt::Overflow);
        rtc.enable_interrupt(RtcInterrupt::Overflow, None);
        rtc.enable_event(RtcInterrupt::Compare3);
        rtc.enable_interrupt(RtcInterrupt::Compare3, None);
        rtc.clear_counter();
        rtc.enable_counter();

        while rtc.get_counter() != 0 {}
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::RTC1);
        }

        Ok(Self {
            rtc_timer: rtc,
            period: 0,
        })
    }

    #[inline(always)]
    pub(crate) fn now(&mut self) -> u64 {
        let current_value = self.rtc_timer.get_counter();

        construct_ticks(self.period, current_value) / 32768
    }

    pub(crate) fn tick_timer(&mut self) {
        let rtc = &self.rtc_timer;
        if rtc.is_event_triggered(RtcInterrupt::Overflow) {
            rtc.reset_event(RtcInterrupt::Overflow);
        }

        if rtc.is_event_triggered(RtcInterrupt::Compare3) {
            rtc.reset_event(RtcInterrupt::Compare3);
        }
        self.period += 1;
    }
}

#[inline(always)]
fn construct_ticks(period: u32, counter: u32) -> u64 {
    ((period as u64) << 23) + ((counter ^ ((period & 1) << 23)) as u64)
}
