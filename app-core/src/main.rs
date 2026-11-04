#![no_std]
#![no_main]

use defmt::{info, unwrap};

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::{
    gpio::{Level, Output, OutputDrive},
    pac::spu::vals::Dmasec,
    vpr::Vpr,
};
use embassy_time::Timer;
use panic_probe as _;

unsafe extern "C" {
    // Provided by the linker: This is the address of the static memory
    pub static mut __shared_mem: u32;
    pub static mut __vpr_load_address: u8;
}

const NUM_IRQS: usize = 96;
const READY_MAGIC: u32 = 0xF1F1_0001;

/// Layout of the shared memory block (must match `flpr-core`).
#[repr(C)]
struct Shared {
    ready: u32,
    last_mcause: u32,
    exceptions: u32,
    _reserved: [u32; 3],
    irq_counts: [u32; NUM_IRQS],
    log_len: u32,
    log: [[u32; 2]; 8],
}

fn shared_ptr() -> *mut Shared {
    &raw mut __shared_mem as *mut Shared
}

const VPR_DATA: &[u8] = include_bytes!("../input/flpr-core.bin");

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let shared_mem_addr: *const u32 = &raw const __shared_mem;

    defmt::debug!("Shared memory address: {:?}", shared_mem_addr);

    // Clear the shared block before the FLPR starts.
    unsafe {
        let words = shared_ptr() as *mut u32;
        for i in 0..core::mem::size_of::<Shared>() / 4 {
            words.add(i).write_volatile(0);
        }
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

    let vpr_address = &raw const __vpr_load_address;

    let mut vpr = Vpr::new(p.VPR, vpr_address).unwrap();

    vpr.load(VPR_DATA).unwrap();

    info!("Start VPR core from address {:#010x}", vpr_address);

    vpr.start();

    // Wait for the FLPR to finish setting up its CLIC.
    let shared = shared_ptr();
    while unsafe { (&raw const (*shared).ready).read_volatile() } != READY_MAGIC {
        Timer::after_millis(10).await;
    }
    info!("FLPR is ready, triggering VPR00 tasks one by one");

    // Fire each VPR task and see which CLIC interrupt(s) the FLPR took.
    // Expected: tasks 0..=15 do nothing, task n in 16..=22 raises VPRCLIC_<n> (IRQn n,
    // reported as CLIC interrupt ID 16 + n in mcause).
    for n in 0..23 {
        let before = snapshot(shared);

        embassy_nrf::pac::VPR00_S.tasks_trigger(n).write_value(1);

        Timer::after_millis(10).await;

        let after = snapshot(shared);
        let mut hit = false;
        for irq in 0..NUM_IRQS {
            if after[irq] != before[irq] {
                hit = true;
                info!(
                    "TASKS_TRIGGER[{}] -> IRQn {} (+{})",
                    n,
                    irq,
                    after[irq] - before[irq]
                );
            }
        }
        if !hit {
            info!("TASKS_TRIGGER[{}] -> (no interrupt)", n);
        }
    }

    let last_mcause = unsafe { (&raw const (*shared).last_mcause).read_volatile() };
    let exceptions = unsafe { (&raw const (*shared).exceptions).read_volatile() };
    info!(
        "Done. last mcause = {:#010x}, exceptions = {}",
        last_mcause, exceptions
    );
}

fn snapshot(shared: *const Shared) -> [u32; NUM_IRQS] {
    let mut out = [0; NUM_IRQS];
    for (i, o) in out.iter_mut().enumerate() {
        *o = unsafe { (&raw const (*shared).irq_counts[i]).read_volatile() };
    }
    out
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
