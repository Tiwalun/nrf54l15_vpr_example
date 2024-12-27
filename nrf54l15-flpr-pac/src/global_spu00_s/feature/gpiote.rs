#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "GPIOTE")]
pub struct Gpiote {
    ch: [Ch; 8],
    interrupt: [Interrupt; 8],
}
impl Gpiote {
    #[doc = "0x00..0x20 - Description collection: Configuration of features for channel o of GPIOTE\\[n\\]"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: Configuration of features for channel o of GPIOTE\\[n\\]"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x20..0x40 - Description collection: Configuration of features for interrupt o of GPIOTE\\[n\\]"]
    #[inline(always)]
    pub const fn interrupt(&self, n: usize) -> &Interrupt {
        &self.interrupt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - Description collection: Configuration of features for interrupt o of GPIOTE\\[n\\]"]
    #[inline(always)]
    pub fn interrupt_iter(&self) -> impl Iterator<Item = &Interrupt> {
        self.interrupt.iter()
    }
}
#[doc = "CH (rw) register accessor: Description collection: Configuration of features for channel o of GPIOTE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch`]
module"]
#[doc(alias = "CH")]
pub type Ch = crate::Reg<ch::ChSpec>;
#[doc = "Description collection: Configuration of features for channel o of GPIOTE\\[n\\]"]
pub mod ch;
#[doc = "INTERRUPT (rw) register accessor: Description collection: Configuration of features for interrupt o of GPIOTE\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Description collection: Configuration of features for interrupt o of GPIOTE\\[n\\]"]
pub mod interrupt;
