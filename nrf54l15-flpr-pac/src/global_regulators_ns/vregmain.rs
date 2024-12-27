#[repr(C)]
#[doc = "Register interface for main voltage regulator."]
#[doc(alias = "VREGMAIN")]
pub struct Vregmain {
    dcdcen: Dcdcen,
    inductordet: Inductordet,
}
impl Vregmain {
    #[doc = "0x00 - Enable DC/DC converter for better power efficiency"]
    #[inline(always)]
    pub const fn dcdcen(&self) -> &Dcdcen {
        &self.dcdcen
    }
    #[doc = "0x04 - VREGMAIN inductor detection"]
    #[inline(always)]
    pub const fn inductordet(&self) -> &Inductordet {
        &self.inductordet
    }
}
#[doc = "DCDCEN (rw) register accessor: Enable DC/DC converter for better power efficiency\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdcen`]
module"]
#[doc(alias = "DCDCEN")]
pub type Dcdcen = crate::Reg<dcdcen::DcdcenSpec>;
#[doc = "Enable DC/DC converter for better power efficiency"]
pub mod dcdcen;
#[doc = "INDUCTORDET (r) register accessor: VREGMAIN inductor detection\n\nYou can [`read`](crate::Reg::read) this register and get [`inductordet::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inductordet`]
module"]
#[doc(alias = "INDUCTORDET")]
pub type Inductordet = crate::Reg<inductordet::InductordetSpec>;
#[doc = "VREGMAIN inductor detection"]
pub mod inductordet;
