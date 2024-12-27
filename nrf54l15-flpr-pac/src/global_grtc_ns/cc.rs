#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CC")]
pub struct Cc {
    ccl: Ccl,
    cch: Cch,
    ccadd: Ccadd,
    ccen: Ccen,
}
impl Cc {
    #[doc = "0x00 - Description cluster: The lower 32-bits of Capture/Compare register CC\\[n\\]"]
    #[inline(always)]
    pub const fn ccl(&self) -> &Ccl {
        &self.ccl
    }
    #[doc = "0x04 - Description cluster: The higher 32-bits of Capture/Compare register CC\\[n\\]"]
    #[inline(always)]
    pub const fn cch(&self) -> &Cch {
        &self.cch
    }
    #[doc = "0x08 - Description cluster: Count to add to CC\\[n\\]
when this register is written."]
    #[inline(always)]
    pub const fn ccadd(&self) -> &Ccadd {
        &self.ccadd
    }
    #[doc = "0x0c - Description cluster: Configure Capture/Compare register CC\\[n\\]"]
    #[inline(always)]
    pub const fn ccen(&self) -> &Ccen {
        &self.ccen
    }
}
#[doc = "CCL (rw) register accessor: Description cluster: The lower 32-bits of Capture/Compare register CC\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccl`]
module"]
#[doc(alias = "CCL")]
pub type Ccl = crate::Reg<ccl::CclSpec>;
#[doc = "Description cluster: The lower 32-bits of Capture/Compare register CC\\[n\\]"]
pub mod ccl;
#[doc = "CCH (rw) register accessor: Description cluster: The higher 32-bits of Capture/Compare register CC\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`cch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cch`]
module"]
#[doc(alias = "CCH")]
pub type Cch = crate::Reg<cch::CchSpec>;
#[doc = "Description cluster: The higher 32-bits of Capture/Compare register CC\\[n\\]"]
pub mod cch;
#[doc = "CCADD (rw) register accessor: Description cluster: Count to add to CC\\[n\\]
when this register is written.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccadd`]
module"]
#[doc(alias = "CCADD")]
pub type Ccadd = crate::Reg<ccadd::CcaddSpec>;
#[doc = "Description cluster: Count to add to CC\\[n\\]
when this register is written."]
pub mod ccadd;
#[doc = "CCEN (rw) register accessor: Description cluster: Configure Capture/Compare register CC\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccen`]
module"]
#[doc(alias = "CCEN")]
pub type Ccen = crate::Reg<ccen::CcenSpec>;
#[doc = "Description cluster: Configure Capture/Compare register CC\\[n\\]"]
pub mod ccen;
