#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cryptmstrdma: Cryptmstrdma,
    _reserved1: [u8; 0x03c0],
    cryptmstrhw: Cryptmstrhw,
    _reserved2: [u8; 0x0be4],
    rngcontrol: Rngcontrol,
    _reserved3: [u8; 0x0f40],
    pk: Pk,
    _reserved4: [u8; 0x0fb8],
    ikg: Ikg,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Unspecified"]
    #[inline(always)]
    pub const fn cryptmstrdma(&self) -> &Cryptmstrdma {
        &self.cryptmstrdma
    }
    #[doc = "0x400..0x41c - Unspecified"]
    #[inline(always)]
    pub const fn cryptmstrhw(&self) -> &Cryptmstrhw {
        &self.cryptmstrhw
    }
    #[doc = "0x1000..0x10c0 - Unspecified"]
    #[inline(always)]
    pub const fn rngcontrol(&self) -> &Rngcontrol {
        &self.rngcontrol
    }
    #[doc = "0x2000..0x2048 - Unspecified"]
    #[inline(always)]
    pub const fn pk(&self) -> &Pk {
        &self.pk
    }
    #[doc = "0x3000..0x3030 - Unspecified"]
    #[inline(always)]
    pub const fn ikg(&self) -> &Ikg {
        &self.ikg
    }
}
#[doc = "Unspecified"]
pub use self::cryptmstrdma::Cryptmstrdma;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod cryptmstrdma;
#[doc = "Unspecified"]
pub use self::cryptmstrhw::Cryptmstrhw;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod cryptmstrhw;
#[doc = "Unspecified"]
pub use self::rngcontrol::Rngcontrol;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rngcontrol;
#[doc = "Unspecified"]
pub use self::pk::Pk;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod pk;
#[doc = "Unspecified"]
pub use self::ikg::Ikg;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ikg;
