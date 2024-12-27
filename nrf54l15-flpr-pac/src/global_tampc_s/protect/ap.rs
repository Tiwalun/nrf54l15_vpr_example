#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "AP")]
pub struct Ap {
    dbgen: Dbgen,
}
impl Ap {
    #[doc = "0x00..0x08 - Unspecified"]
    #[inline(always)]
    pub const fn dbgen(&self) -> &Dbgen {
        &self.dbgen
    }
}
#[doc = "Unspecified"]
pub use self::dbgen::Dbgen;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dbgen;
