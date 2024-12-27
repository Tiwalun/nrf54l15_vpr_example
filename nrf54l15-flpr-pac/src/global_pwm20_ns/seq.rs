#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "SEQ")]
pub struct Seq {
    _reserved0: [u8; 0x08],
    refresh: Refresh,
    enddelay: Enddelay,
}
impl Seq {
    #[doc = "0x08 - Description cluster: Number of additional PWM periods between samples loaded into compare register"]
    #[inline(always)]
    pub const fn refresh(&self) -> &Refresh {
        &self.refresh
    }
    #[doc = "0x0c - Description cluster: Time added after the sequence"]
    #[inline(always)]
    pub const fn enddelay(&self) -> &Enddelay {
        &self.enddelay
    }
}
#[doc = "REFRESH (rw) register accessor: Description cluster: Number of additional PWM periods between samples loaded into compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh`]
module"]
#[doc(alias = "REFRESH")]
pub type Refresh = crate::Reg<refresh::RefreshSpec>;
#[doc = "Description cluster: Number of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "ENDDELAY (rw) register accessor: Description cluster: Time added after the sequence\n\nYou can [`read`](crate::Reg::read) this register and get [`enddelay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enddelay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enddelay`]
module"]
#[doc(alias = "ENDDELAY")]
pub type Enddelay = crate::Reg<enddelay::EnddelaySpec>;
#[doc = "Description cluster: Time added after the sequence"]
pub mod enddelay;
