#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "RX")]
pub struct Rx {
    end: End,
    ready: Ready,
    buserror: Buserror,
    match_: [Match; 4],
}
impl Rx {
    #[doc = "0x00 - Generated after all MAXCNT bytes have been transferred"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
    #[doc = "0x04 - Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x08 - An error occured during the bus transfer."]
    #[inline(always)]
    pub const fn buserror(&self) -> &Buserror {
        &self.buserror
    }
    #[doc = "0x0c..0x1c - Description collection: Pattern match is detected on the DMA data bus."]
    #[inline(always)]
    pub const fn match_(&self, n: usize) -> &Match {
        &self.match_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - Description collection: Pattern match is detected on the DMA data bus."]
    #[inline(always)]
    pub fn match__iter(&self) -> impl Iterator<Item = &Match> {
        self.match_.iter()
    }
}
#[doc = "END (rw) register accessor: Generated after all MAXCNT bytes have been transferred\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`]
module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "Generated after all MAXCNT bytes have been transferred"]
pub mod end;
#[doc = "READY (rw) register accessor: Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence.\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ready`]
module"]
#[doc(alias = "READY")]
pub type Ready = crate::Reg<ready::ReadySpec>;
#[doc = "Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
pub mod ready;
#[doc = "BUSERROR (rw) register accessor: An error occured during the bus transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserror`]
module"]
#[doc(alias = "BUSERROR")]
pub type Buserror = crate::Reg<buserror::BuserrorSpec>;
#[doc = "An error occured during the bus transfer."]
pub mod buserror;
#[doc = "MATCH (rw) register accessor: Description collection: Pattern match is detected on the DMA data bus.\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`]
module"]
#[doc(alias = "MATCH")]
pub type Match = crate::Reg<match_::MatchSpec>;
#[doc = "Description collection: Pattern match is detected on the DMA data bus."]
pub mod match_;
