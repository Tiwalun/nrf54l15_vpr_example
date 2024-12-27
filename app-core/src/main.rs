#![no_std]
#![no_main]

use defmt::{info, unwrap};

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive},
    pac::vpr::vals,
};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Turn on LED0
    let led0 = Output::new(p.P2_09, Level::Low, OutputDrive::Standard);

    unwrap!(spawner.spawn(blinker(led0)));

    info!("Configure VPR core to secure mode");
    let spu = embassy_nrf::pac::SPU00_S;

    let flpr_index = 12;

    spu.periph(flpr_index).perm().write(|w| {
        w.set_secattr(true);
        w.set_dmasec(true);
    });

    let vpr = embassy_nrf::pac::VPR00_S;

    // Start the riscv core
    const RISCV_ENTRY_ADDR: u32 = 0x00010000;

    info!("Start VPR core from address {:#010x}", RISCV_ENTRY_ADDR);

    vpr.initpc().write(|w| {
        *w = RISCV_ENTRY_ADDR;
    });

    vpr.cpurun().write(|w| w.set_en(vals::CpurunEn::RUNNING));

    // Turn on LED2
    let led2 = Output::new(p.P2_07, Level::Low, OutputDrive::Standard);
    unwrap!(spawner.spawn(blinker(led2)));
}

#[embassy_executor::task(pool_size = 2)]
async fn blinker(mut led: Output<'static>) -> ! {
    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
