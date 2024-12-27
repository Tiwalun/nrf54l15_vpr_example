#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "LFCLK")]
pub struct Lfclk {
    src: Src,
    _reserved1: [u8; 0x04],
    run: Run,
    stat: Stat,
    srccopy: Srccopy,
}
impl Lfclk {
    #[doc = "0x00 - Clock source for LFCLK"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x08 - Indicates that LFCLKSTART task was triggered"]
    #[inline(always)]
    pub const fn run(&self) -> &Run {
        &self.run
    }
    #[doc = "0x0c - Copy of LFCLK.SRCCOPY register, set when LFCLKSTARTED event is triggered."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x10 - Copy of LFCLK.SRC register, set when LFCLKSTART task is triggered"]
    #[inline(always)]
    pub const fn srccopy(&self) -> &Srccopy {
        &self.srccopy
    }
}
#[doc = "SRC (rw) register accessor: Clock source for LFCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "Clock source for LFCLK"]
pub mod src;
#[doc = "RUN (r) register accessor: Indicates that LFCLKSTART task was triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`run::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@run`]
module"]
#[doc(alias = "RUN")]
pub type Run = crate::Reg<run::RunSpec>;
#[doc = "Indicates that LFCLKSTART task was triggered"]
pub mod run;
#[doc = "STAT (r) register accessor: Copy of LFCLK.SRCCOPY register, set when LFCLKSTARTED event is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Copy of LFCLK.SRCCOPY register, set when LFCLKSTARTED event is triggered."]
pub mod stat;
#[doc = "SRCCOPY (rw) register accessor: Copy of LFCLK.SRC register, set when LFCLKSTART task is triggered\n\nYou can [`read`](crate::Reg::read) this register and get [`srccopy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srccopy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srccopy`]
module"]
#[doc(alias = "SRCCOPY")]
pub type Srccopy = crate::Reg<srccopy::SrccopySpec>;
#[doc = "Copy of LFCLK.SRC register, set when LFCLKSTART task is triggered"]
pub mod srccopy;
