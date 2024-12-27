#[repr(C)]
#[doc = "Register interface for the medium voltage regulator"]
#[doc(alias = "VREGM")]
pub struct Vregm {
    enable: Enable,
}
impl Vregm {
    #[doc = "0x00 - Enable register for VREGM"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
}
#[doc = "ENABLE (rw) register accessor: Enable register for VREGM\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable register for VREGM"]
pub mod enable;
