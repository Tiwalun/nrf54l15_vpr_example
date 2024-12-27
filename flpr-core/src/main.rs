#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

use riscv::{self as _, asm::wfi};

#[entry]
fn main() -> ! {
    let peripherals = nrf54l15_flpr_pac::Peripherals::take().unwrap();

    // Turn on LED1
    let gpio_p1 = peripherals.global_p1_s;

    gpio_p1.outset().write(|w| w.pin10().bit(true));
    gpio_p1.dirset().write(|w| w.pin10().set_bit());

    loop {
        wfi();
    }
}
