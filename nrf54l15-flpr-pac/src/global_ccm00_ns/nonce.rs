#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "NONCE")]
pub struct Nonce {
    value: [Value; 4],
}
impl Nonce {
    #[doc = "0x00..0x10 - Description collection: 13-byte NONCE vector Only the lower 13 bytes are used"]
    #[inline(always)]
    pub const fn value(&self, n: usize) -> &Value {
        &self.value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Description collection: 13-byte NONCE vector Only the lower 13 bytes are used"]
    #[inline(always)]
    pub fn value_iter(&self) -> impl Iterator<Item = &Value> {
        self.value.iter()
    }
}
#[doc = "VALUE (rw) register accessor: Description collection: 13-byte NONCE vector Only the lower 13 bytes are used\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Description collection: 13-byte NONCE vector Only the lower 13 bytes are used"]
pub mod value;
