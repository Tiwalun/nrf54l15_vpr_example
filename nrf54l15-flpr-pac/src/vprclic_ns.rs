#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clic: Clic,
}
impl RegisterBlock {
    #[doc = "0x00..0x143c - Unspecified"]
    #[inline(always)]
    pub const fn clic(&self) -> &Clic {
        &self.clic
    }
}
#[doc = "Unspecified"]
pub use self::clic::Clic;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod clic;
