#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "AUTHOPKEY")]
pub struct Authopkey {
    digest: [Digest; 8],
    revoke: [Revoke; 3],
}
impl Authopkey {
    #[doc = "0x00..0x20 - Description collection: First 256 bits of SHA2-512 digest over RoT authenticated operation public key generation \\[n\\]."]
    #[inline(always)]
    pub const fn digest(&self, n: usize) -> &Digest {
        &self.digest[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: First 256 bits of SHA2-512 digest over RoT authenticated operation public key generation \\[n\\]."]
    #[inline(always)]
    pub fn digest_iter(&self) -> impl Iterator<Item = &Digest> {
        self.digest.iter()
    }
    #[doc = "0x20..0x2c - Description collection: Revocation status for RoT authenticated operation public key generation \\[n\\]."]
    #[inline(always)]
    pub const fn revoke(&self, n: usize) -> &Revoke {
        &self.revoke[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x2c - Description collection: Revocation status for RoT authenticated operation public key generation \\[n\\]."]
    #[inline(always)]
    pub fn revoke_iter(&self) -> impl Iterator<Item = &Revoke> {
        self.revoke.iter()
    }
}
#[doc = "DIGEST (rw) register accessor: Description collection: First 256 bits of SHA2-512 digest over RoT authenticated operation public key generation \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`digest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`digest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@digest`]
module"]
#[doc(alias = "DIGEST")]
pub type Digest = crate::Reg<digest::DigestSpec>;
#[doc = "Description collection: First 256 bits of SHA2-512 digest over RoT authenticated operation public key generation \\[n\\]."]
pub mod digest;
#[doc = "REVOKE (rw) register accessor: Description collection: Revocation status for RoT authenticated operation public key generation \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`revoke::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`revoke::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revoke`]
module"]
#[doc(alias = "REVOKE")]
pub type Revoke = crate::Reg<revoke::RevokeSpec>;
#[doc = "Description collection: Revocation status for RoT authenticated operation public key generation \\[n\\]."]
pub mod revoke;
