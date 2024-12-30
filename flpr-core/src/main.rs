#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use panic_halt as _;

use riscv::{self as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Turn on LED1
    let mut led = Output::new(p.P1_10, Level::Low, OutputDrive::Standard);

    led.set_high();
}
