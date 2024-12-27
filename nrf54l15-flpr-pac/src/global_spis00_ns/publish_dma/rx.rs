#[repr(C)]
#[doc = "Publish configuration for events"]
#[doc(alias = "RX")]
pub struct Rx {
    end: End,
    ready: Ready,
    buserror: Buserror,
    match_: [Match; 4],
}
impl Rx {
    #[doc = "0x00 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
    #[doc = "0x04 - Publish configuration for event READY"]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x08 - Publish configuration for event BUSERROR"]
    #[inline(always)]
    pub const fn buserror(&self) -> &Buserror {
        &self.buserror
    }
    #[doc = "0x0c..0x1c - Description collection: Publish configuration for event MATCH\\[n\\]"]
    #[inline(always)]
    pub const fn match_(&self, n: usize) -> &Match {
        &self.match_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - Description collection: Publish configuration for event MATCH\\[n\\]"]
    #[inline(always)]
    pub fn match__iter(&self) -> impl Iterator<Item = &Match> {
        self.match_.iter()
    }
}
#[doc = "END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`]
module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "Publish configuration for event END"]
pub mod end;
#[doc = "READY (rw) register accessor: Publish configuration for event READY\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ready`]
module"]
#[doc(alias = "READY")]
pub type Ready = crate::Reg<ready::ReadySpec>;
#[doc = "Publish configuration for event READY"]
pub mod ready;
#[doc = "BUSERROR (rw) register accessor: Publish configuration for event BUSERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`buserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserror`]
module"]
#[doc(alias = "BUSERROR")]
pub type Buserror = crate::Reg<buserror::BuserrorSpec>;
#[doc = "Publish configuration for event BUSERROR"]
pub mod buserror;
#[doc = "MATCH (rw) register accessor: Description collection: Publish configuration for event MATCH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`]
module"]
#[doc(alias = "MATCH")]
pub type Match = crate::Reg<match_::MatchSpec>;
#[doc = "Description collection: Publish configuration for event MATCH\\[n\\]"]
pub mod match_;
