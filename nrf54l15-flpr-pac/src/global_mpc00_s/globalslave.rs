#[repr(C)]
#[doc = "Global slave master port connection information"]
#[doc(alias = "GLOBALSLAVE")]
pub struct Globalslave {
    masterport: Masterport,
    lock: Lock,
}
impl Globalslave {
    #[doc = "0x00 - Global slave connection information for master port"]
    #[inline(always)]
    pub const fn masterport(&self) -> &Masterport {
        &self.masterport
    }
    #[doc = "0x04 - Lock global slave registers"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "MASTERPORT (rw) register accessor: Global slave connection information for master port\n\nYou can [`read`](crate::Reg::read) this register and get [`masterport::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masterport::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masterport`]
module"]
#[doc(alias = "MASTERPORT")]
pub type Masterport = crate::Reg<masterport::MasterportSpec>;
#[doc = "Global slave connection information for master port"]
pub mod masterport;
#[doc = "LOCK (rw) register accessor: Lock global slave registers\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Lock global slave registers"]
pub mod lock;
