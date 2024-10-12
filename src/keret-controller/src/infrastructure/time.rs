use crate::error::{ClockInitializationFailedSnafu, InitializationError};
use keret_controller_appservice::ports::RunningTimeClock;
use keret_controller_domain::Instant;
use microbit::{
    hal::rtc::RtcInterrupt,
    hal::rtc::{Instance, RtcCompareReg},
    hal::{Clocks, Rtc},
    pac::CLOCK,
};

/// a timer to keep track of the overall running time of the microcontroller
/// it uses an RTC, reading the current ticks + handles any overflow of the timer
/// this way the timer can not only handle minutes to hours (RTC overflow) but several years
/// majority of functionality is modeled after
/// https://github.com/embassy-rs/embassy/blob/main/embassy-nrf/src/time_driver.rs
pub(crate) struct RunningTimer<T> {
    /// the RTC itself
    rtc_timer: Rtc<T>,
    /// how many times the clock overflowed or hit the half-overflow marker
    period: u32,
}

impl<T: Instance> RunningTimer<T> {
    /// create a new instance, configuring the RTC and starting the CLOCK in low frequency
    pub(crate) fn new(clock: CLOCK, rtc_component: T) -> Result<Self, InitializationError> {
        Clocks::new(clock).start_lfclk();

        let Ok(mut rtc) = Rtc::new(rtc_component, 511) else {
            return ClockInitializationFailedSnafu.fail();
        };

        // full overflow, the RTC sets its own value back to 0
        rtc.enable_event(RtcInterrupt::Overflow);
        rtc.enable_interrupt(RtcInterrupt::Overflow, None);

        // compare 3 is configured to the half mark for an overflow. See time_driver in embassy-nrf
        // for details
        if rtc.set_compare(RtcCompareReg::Compare3, 0x800000).is_err() {
            return ClockInitializationFailedSnafu.fail();
        }
        rtc.enable_event(RtcInterrupt::Compare3);
        rtc.enable_interrupt(RtcInterrupt::Compare3, None);

        rtc.clear_counter();
        rtc.enable_counter();

        while rtc.get_counter() != 0 {}

        Ok(Self {
            rtc_timer: rtc,
            period: 0,
        })
    }

    /// handle a interrupt from the RTC (overflow or half-mark)
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

impl<T: Instance> RunningTimeClock for RunningTimer<T> {
    /// calculates the current running time
    /// using the periods + current tick count
    #[inline(always)]
    fn now(&mut self) -> Instant {
        let current_value = self.rtc_timer.get_counter();

        (construct_ticks(self.period, current_value) / 64).into()
    }
}

/// see `calc_now` at https://github.com/embassy-rs/embassy/blob/main/embassy-nrf/src/time_driver.rs
#[inline(always)]
fn construct_ticks(period: u32, counter: u32) -> u64 {
    ((period as u64) << 23) + ((counter ^ ((period & 1) << 23)) as u64)
}
