#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "ERASE")]
pub struct Erase {
    eraseall: Eraseall,
}
impl Erase {
    #[doc = "0x00 - Register for erasing whole RRAM main block, that includes the SICR and the UICR"]
    #[inline(always)]
    pub const fn eraseall(&self) -> &Eraseall {
        &self.eraseall
    }
}
#[doc = "ERASEALL (rw) register accessor: Register for erasing whole RRAM main block, that includes the SICR and the UICR\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseall`]
module"]
#[doc(alias = "ERASEALL")]
pub type Eraseall = crate::Reg<eraseall::EraseallSpec>;
#[doc = "Register for erasing whole RRAM main block, that includes the SICR and the UICR"]
pub mod eraseall;
