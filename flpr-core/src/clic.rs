//! CLIC (Core-Local Interrupt Controller) support for the FLPR, integrated with `riscv-rt`.
//!
//! The FLPR has no PLIC/CLINT, only a CLIC (at `0xF000_0000` as seen from the core) that runs in
//! CLIC mode with hardware vectoring. `riscv-rt` does not know about the CLIC, so this module plugs
//! the CLIC into the standard `riscv-rt` hooks:
//!
//! * [`Interrupt`] implements [`riscv::CoreInterruptNumber`] (using [`riscv::pac_enum`]), so
//!   handlers are defined with the standard [`riscv_rt::core_interrupt`] attribute. Interrupts
//!   without a handler go to `DefaultHandler`, as usual.
//! * Exceptions go to the standard [`riscv_rt::exception`] handlers / `ExceptionHandler`.
//! * `_setup_interrupts` (the `custom-setup-interrupts` hook of `riscv-rt`) switches `mtvec` to CLIC
//!   mode and installs the vector table in `mtvt`, before `main` runs.
//! * Interrupts are globally enabled with the usual [`riscv::interrupt::enable`], and individual
//!   interrupts with [`enable`] / [`disable`] in this module.
//!
//! See [`rt`] for how the trap handling works.
//!
//! # Interrupt numbering
//!
//! Measured on the nRF54L15:
//!
//! ```text
//!  Nordic / PAC IRQn   clicint index   mcause code / mtvt index
//!  ─────────────────   ─────────────   ────────────────────────
//!  VPRCLIC_16 = 16     clicint[16]     32
//!  TIMER00    = 85     clicint[85]     101
//!  n                   clicint[n]      ID_OFFSET + n
//! ```
//!
//! The [`Interrupt`] enum uses the Nordic numbers. Only `rt` and [`current`] deal with the offset.
//! CLIC interrupt IDs `0..16` (the standard RISC-V interrupts, e.g. machine timer) are not wired up
//! on this core; they have no `clicint` register.
//!
//! # Differences to the standard (non-CLIC) RISC-V interrupt handling
//!
//! * `mie` / `mip` are ignored in CLIC mode, so [`riscv::interrupt::enable_interrupt`] and friends
//!   do nothing; use [`enable`] / [`disable`] instead.
//! * `mcause` contains extra fields in CLIC mode (`mpil`, `mpie`, `mpp`, `minhv`), so
//!   `riscv::register::mcause::read().code()` is wrong. Use [`current`] inside interrupt handlers.

use nrf_pac::clic::regs::Clicint;
use nrf_pac::clic::vals::{Mode, Trig};
use riscv::InterruptNumber;

mod interrupt;
pub mod rt;

pub use interrupt::Interrupt;

/// Offset between the Nordic interrupt number and the CLIC interrupt ID used in `mcause` and `mtvt`.
pub const ID_OFFSET: usize = 16;

/// Mask for the exception code / interrupt ID in `mcause` in CLIC mode.
const MCAUSE_CODE_MASK: usize = 0xfff;

/// Interrupt priority.
///
/// The FLPR CLIC implements 2 priority bits, and `cliccfg.nlbits` is 8, so this is the interrupt
/// *level*: when several interrupts are pending, the one with the highest level is taken first.
/// [`P3`](Self::P3) is the highest.
///
/// Interrupts do not preempt each other: handlers run with interrupts globally disabled.
#[riscv::pac_enum(unsafe PriorityNumber)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Priority {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
}

impl Priority {
    /// Value for `clicint.priority` (the implemented bits are the most significant ones, the rest
    /// read as 1).
    const fn to_clicint(self) -> nrf_pac::clic::vals::Priority {
        nrf_pac::clic::vals::Priority::from_bits(((self as u8) << 6) | 0x3f)
    }

    const fn from_clicint(p: nrf_pac::clic::vals::Priority) -> Self {
        match p.to_bits() >> 6 {
            0 => Self::P0,
            1 => Self::P1,
            2 => Self::P2,
            _ => Self::P3,
        }
    }
}

fn clicint(irq: Interrupt) -> nrf_pac::common::Reg<Clicint, nrf_pac::common::RW> {
    nrf_pac::VPRCLIC_NS.clic().clicint(irq.number())
}

/// Enables `irq`.
///
/// This also configures it as an edge-triggered, machine-mode interrupt (the only configuration
/// the FLPR CLIC supports). The priority is not changed.
///
/// # Safety
///
/// Like `cortex_m::peripheral::NVIC::unmask`, this can break mask-based critical sections.
#[inline]
pub unsafe fn enable(irq: Interrupt) {
    clicint(irq).modify(|c| {
        c.set_trig(Trig::EdgeTriggered);
        c.set_mode(Mode::MachineMode);
        c.set_ie(true);
    });
}

/// Disables `irq`.
#[inline]
pub fn disable(irq: Interrupt) {
    clicint(irq).modify(|c| c.set_ie(false));
}

/// Returns `true` if `irq` is enabled.
#[inline]
pub fn is_enabled(irq: Interrupt) -> bool {
    clicint(irq).read().ie()
}

/// Sets `irq` pending, i.e. triggers it from software.
#[inline]
pub fn pend(irq: Interrupt) {
    clicint(irq).modify(|c| c.set_ip(true));
}

/// Clears the pending state of `irq`.
#[inline]
pub fn unpend(irq: Interrupt) {
    clicint(irq).modify(|c| c.set_ip(false));
}

/// Returns `true` if `irq` is pending.
#[inline]
pub fn is_pending(irq: Interrupt) -> bool {
    clicint(irq).read().ip()
}

/// Returns the priority of `irq`.
#[inline]
pub fn priority(irq: Interrupt) -> Priority {
    Priority::from_clicint(clicint(irq).read().priority())
}

/// Sets the priority of `irq`.
///
/// # Safety
///
/// Changing priorities can break priority-based critical sections.
#[inline]
pub unsafe fn set_priority(irq: Interrupt, priority: Priority) {
    clicint(irq).modify(|c| c.set_priority(priority.to_clicint()));
}

/// Returns the interrupt that is currently being handled, based on `mcause`.
///
/// Returns `None` outside of an interrupt handler, or for interrupt IDs that are not part of
/// [`Interrupt`]. Useful in `DefaultHandler`.
#[inline]
pub fn current() -> Option<Interrupt> {
    let mcause = riscv::register::mcause::read();
    if !mcause.is_interrupt() {
        return None;
    }
    let id = mcause.bits() & MCAUSE_CODE_MASK;
    Interrupt::from_number(id.checked_sub(ID_OFFSET)?).ok()
}
