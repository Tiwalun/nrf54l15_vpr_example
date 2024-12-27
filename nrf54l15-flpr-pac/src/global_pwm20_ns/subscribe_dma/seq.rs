#[repr(C)]
#[doc = "Subscribe configuration for tasks"]
#[doc(alias = "SEQ")]
pub struct Seq {
    start: Start,
    stop: Stop,
}
impl Seq {
    #[doc = "0x00 - Description cluster: Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x04 - Description cluster: Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
}
#[doc = "START (rw) register accessor: Description cluster: Subscribe configuration for task START\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Description cluster: Subscribe configuration for task START"]
pub mod start;
#[doc = "STOP (rw) register accessor: Description cluster: Subscribe configuration for task STOP\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "Description cluster: Subscribe configuration for task STOP"]
pub mod stop;
