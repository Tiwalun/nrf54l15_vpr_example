#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PERIPH")]
pub struct Periph {
    perm: Perm,
}
impl Periph {
    #[doc = "0x00 - Description cluster: Get and set the applicable access permissions for the peripheral slave index n"]
    #[inline(always)]
    pub const fn perm(&self) -> &Perm {
        &self.perm
    }
}
#[doc = "PERM (rw) register accessor: Description cluster: Get and set the applicable access permissions for the peripheral slave index n\n\nYou can [`read`](crate::Reg::read) this register and get [`perm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perm`]
module"]
#[doc(alias = "PERM")]
pub type Perm = crate::Reg<perm::PermSpec>;
#[doc = "Description cluster: Get and set the applicable access permissions for the peripheral slave index n"]
pub mod perm;
