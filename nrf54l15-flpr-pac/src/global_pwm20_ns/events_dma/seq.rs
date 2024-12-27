#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "SEQ")]
pub struct Seq {
    end: End,
    ready: Ready,
    buserror: Buserror,
}
impl Seq {
    #[doc = "0x00 - Description cluster: Generated after all MAXCNT bytes have been transferred"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
    #[doc = "0x04 - Description cluster: Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x08 - Description cluster: An error occured during the bus transfer."]
    #[inline(always)]
    pub const fn buserror(&self) -> &Buserror {
        &self.buserror
    }
}
#[doc = "END (rw) register accessor: Description cluster: Generated after all MAXCNT bytes have been transferred\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`]
module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "Description cluster: Generated after all MAXCNT bytes have been transferred"]
pub mod end;
#[doc = "READY (rw) register accessor: Description cluster: Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence.\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ready`]
module"]
#[doc(alias = "READY")]
pub type Ready = crate::Reg<ready::ReadySpec>;
#[doc = "Description cluster: Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
pub mod ready;
#[doc = "BUSERROR (rw) register accessor: Description cluster: An error occured during the bus transfer.\n\nYou can [`read`](crate::Reg::read) this register and get [`buserror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buserror`]
module"]
#[doc(alias = "BUSERROR")]
pub type Buserror = crate::Reg<buserror::BuserrorSpec>;
#[doc = "Description cluster: An error occured during the bus transfer."]
pub mod buserror;
