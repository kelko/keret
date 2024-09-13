use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::hal::pac::SYST;

static DISPLAY: Mutex<RefCell<Option<SYST>>> = Mutex::new(RefCell::new(None));

pub(crate) fn init_time(mut board_syst: SYST) {
    board_syst.set_reload(0x00ffffff);
    board_syst.clear_current();
    //board_syst.enable_interrupt();
    board_syst.enable_counter();

    free(move |cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(board_syst);
    });
}

fn calc_now(period: u32, counter: u32) -> u64 {
    ((period as u64) << 23) + ((counter ^ ((period & 1) << 23)) as u64)
}

pub(crate) fn now() -> u64 {
    let p = 0_u32;
    let v = SYST::get_current();

    calc_now(p, v)
}
