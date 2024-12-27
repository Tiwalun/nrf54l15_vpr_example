#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "OVERFLOW")]
pub struct Overflow {
    send: Send,
}
impl Overflow {
    #[doc = "0x00 - The task overflow for SEND tasks using SUBSCRIBE_SEND. Write 0 to clear."]
    #[inline(always)]
    pub const fn send(&self) -> &Send {
        &self.send
    }
}
#[doc = "SEND (rw) register accessor: The task overflow for SEND tasks using SUBSCRIBE_SEND. Write 0 to clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`send::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`send::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@send`]
module"]
#[doc(alias = "SEND")]
pub type Send = crate::Reg<send::SendSpec>;
#[doc = "The task overflow for SEND tasks using SUBSCRIBE_SEND. Write 0 to clear."]
pub mod send;
