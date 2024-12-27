#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "ACTIVESHIELD")]
pub struct Activeshield {
    chen: Chen,
}
impl Activeshield {
    #[doc = "0x00 - Active shield detector channel enable register."]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
}
#[doc = "CHEN (rw) register accessor: Active shield detector channel enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "Active shield detector channel enable register."]
pub mod chen;
