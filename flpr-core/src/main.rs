//! FLPR interrupt demo.
//!
//! The application core raises interrupts on the FLPR by writing to the
//! `VPR00.TASKS_TRIGGER[n]` registers ("VEVIF" = VPR event interface). The
//! FLPR handles them with the standard `riscv-rt` attributes; the CLIC
//! integration lives in [`flpr_core::clic`].
//!
//! ## Interrupt mapping
//!
//! ```text
//!  app core                          FLPR
//!  VPR00.TASKS_TRIGGER[n]  ───────►  VEVIF task n (bit n of CSR NORDIC_TASKS, 0x7e0)
//!    n = 16..=22 (0..=15 do nothing)        │
//!                                           ▼
//!                                    Interrupt::VPRCLIC_<n> (clicint[n])
//!                                    mcause = 0x8000_0000 | (16 + n), plus CLIC status bits
//! ```
//!
//! * The task stays pending in `NORDIC_TASKS` until the FLPR clears it
//!   (`csrc 0x7e0`), otherwise the interrupt immediately re-triggers.
//! * `NORDIC_TASKS` & co. trap as illegal instruction until
//!   `NORDIC_CTRL.enable_rt_periph` is set (using the unlock key).
//!
//! This firmware:
//!  1. unlocks the Nordic CSRs,
//!  2. enables the VEVIF interrupts `VPRCLIC_16..=22`,
//!  3. counts, per interrupt (Nordic `IRQn`), how often it was taken, in the
//!     shared memory block, so that the application core can print the result.
//!
//! `VPRCLIC_16` and `VPRCLIC_17` have their own handlers, the other interrupts
//! end up in `DefaultHandler`.
//!
//! See `app-core/src/main.rs` for the other half.
#![no_std]
#![no_main]

use core::arch::asm;

use flpr_core::clic::{self, Interrupt, Priority};
use flpr_core::csr;
use panic_halt as _;
use riscv::InterruptNumber;
use riscv::asm;
use riscv::register::{mcause, mepc};
use riscv_rt::{TrapFrame, core_interrupt, entry};

/// Number of interrupts (Nordic `IRQn`) we count.
const NUM_IRQS: usize = 96;

/// Layout of the shared memory block (must match `app-core`).
#[repr(C)]
pub struct Shared {
    /// Set to `READY_MAGIC` by the FLPR once the interrupts are set up.
    pub ready: u32,
    /// `mcause` of the most recent trap.
    pub last_mcause: u32,
    /// Number of exceptions (as opposed to interrupts) seen.
    pub exceptions: u32,
    pub _reserved: [u32; 3],
    /// Number of times interrupt `IRQn = i` was taken.
    pub irq_counts: [u32; NUM_IRQS],
    /// Log of the first traps: number of entries, then (mcause, mepc) pairs.
    pub log_len: u32,
    pub log: [[u32; 2]; 8],
}

const READY_MAGIC: u32 = 0xF1F1_0001;

unsafe extern "C" {
    // Provided by the linker: start of the shared memory buffer.
    static mut __shared_mem: Shared;
}

fn shared() -> *mut Shared {
    &raw mut __shared_mem
}

/// Records a trap in the shared memory block.
fn record_trap(irq: Option<Interrupt>) {
    let shared = shared();
    let mcause = mcause::read().bits() as u32;

    unsafe {
        (&raw mut (*shared).last_mcause).write_volatile(mcause);

        let n = (&raw mut (*shared).log_len).read_volatile() as usize;
        if n < 8 {
            (&raw mut (*shared).log[n]).write_volatile([mcause, mepc::read() as u32]);
            (&raw mut (*shared).log_len).write_volatile(n as u32 + 1);
        }

        let counter = match irq {
            Some(irq) if irq.number() < NUM_IRQS => &raw mut (*shared).irq_counts[irq.number()],
            Some(_) => return,
            None => &raw mut (*shared).exceptions,
        };
        counter.write_volatile(counter.read_volatile() + 1);
    }
}

/// Clears VEVIF task `n` in the `NORDIC_TASKS` CSR (0x7e0).
///
/// Interrupt `VPRCLIC_<n>` stays pending until this is done.
fn clear_vevif_task(n: usize) {
    unsafe { asm!("csrc 0x7e0, {}", in(reg) 1u32 << n) };
}

/// Acknowledges a VEVIF interrupt by clearing its task.
fn ack_vevif(irq: Interrupt) {
    let n = irq.number();
    if n < 32 {
        clear_vevif_task(n);
    }
}

#[core_interrupt(Interrupt::VPRCLIC_16)]
fn vevif_16() {
    clear_vevif_task(16);
    record_trap(Some(Interrupt::VPRCLIC_16));
}

#[core_interrupt(Interrupt::VPRCLIC_17)]
fn vevif_17() {
    clear_vevif_task(17);
    record_trap(Some(Interrupt::VPRCLIC_17));
}

/// Handles all interrupts without a dedicated handler.
#[unsafe(no_mangle)]
extern "C" fn DefaultHandler() {
    let irq = clic::current();
    if let Some(irq) = irq {
        ack_vevif(irq);
    }
    record_trap(irq);
}

/// Handles all exceptions. Returning would re-execute the faulting
/// instruction, so we stop here after recording it.
#[unsafe(no_mangle)]
extern "C" fn ExceptionHandler(_trap_frame: &TrapFrame) -> ! {
    record_trap(None);
    loop {
        asm::wfi();
    }
}

#[entry]
fn main() -> ! {
    // `riscv-rt` has already installed the CLIC trap handling (see
    // `flpr_core::clic::rt`), interrupts are globally disabled.

    // The Nordic-specific CSRs (VEVIF tasks/events at 0x7e0..0x7e6, ...) trap as
    // illegal instructions until the real-time peripheral block is unlocked.
    let mut ctrl = csr::ctrl::read();
    ctrl.set_nordic_key(0x507D);
    ctrl.set_enable_rt_periph(true);
    unsafe { csr::ctrl::write(ctrl) };

    // Enable the VEVIF interrupts that `VPR00.TASKS_TRIGGER[n]` can raise.
    for irq in [
        Interrupt::VPRCLIC_16,
        Interrupt::VPRCLIC_17,
        Interrupt::VPRCLIC_18,
        Interrupt::VPRCLIC_19,
        Interrupt::VPRCLIC_20,
        Interrupt::VPRCLIC_21,
        Interrupt::VPRCLIC_22,
    ] {
        unsafe {
            clic::set_priority(irq, Priority::P3);
            clic::enable(irq);
        }
    }

    unsafe {
        (&raw mut (*shared()).ready).write_volatile(READY_MAGIC);
        riscv::interrupt::enable();
    }

    loop {
        // Sleep until an interrupt wakes us up.
        asm::wfi();
    }
}
