#[repr(C)]
#[doc = "Subscribe configuration for tasks"]
#[doc(alias = "RX")]
pub struct Rx {
    start: Start,
    stop: Stop,
    enablematch: [Enablematch; 4],
    disablematch: [Disablematch; 4],
}
impl Rx {
    #[doc = "0x00 - Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x04 - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
    #[doc = "0x08..0x18 - Description collection: Subscribe configuration for task ENABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub const fn enablematch(&self, n: usize) -> &Enablematch {
        &self.enablematch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - Description collection: Subscribe configuration for task ENABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn enablematch_iter(&self) -> impl Iterator<Item = &Enablematch> {
        self.enablematch.iter()
    }
    #[doc = "0x18..0x28 - Description collection: Subscribe configuration for task DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub const fn disablematch(&self, n: usize) -> &Disablematch {
        &self.disablematch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x28 - Description collection: Subscribe configuration for task DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn disablematch_iter(&self) -> impl Iterator<Item = &Disablematch> {
        self.disablematch.iter()
    }
}
#[doc = "START (rw) register accessor: Subscribe configuration for task START\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Subscribe configuration for task START"]
pub mod start;
#[doc = "STOP (rw) register accessor: Subscribe configuration for task STOP\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "Subscribe configuration for task STOP"]
pub mod stop;
#[doc = "ENABLEMATCH (rw) register accessor: Description collection: Subscribe configuration for task ENABLEMATCH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`enablematch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enablematch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enablematch`]
module"]
#[doc(alias = "ENABLEMATCH")]
pub type Enablematch = crate::Reg<enablematch::EnablematchSpec>;
#[doc = "Description collection: Subscribe configuration for task ENABLEMATCH\\[n\\]"]
pub mod enablematch;
#[doc = "DISABLEMATCH (rw) register accessor: Description collection: Subscribe configuration for task DISABLEMATCH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`disablematch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disablematch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disablematch`]
module"]
#[doc(alias = "DISABLEMATCH")]
pub type Disablematch = crate::Reg<disablematch::DisablematchSpec>;
#[doc = "Description collection: Subscribe configuration for task DISABLEMATCH\\[n\\]"]
pub mod disablematch;
