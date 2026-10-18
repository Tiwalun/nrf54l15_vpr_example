#![no_std]
#![no_main]

use core::{arch::global_asm, mem::MaybeUninit};

use defmt::{info, unwrap};

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive},
    pac::{spu::vals::Dmasec, vpr::vals},
};
use embassy_time::Timer;
use panic_probe as _;

unsafe extern "C" {
    // Provided by the linker: This is the address of the static memory
    pub static mut __shared_mem: u32;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let shared_mem_addr: *const u32 = &raw const __shared_mem;

    defmt::debug!("Shared memory address: {:?}", shared_mem_addr);

    // Ensure shared flag is set to 0
    unsafe {
        let shared_mem_ptr: *mut u32 = &raw mut __shared_mem;
        shared_mem_ptr.write_volatile(0);
    }

    // Turn on LED1
    let led1 = Output::new(p.P1_10, Level::Low, OutputDrive::Standard);

    spawner.spawn(unwrap!(blinker(led1)));

    info!("Configure VPR core to secure mode");
    let spu = embassy_nrf::pac::SPU00_S;

    let flpr_index = 12;

    spu.periph(flpr_index).perm().write(|w| {
        w.set_secattr(true);
        w.set_dmasec(Dmasec::Secure);
    });

    let vpr = embassy_nrf::pac::VPR00_S;

    // Start the riscv core
    const RISCV_ENTRY_ADDR: u32 = 0x00020000;

    info!("Start VPR core from address {:#010x}", RISCV_ENTRY_ADDR);

    vpr.initpc().write(|w| {
        *w = RISCV_ENTRY_ADDR;
    });

    vpr.cpurun().write(|w| w.set_en(vals::CpurunEn::Running));

    Timer::after_secs(5).await;

    defmt::info!("Asking VPR to start blinking");

    // Set shared flag to 1
    unsafe {
        let shared_mem_ptr: *mut u32 = &raw mut __shared_mem;
        shared_mem_ptr.write_volatile(1);
    }

    // Turn on LED3
    let led3 = Output::new(p.P1_14, Level::Low, OutputDrive::Standard);
    spawner.spawn(unwrap!(blinker(led3)));
}

#[embassy_executor::task(pool_size = 2)]
async fn blinker(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
