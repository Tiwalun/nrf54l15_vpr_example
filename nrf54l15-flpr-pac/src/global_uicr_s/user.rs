#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "USER")]
pub struct User {
    rot: Rot,
}
impl User {
    #[doc = "0x00..0x160 - Assets installed to establish initial Root of Trust in the device."]
    #[inline(always)]
    pub const fn rot(&self) -> &Rot {
        &self.rot
    }
}
#[doc = "Assets installed to establish initial Root of Trust in the device."]
pub use self::rot::Rot;
#[doc = r"Cluster"]
#[doc = "Assets installed to establish initial Root of Trust in the device."]
pub mod rot;
