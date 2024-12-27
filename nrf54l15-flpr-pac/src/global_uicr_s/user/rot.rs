#[repr(C)]
#[doc = "Assets installed to establish initial Root of Trust in the device."]
#[doc(alias = "ROT")]
pub struct Rot {
    pubkey: [Pubkey; 4],
    authopkey: [Authopkey; 4],
}
impl Rot {
    #[doc = "0x00..0xb0 - Unspecified"]
    #[inline(always)]
    pub const fn pubkey(&self, n: usize) -> &Pubkey {
        &self.pubkey[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xb0 - Unspecified"]
    #[inline(always)]
    pub fn pubkey_iter(&self) -> impl Iterator<Item = &Pubkey> {
        self.pubkey.iter()
    }
    #[doc = "0xb0..0x160 - Unspecified"]
    #[inline(always)]
    pub const fn authopkey(&self, n: usize) -> &Authopkey {
        &self.authopkey[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0x160 - Unspecified"]
    #[inline(always)]
    pub fn authopkey_iter(&self) -> impl Iterator<Item = &Authopkey> {
        self.authopkey.iter()
    }
}
#[doc = "Unspecified"]
pub use self::pubkey::Pubkey;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod pubkey;
#[doc = "Unspecified"]
pub use self::authopkey::Authopkey;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod authopkey;
