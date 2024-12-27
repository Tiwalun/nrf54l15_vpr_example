#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CRYPTMSTRHW")]
pub struct Cryptmstrhw {
    inclipshwcfg: Inclipshwcfg,
    ba411eaeshwcfg1: Ba411eaeshwcfg1,
    ba411eaeshwcfg2: Ba411eaeshwcfg2,
    ba413hashhwcfg: Ba413hashhwcfg,
    ba418sha3hwcfg: Ba418sha3hwcfg,
    ba419sm4hwcfg: Ba419sm4hwcfg,
    ba424ariahwcfg: Ba424ariahwcfg,
}
impl Cryptmstrhw {
    #[doc = "0x00 - Incuded IPs Hardware configuration"]
    #[inline(always)]
    pub const fn inclipshwcfg(&self) -> &Inclipshwcfg {
        &self.inclipshwcfg
    }
    #[doc = "0x04 - Generic g_AesModesPoss value."]
    #[inline(always)]
    pub const fn ba411eaeshwcfg1(&self) -> &Ba411eaeshwcfg1 {
        &self.ba411eaeshwcfg1
    }
    #[doc = "0x08 - Generic g_CtrSize value."]
    #[inline(always)]
    pub const fn ba411eaeshwcfg2(&self) -> &Ba411eaeshwcfg2 {
        &self.ba411eaeshwcfg2
    }
    #[doc = "0x0c - Generic g_Hash value"]
    #[inline(always)]
    pub const fn ba413hashhwcfg(&self) -> &Ba413hashhwcfg {
        &self.ba413hashhwcfg
    }
    #[doc = "0x10 - Generic g_Sha3CtxtEn value."]
    #[inline(always)]
    pub const fn ba418sha3hwcfg(&self) -> &Ba418sha3hwcfg {
        &self.ba418sha3hwcfg
    }
    #[doc = "0x14 - Generic g_SM4ModesPoss value."]
    #[inline(always)]
    pub const fn ba419sm4hwcfg(&self) -> &Ba419sm4hwcfg {
        &self.ba419sm4hwcfg
    }
    #[doc = "0x18 - Generic g_aria_modePoss value."]
    #[inline(always)]
    pub const fn ba424ariahwcfg(&self) -> &Ba424ariahwcfg {
        &self.ba424ariahwcfg
    }
}
#[doc = "INCLIPSHWCFG (rw) register accessor: Incuded IPs Hardware configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`inclipshwcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inclipshwcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inclipshwcfg`]
module"]
#[doc(alias = "INCLIPSHWCFG")]
pub type Inclipshwcfg = crate::Reg<inclipshwcfg::InclipshwcfgSpec>;
#[doc = "Incuded IPs Hardware configuration"]
pub mod inclipshwcfg;
#[doc = "BA411EAESHWCFG1 (rw) register accessor: Generic g_AesModesPoss value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba411eaeshwcfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba411eaeshwcfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ba411eaeshwcfg1`]
module"]
#[doc(alias = "BA411EAESHWCFG1")]
pub type Ba411eaeshwcfg1 = crate::Reg<ba411eaeshwcfg1::Ba411eaeshwcfg1Spec>;
#[doc = "Generic g_AesModesPoss value."]
pub mod ba411eaeshwcfg1;
#[doc = "BA411EAESHWCFG2 (rw) register accessor: Generic g_CtrSize value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba411eaeshwcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba411eaeshwcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ba411eaeshwcfg2`]
module"]
#[doc(alias = "BA411EAESHWCFG2")]
pub type Ba411eaeshwcfg2 = crate::Reg<ba411eaeshwcfg2::Ba411eaeshwcfg2Spec>;
#[doc = "Generic g_CtrSize value."]
pub mod ba411eaeshwcfg2;
#[doc = "BA413HASHHWCFG (rw) register accessor: Generic g_Hash value\n\nYou can [`read`](crate::Reg::read) this register and get [`ba413hashhwcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba413hashhwcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ba413hashhwcfg`]
module"]
#[doc(alias = "BA413HASHHWCFG")]
pub type Ba413hashhwcfg = crate::Reg<ba413hashhwcfg::Ba413hashhwcfgSpec>;
#[doc = "Generic g_Hash value"]
pub mod ba413hashhwcfg;
#[doc = "BA418SHA3HWCFG (rw) register accessor: Generic g_Sha3CtxtEn value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba418sha3hwcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba418sha3hwcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ba418sha3hwcfg`]
module"]
#[doc(alias = "BA418SHA3HWCFG")]
pub type Ba418sha3hwcfg = crate::Reg<ba418sha3hwcfg::Ba418sha3hwcfgSpec>;
#[doc = "Generic g_Sha3CtxtEn value."]
pub mod ba418sha3hwcfg;
#[doc = "BA419SM4HWCFG (rw) register accessor: Generic g_SM4ModesPoss value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba419sm4hwcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba419sm4hwcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ba419sm4hwcfg`]
module"]
#[doc(alias = "BA419SM4HWCFG")]
pub type Ba419sm4hwcfg = crate::Reg<ba419sm4hwcfg::Ba419sm4hwcfgSpec>;
#[doc = "Generic g_SM4ModesPoss value."]
pub mod ba419sm4hwcfg;
#[doc = "BA424ARIAHWCFG (rw) register accessor: Generic g_aria_modePoss value.\n\nYou can [`read`](crate::Reg::read) this register and get [`ba424ariahwcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ba424ariahwcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ba424ariahwcfg`]
module"]
#[doc(alias = "BA424ARIAHWCFG")]
pub type Ba424ariahwcfg = crate::Reg<ba424ariahwcfg::Ba424ariahwcfgSpec>;
#[doc = "Generic g_aria_modePoss value."]
pub mod ba424ariahwcfg;
