#[repr(C)]
#[doc = "Peripheral tasks."]
#[doc(alias = "TX")]
pub struct Tx {
    start: Start,
    stop: Stop,
}
impl Tx {
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
