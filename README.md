# Rust on nRF54L15 VPR core
This is an example describing how to run Rust code on the
VPR core of the nRF54L15 microcontroller.

The code assumes use of the nRF54L15 DK.

The application core FW is used to start the VPR core, and
it turns on two leds (LED0 and LED2) for debugging purposes.

The VPR core will turn on LED1.

## Interrupts on the FLPR core
The FLPR has a CLIC instead of a PLIC/CLINT. `flpr_core::clic` integrates it
with `riscv-rt`, so interrupt and exception handlers use the standard
attributes:

```rust
use flpr_core::clic::{self, Interrupt, Priority};

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
`mcause` and uses that index in the `mtvt` vector table. The trap handler in
`flpr_core::clic::rt` takes care of this offset.

The application core can raise interrupts on the FLPR by writing
`VPR00.TASKS_TRIGGER[n]` (only `n = 16..=22` are implemented). This raises
`Interrupt::VPRCLIC_<n>` on the FLPR, which must clear the pending task in CSR
`0x7e0`. See the module docs in `flpr-core/src/clic.rs` and
`flpr-core/src/main.rs` for details, and run with `DEFMT_LOG=info` to see the
app core print which interrupt each task raised.
