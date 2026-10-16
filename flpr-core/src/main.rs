#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs as _;
use nrf_pac as pac;
use pac::gpio::vals;
use panic_halt as _;
use riscv::delay::McycleDelay;
use riscv::{self as _};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Use LED 0
    const LED_PIN: usize = 9;

    let port2 = pac::P2_S;
    port2.pin_cnf(LED_PIN).write(|w| {
        w.set_dir(vals::Dir::Output);
        w.set_input(vals::Input::Disconnect);
        w.set_pull(vals::Pull::Disabled);
        w.set_drive0(vals::Drive::S);
        w.set_drive1(vals::Drive::S);
        w.set_sense(vals::Sense::Disabled);
    });

    // 32 MHz seems to be the correct frequency for the RISCV core,
    // but it's not documented in the datasheet.
    const TICKS_PER_SECOND: u32 = 32_000_000;

    let mut delay = McycleDelay::new(TICKS_PER_SECOND);

    // Enable cycle counter, by clearing the CY inhibit bit
    unsafe {
        riscv::register::mcountinhibit::clear_cy();
    }

    loop {
        delay.delay_ms(900);
        port2.outclr().write(|w| w.set_pin(LED_PIN, true));
        delay.delay_ms(900);
        port2.outset().write(|w| w.set_pin(LED_PIN, true));
    }
}
