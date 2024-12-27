#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "NFCID1")]
pub struct Nfcid1 {
    last: Last,
    secondlast: Secondlast,
    thirdlast: Thirdlast,
}
impl Nfcid1 {
    #[doc = "0x00 - Last NFCID1 part (4, 7 or 10 bytes ID)"]
    #[inline(always)]
    pub const fn last(&self) -> &Last {
        &self.last
    }
    #[doc = "0x04 - Second last NFCID1 part (7 or 10 bytes ID)"]
    #[inline(always)]
    pub const fn secondlast(&self) -> &Secondlast {
        &self.secondlast
    }
    #[doc = "0x08 - Third last NFCID1 part (10 bytes ID)"]
    #[inline(always)]
    pub const fn thirdlast(&self) -> &Thirdlast {
        &self.thirdlast
    }
}
#[doc = "LAST (rw) register accessor: Last NFCID1 part (4, 7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`last::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`last::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@last`]
module"]
#[doc(alias = "LAST")]
pub type Last = crate::Reg<last::LastSpec>;
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
pub mod last;
#[doc = "SECONDLAST (rw) register accessor: Second last NFCID1 part (7 or 10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`secondlast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secondlast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secondlast`]
module"]
#[doc(alias = "SECONDLAST")]
pub type Secondlast = crate::Reg<secondlast::SecondlastSpec>;
#[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
pub mod secondlast;
#[doc = "THIRDLAST (rw) register accessor: Third last NFCID1 part (10 bytes ID)\n\nYou can [`read`](crate::Reg::read) this register and get [`thirdlast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thirdlast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thirdlast`]
module"]
#[doc(alias = "THIRDLAST")]
pub type Thirdlast = crate::Reg<thirdlast::ThirdlastSpec>;
#[doc = "Third last NFCID1 part (10 bytes ID)"]
pub mod thirdlast;
