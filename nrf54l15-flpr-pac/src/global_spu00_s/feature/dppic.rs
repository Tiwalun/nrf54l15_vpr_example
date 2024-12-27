#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DPPIC")]
pub struct Dppic {
    ch: [Ch; 24],
    chg: [Chg; 8],
}
impl Dppic {
    #[doc = "0x00..0x60 - Description collection: Configuration of features for channel n of DPPIC"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x60 - Description collection: Configuration of features for channel n of DPPIC"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x60..0x80 - Description collection: Configuration of features for channel group n of DPPIC"]
    #[inline(always)]
    pub const fn chg(&self, n: usize) -> &Chg {
        &self.chg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - Description collection: Configuration of features for channel group n of DPPIC"]
    #[inline(always)]
    pub fn chg_iter(&self) -> impl Iterator<Item = &Chg> {
        self.chg.iter()
    }
}
#[doc = "CH (rw) register accessor: Description collection: Configuration of features for channel n of DPPIC\n\nYou can [`read`](crate::Reg::read) this register and get [`ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch`]
module"]
#[doc(alias = "CH")]
pub type Ch = crate::Reg<ch::ChSpec>;
#[doc = "Description collection: Configuration of features for channel n of DPPIC"]
pub mod ch;
#[doc = "CHG (rw) register accessor: Description collection: Configuration of features for channel group n of DPPIC\n\nYou can [`read`](crate::Reg::read) this register and get [`chg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chg`]
module"]
#[doc(alias = "CHG")]
pub type Chg = crate::Reg<chg::ChgSpec>;
#[doc = "Description collection: Configuration of features for channel group n of DPPIC"]
pub mod chg;
