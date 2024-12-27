#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DOMAIN")]
pub struct Domain {
    dbgen: Dbgen,
    niden: Niden,
    spiden: Spiden,
    spniden: Spniden,
}
impl Domain {
    #[doc = "0x00..0x08 - Unspecified"]
    #[inline(always)]
    pub const fn dbgen(&self) -> &Dbgen {
        &self.dbgen
    }
    #[doc = "0x08..0x10 - Unspecified"]
    #[inline(always)]
    pub const fn niden(&self) -> &Niden {
        &self.niden
    }
    #[doc = "0x10..0x18 - Unspecified"]
    #[inline(always)]
    pub const fn spiden(&self) -> &Spiden {
        &self.spiden
    }
    #[doc = "0x18..0x20 - Unspecified"]
    #[inline(always)]
    pub const fn spniden(&self) -> &Spniden {
        &self.spniden
    }
}
#[doc = "Unspecified"]
pub use self::dbgen::Dbgen;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dbgen;
#[doc = "Unspecified"]
pub use self::niden::Niden;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod niden;
#[doc = "Unspecified"]
pub use self::spiden::Spiden;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod spiden;
#[doc = "Unspecified"]
pub use self::spniden::Spniden;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod spniden;
