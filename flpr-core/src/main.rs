#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use riscv_rt::entry;

use riscv::{self as _, delay::McycleDelay};

#[entry]
fn main() -> ! {
    let peripherals = nrf54l15_flpr_pac::Peripherals::take().unwrap();

    // Turn on LED1
    let gpio_p1 = peripherals.global_p1_s;

    gpio_p1.outset().write(|w| w.pin10().bit(true));
    gpio_p1.dirset().write(|w| w.pin10().set_bit());

    // 32 MHz seems to be the correct frequency for the RISCV core,
    // but it's not documented in the datasheet.
    const TICKS_PER_SECOND: u32 = 32_000_000;

    let mut delay = McycleDelay::new(TICKS_PER_SECOND);

    // Enable cycle counter, by clearing the CY inhibit bit
    unsafe {
        riscv::register::mcountinhibit::clear_cy();
    }

    loop {
        delay.delay_ms(1_000);
        gpio_p1.outclr().write(|w| w.pin10().bit(true));
        delay.delay_ms(1_000);
        gpio_p1.outset().write(|w| w.pin10().bit(true));
    }
}
