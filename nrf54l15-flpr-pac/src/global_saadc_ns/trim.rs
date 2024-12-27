#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "TRIM")]
pub struct Trim {
    lincalcoeff: [Lincalcoeff; 6],
}
impl Trim {
    #[doc = "0x00..0x18 - Description collection: Linearity calibration coefficient"]
    #[inline(always)]
    pub const fn lincalcoeff(&self, n: usize) -> &Lincalcoeff {
        &self.lincalcoeff[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - Description collection: Linearity calibration coefficient"]
    #[inline(always)]
    pub fn lincalcoeff_iter(&self) -> impl Iterator<Item = &Lincalcoeff> {
        self.lincalcoeff.iter()
    }
}
#[doc = "LINCALCOEFF (rw) register accessor: Description collection: Linearity calibration coefficient\n\nYou can [`read`](crate::Reg::read) this register and get [`lincalcoeff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincalcoeff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lincalcoeff`]
module"]
#[doc(alias = "LINCALCOEFF")]
pub type Lincalcoeff = crate::Reg<lincalcoeff::LincalcoeffSpec>;
#[doc = "Description collection: Linearity calibration coefficient"]
pub mod lincalcoeff;
