//! FLPR interrupt sources and the CLIC vector table.
//!
//! The list is taken from the `Interrupt` enum of `nrf-pac` (`nrf54l15-flpr`), so the names and
//! numbers are the same as in the PAC and in Nordic's documentation. Every entry needs a matching
//! `PROVIDE(<name> = DefaultHandler);` line in `device.x`; if one is missing, linking
//! fails with an undefined symbol.

use super::ID_OFFSET;
use super::rt::{NUM_VECTORS, VectorTable};

/// Defines [`Interrupt`], implements the `riscv` interrupt traits for it, and builds the vector
/// table that points every CLIC interrupt ID at the handler of the same name.
macro_rules! interrupts {
    ($($name:ident = $irqn:literal,)*) => {
        /// Interrupt sources of the FLPR CLIC.
        ///
        /// The discriminant is the Nordic / PAC interrupt number (`IRQn`), which is also the index
        /// of the interrupt's `clicint` register. The CLIC reports interrupts in `mcause`, and looks
        /// them up in `mtvt`, using [`ID_OFFSET`]` + IRQn`.
        ///
        /// Use the variants with [`riscv_rt::core_interrupt`] to install a handler:
        ///
        /// ```ignore
        /// #[riscv_rt::core_interrupt(nrf_vpr_clic::Interrupt::VPRCLIC_16)]
        /// fn vevif_16() {
        ///     // ...
        /// }
        /// ```
        ///
        /// Of the `VPRCLIC_<n>` (VEVIF) interrupts, only `n = 16..=22` are triggered by
        /// `VPR00.TASKS_TRIGGER[n]` on the nRF54L15.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[allow(non_camel_case_types)]
        #[repr(u16)]
        pub enum Interrupt {
            $($name = $irqn,)*
        }

        /// Highest interrupt number.
        #[allow(unused_comparisons)]
        const MAX_IRQN: usize = {
            let mut max = 0;
            $(if $irqn > max { max = $irqn; })*
            max
        };

        /// Bitmap of the valid interrupt numbers. Much smaller than a `match` on all variants,
        /// which compiles to a jump table with one entry per number up to `MAX_IRQN`.
        const VALID_IRQN: [u32; MAX_IRQN / 32 + 1] = {
            let mut valid = [0; MAX_IRQN / 32 + 1];
            $(valid[$irqn / 32] |= 1 << ($irqn % 32);)*
            valid
        };

        // Implemented by hand instead of with `riscv::pac_enum`: with the `rt` feature, `pac_enum`
        // also generates a software dispatch table (`__CORE_INTERRUPTS`), which is not needed
        // because the hardware jumps straight to the handlers through `VECTOR_TABLE`.
        //
        // SAFETY: the discriminants are distinct and fixed, and `MAX_INTERRUPT_NUMBER` is the
        // largest of them.
        unsafe impl riscv::InterruptNumber for Interrupt {
            const MAX_INTERRUPT_NUMBER: usize = MAX_IRQN;

            #[inline]
            fn number(self) -> usize {
                self as usize
            }

            #[inline]
            fn from_number(value: usize) -> riscv::result::Result<Self> {
                if value <= MAX_IRQN && VALID_IRQN[value / 32] & (1 << (value % 32)) != 0 {
                    // SAFETY: `value` is the discriminant of a variant of this `repr(u16)` enum.
                    Ok(unsafe { core::mem::transmute::<u16, Self>(value as u16) })
                } else {
                    Err(riscv::result::Error::InvalidVariant(value))
                }
            }
        }

        // SAFETY: every variant is a CLIC interrupt of this core. Note that `mcause` reports it as
        // `ID_OFFSET + number()`.
        unsafe impl riscv::CoreInterruptNumber for Interrupt {}

        #[allow(non_snake_case)]
        unsafe extern "C" {
            // Interrupt handlers: `#[riscv_rt::core_interrupt]` functions, or `DefaultHandler`
            // (see `device.x`).
            $(fn $name();)*
            fn DefaultHandler();
        }

        /// The CLIC vector table (`mtvt`): entry `ID_OFFSET + IRQn` is the handler of `IRQn`.
        ///
        /// IDs without an [`Interrupt`] (including `0..ID_OFFSET`, which have no `clicint`
        /// register on this core) point to `DefaultHandler`.
        pub(super) static VECTOR_TABLE: VectorTable = {
            let mut table = [DefaultHandler as unsafe extern "C" fn(); NUM_VECTORS];
            $(table[ID_OFFSET + $irqn] = $name;)*
            VectorTable(table)
        };
    };
}

interrupts! {
    VPRCLIC_0 = 0,
    VPRCLIC_1 = 1,
    VPRCLIC_2 = 2,
    VPRCLIC_3 = 3,
    VPRCLIC_4 = 4,
    VPRCLIC_5 = 5,
    VPRCLIC_6 = 6,
    VPRCLIC_7 = 7,
    VPRCLIC_8 = 8,
    VPRCLIC_9 = 9,
    VPRCLIC_10 = 10,
    VPRCLIC_11 = 11,
    VPRCLIC_12 = 12,
    VPRCLIC_13 = 13,
    VPRCLIC_14 = 14,
    VPRCLIC_15 = 15,
    VPRCLIC_16 = 16,
    VPRCLIC_17 = 17,
    VPRCLIC_18 = 18,
    VPRCLIC_19 = 19,
    VPRCLIC_20 = 20,
    VPRCLIC_21 = 21,
    VPRCLIC_22 = 22,
    VPRCLIC_23 = 23,
    VPRCLIC_24 = 24,
    VPRCLIC_25 = 25,
    VPRCLIC_26 = 26,
    VPRCLIC_27 = 27,
    VPRCLIC_28 = 28,
    VPRCLIC_29 = 29,
    VPRCLIC_30 = 30,
    VPRCLIC_31 = 31,
    SPU00 = 64,
    MPC00 = 65,
    AAR00_CCM00 = 70,
    ECB00 = 71,
    CRACEN = 72,
    SERIAL00 = 74,
    RRAMC = 75,
    VPR00 = 76,
    CTRLAP = 82,
    TIMER00 = 85,
    SPU10 = 128,
    TIMER10 = 133,
    EGU10 = 135,
    RADIO_0 = 138,
    RADIO_1 = 139,
    SPU20 = 192,
    SERIAL20 = 198,
    SERIAL21 = 199,
    SERIAL22 = 200,
    EGU20 = 201,
    TIMER20 = 202,
    TIMER21 = 203,
    TIMER22 = 204,
    TIMER23 = 205,
    TIMER24 = 206,
    PDM20 = 208,
    PDM21 = 209,
    PWM20 = 210,
    PWM21 = 211,
    PWM22 = 212,
    SAADC = 213,
    NFCT = 214,
    TEMP = 215,
    GPIOTE20_0 = 218,
    GPIOTE20_1 = 219,
    TAMPC = 220,
    I2S20 = 221,
    QDEC20 = 224,
    QDEC21 = 225,
    GRTC_0 = 226,
    GRTC_1 = 227,
    GRTC_2 = 228,
    GRTC_3 = 229,
    SPU30 = 256,
    SERIAL30 = 260,
    COMP_LPCOMP = 262,
    WDT30 = 264,
    WDT31 = 265,
    GPIOTE30_0 = 268,
    GPIOTE30_1 = 269,
}
