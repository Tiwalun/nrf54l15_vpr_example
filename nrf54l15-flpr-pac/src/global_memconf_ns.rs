#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    power: [Power; 2],
}
impl RegisterBlock {
    #[doc = "0x500..0x520 - Unspecified"]
    #[inline(always)]
    pub const fn power(&self, n: usize) -> &Power {
        &self.power[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x520 - Unspecified"]
    #[inline(always)]
    pub fn power_iter(&self) -> impl Iterator<Item = &Power> {
        self.power.iter()
    }
}
#[doc = "Unspecified"]
pub use self::power::Power;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod power;
