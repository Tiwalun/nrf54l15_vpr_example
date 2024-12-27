#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "GPIO")]
pub struct Gpio {
    pin: [Pin; 32],
}
impl Gpio {
    #[doc = "0x00..0x80 - Description collection: Configuration of features for GPIO\\[n\\]
PIN\\[o\\]"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &Pin {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Description collection: Configuration of features for GPIO\\[n\\]
PIN\\[o\\]"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &Pin> {
        self.pin.iter()
    }
}
#[doc = "PIN (rw) register accessor: Description collection: Configuration of features for GPIO\\[n\\]
PIN\\[o\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`]
module"]
#[doc(alias = "PIN")]
pub type Pin = crate::Reg<pin::PinSpec>;
#[doc = "Description collection: Configuration of features for GPIO\\[n\\]
PIN\\[o\\]"]
pub mod pin;
