#[repr(C)]
#[doc = "Peripheral tasks."]
#[doc(alias = "RX")]
pub struct Rx {
    start: Start,
    stop: Stop,
    enablematch: [Enablematch; 4],
    disablematch: [Disablematch; 4],
}
impl Rx {
    #[doc = "0x00 - Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA."]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x04 - Stops operation using easyDMA. This does not trigger an END event."]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
    #[doc = "0x08..0x18 - Description collection: Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register."]
    #[inline(always)]
    pub const fn enablematch(&self, n: usize) -> &Enablematch {
        &self.enablematch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - Description collection: Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register."]
    #[inline(always)]
    pub fn enablematch_iter(&self) -> impl Iterator<Item = &Enablematch> {
        self.enablematch.iter()
    }
    #[doc = "0x18..0x28 - Description collection: Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register."]
    #[inline(always)]
    pub const fn disablematch(&self, n: usize) -> &Disablematch {
        &self.disablematch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x28 - Description collection: Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register."]
    #[inline(always)]
    pub fn disablematch_iter(&self) -> impl Iterator<Item = &Disablematch> {
        self.disablematch.iter()
    }
}
#[doc = "START (w) register accessor: Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Starts operation using easyDMA to load the values. See peripheral description for operation using easyDMA."]
pub mod start;
#[doc = "STOP (w) register accessor: Stops operation using easyDMA. This does not trigger an END event.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "Stops operation using easyDMA. This does not trigger an END event."]
pub mod stop;
#[doc = "ENABLEMATCH (w) register accessor: Description collection: Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enablematch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enablematch`]
module"]
#[doc(alias = "ENABLEMATCH")]
pub type Enablematch = crate::Reg<enablematch::EnablematchSpec>;
#[doc = "Description collection: Enables the MATCH\\[n\\]
event by setting the ENABLE\\[n\\]
bit in the CONFIG register."]
pub mod enablematch;
#[doc = "DISABLEMATCH (w) register accessor: Description collection: Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disablematch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disablematch`]
module"]
#[doc(alias = "DISABLEMATCH")]
pub type Disablematch = crate::Reg<disablematch::DisablematchSpec>;
#[doc = "Description collection: Disables the MATCH\\[n\\]
event by clearing the ENABLE\\[n\\]
bit in the CONFIG register."]
pub mod disablematch;
