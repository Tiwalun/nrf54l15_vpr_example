#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "KEY")]
pub struct Key {
    value: [Value; 4],
}
impl Key {
    #[doc = "0x00..0x10 - Description collection: 128-bit AES key"]
    #[inline(always)]
    pub const fn value(&self, n: usize) -> &Value {
        &self.value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Description collection: 128-bit AES key"]
    #[inline(always)]
    pub fn value_iter(&self) -> impl Iterator<Item = &Value> {
        self.value.iter()
    }
}
#[doc = "VALUE (w) register accessor: Description collection: 128-bit AES key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Description collection: 128-bit AES key"]
pub mod value;
