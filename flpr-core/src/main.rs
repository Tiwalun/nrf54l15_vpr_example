#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs as _;
use flpr_core::csr::{
    self, cnt::Cnt, cntmode0::CntMode0, cntmode1::CntMode1, cnttop::CntTop, outbtgl::OutBTgl,
    wait::Wait,
};
use nrf_pac as pac;
use pac::gpio::vals;
use panic_halt as _;
use riscv::{asm, delay::McycleDelay};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Use LED 0
    const LED0_PIN: usize = 9;

    let port2 = pac::P2_S;
    port2.pin_cnf(LED0_PIN).write(|w| {
        w.set_dir(vals::Dir::Output);
        w.set_input(vals::Input::Disconnect);
        w.set_pull(vals::Pull::Disabled);
        w.set_drive0(vals::Drive::S);
        w.set_drive1(vals::Drive::S);
        w.set_sense(vals::Sense::Disabled);
    });

    // Manually configure P2_07 to be used by the VPR core,
    // no embassy API (yet).
    const LED2_PIN: usize = 7;

    pac::P2_S.outset().write(|w| w.set_pin(LED2_PIN, true));

    pac::P2_S.pin_cnf(LED2_PIN).write(|w| {
        w.set_ctrlsel(vals::Ctrlsel::Vpr);
        w.set_dir(vals::Dir::Output);
        w.set_input(vals::Input::Disconnect);
        w.set_pull(vals::Pull::Disabled);
        w.set_drive0(vals::Drive::S);
        w.set_drive1(vals::Drive::S);
        w.set_sense(vals::Sense::Disabled);
    });

    // Enable the Real-Time peripheral
    let mut ctrl = csr::ctrl::read();
    ctrl.set_nordic_key(0x507D);
    ctrl.set_enable_rt_periph(true);
    unsafe {
        csr::ctrl::write(ctrl);
    }

    // Set GPIO 2_07 low (enable LED)
    //
    // See mapping at https://nrfconnectdocs.nordicsemi.com/ncs/latest/nrf/app_dev/device_guides/coprocessors/rt_peripherals.html#hpf-real-time-peripherals
    let mut out = csr::out::read();
    out.set_pins(1 << LED2_PIN);
    unsafe {
        csr::out::write(out);
    }

    let mut dir = csr::dir::read();
    dir.set_pins(1 << LED2_PIN);
    unsafe {
        csr::dir::write(dir);
    }

    // Setup the counter
    //
    // Goal is a 500 ms wait -> 64_000_000 / 32_000_000

    // Lets load the combined counter, this means that it uses the combined value
    // and we effectively only have cnt1.

    // Reload after counter 0 has expired
    let cnt_mode_0 = CntMode0::from_bits(2);

    // Combine counter 1 with counter 0
    let cnt_mode_1 = CntMode1::from_bits(3);

    // Set reload value and initial value
    let cnt_top = CntTop::from_bits(32_000_000);
    let cnt = Cnt::from_bits(32_000_000);
    unsafe {
        csr::cnttop::write(cnt_top);
        csr::cnt::write(cnt);
        csr::cntmode1::write(cnt_mode_1);

        // Have this last, because this starts?
        csr::cntmode0::write(cnt_mode_0);
    }

    // TODO: Remove, controlled above...
    //port2.outclr().write(|w| w.set_pin(7, true));

    // Mark that we got here
    port2.outset().write(|w| w.set_pin(LED0_PIN, true));

    /*
    // 32 MHz seems to be the correct frequency for the RISCV core,
    // but it's not documented in the datasheet.
    const TICKS_PER_SECOND: u32 = 32_000_000;

    // This gets disabled when the realtime peripherals are enabled, I think

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
    */

    let out_b_tgl = OutBTgl::from_bits(1 << LED2_PIN);

    loop {
        // This will wait until the counter has expired.
        unsafe {
            csr::outbtgl::write(out_b_tgl);
        }
    }
}
