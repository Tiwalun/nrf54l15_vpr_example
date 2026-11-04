# Rust on nRF54L15 VPR core
This is an example describing how to run Rust code on the
VPR core of the nRF54L15 microcontroller.

The code assumes use of the nRF54L15 DK.

The application core FW is used to start the VPR core, and
it turns on two leds (LED0 and LED2) for debugging purposes.

The VPR core will turn on LED1.

## Interrupts on the FLPR core
The FLPR has a CLIC instead of a PLIC/CLINT. The `nrf-vpr-clic` crate
integrates it with `riscv-rt`, so interrupt and exception handlers use the
standard attributes. Other FLPR firmware can use it by adding it as a
dependency; see the crate docs in `nrf-vpr-clic/src/lib.rs` for the
requirements.

```rust
use nrf_vpr_clic::{self as clic, Interrupt, Priority};

#[riscv_rt::core_interrupt(Interrupt::VPRCLIC_16)]
fn vevif_16() { /* ... */ }

// in main:
unsafe {
    clic::set_priority(Interrupt::VPRCLIC_16, Priority::P3);
    clic::enable(Interrupt::VPRCLIC_16);
    riscv::interrupt::enable();
}
```

`Interrupt` uses the Nordic / PAC interrupt numbers, which are also the
`clicint` register indices. The CLIC reports interrupt `n` as ID `16 + n` in
`mcause` and uses that index in the `mtvt` vector table; the crate takes care
of this offset. Interrupt handlers are called directly by the hardware, which
saves the caller-saved registers itself (see `nrf-vpr-clic/src/rt.rs`).

The application core can raise interrupts on the FLPR by writing
`VPR00.TASKS_TRIGGER[n]` (only `n = 16..=22` are implemented). This raises
`Interrupt::VPRCLIC_<n>` on the FLPR, which must clear the pending task in CSR
`0x7e0`. See the docs in `nrf-vpr-clic/src/lib.rs` and
`flpr-core/src/main.rs` for details, and run with `DEFMT_LOG=info` to see the
app core print which interrupt each task raised.
