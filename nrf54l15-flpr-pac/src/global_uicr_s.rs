#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    approtect: [Approtect; 1],
    secureapprotect: [Secureapprotect; 1],
    auxapprotect: [Auxapprotect; 1],
    eraseprotect: [Eraseprotect; 1],
    bootconf: Bootconf,
    _reserved5: [u8; 0x017c],
    user: User,
    _reserved6: [u8; 0x01a0],
    otp: [Otp; 320],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Access Port Protection Registers"]
    #[inline(always)]
    pub const fn approtect(&self, n: usize) -> &Approtect {
        &self.approtect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Access Port Protection Registers"]
    #[inline(always)]
    pub fn approtect_iter(&self) -> impl Iterator<Item = &Approtect> {
        self.approtect.iter()
    }
    #[doc = "0x20..0x40 - Access Port Protection Registers"]
    #[inline(always)]
    pub const fn secureapprotect(&self, n: usize) -> &Secureapprotect {
        &self.secureapprotect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - Access Port Protection Registers"]
    #[inline(always)]
    pub fn secureapprotect_iter(&self) -> impl Iterator<Item = &Secureapprotect> {
        self.secureapprotect.iter()
    }
    #[doc = "0x40..0x60 - Access Port Protection Registers"]
    #[inline(always)]
    pub const fn auxapprotect(&self, n: usize) -> &Auxapprotect {
        &self.auxapprotect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - Access Port Protection Registers"]
    #[inline(always)]
    pub fn auxapprotect_iter(&self) -> impl Iterator<Item = &Auxapprotect> {
        self.auxapprotect.iter()
    }
    #[doc = "0x60..0x80 - Erase Protection Registers"]
    #[inline(always)]
    pub const fn eraseprotect(&self, n: usize) -> &Eraseprotect {
        &self.eraseprotect[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x80 - Erase Protection Registers"]
    #[inline(always)]
    pub fn eraseprotect_iter(&self) -> impl Iterator<Item = &Eraseprotect> {
        self.eraseprotect.iter()
    }
    #[doc = "0x80 - Immutable boot region configuration."]
    #[inline(always)]
    pub const fn bootconf(&self) -> &Bootconf {
        &self.bootconf
    }
    #[doc = "0x200..0x360 - Unspecified"]
    #[inline(always)]
    pub const fn user(&self) -> &User {
        &self.user
    }
    #[doc = "0x500..0xa00 - Description collection: One time programmable memory"]
    #[inline(always)]
    pub const fn otp(&self, n: usize) -> &Otp {
        &self.otp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0xa00 - Description collection: One time programmable memory"]
    #[inline(always)]
    pub fn otp_iter(&self) -> impl Iterator<Item = &Otp> {
        self.otp.iter()
    }
}
#[doc = "Access Port Protection Registers"]
pub use self::approtect::Approtect;
#[doc = r"Cluster"]
#[doc = "Access Port Protection Registers"]
pub mod approtect;
#[doc = "Access Port Protection Registers"]
pub use self::secureapprotect::Secureapprotect;
#[doc = r"Cluster"]
#[doc = "Access Port Protection Registers"]
pub mod secureapprotect;
#[doc = "Access Port Protection Registers"]
pub use self::auxapprotect::Auxapprotect;
#[doc = r"Cluster"]
#[doc = "Access Port Protection Registers"]
pub mod auxapprotect;
#[doc = "Erase Protection Registers"]
pub use self::eraseprotect::Eraseprotect;
#[doc = r"Cluster"]
#[doc = "Erase Protection Registers"]
pub mod eraseprotect;
#[doc = "BOOTCONF (rw) register accessor: Immutable boot region configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`bootconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootconf`]
module"]
#[doc(alias = "BOOTCONF")]
pub type Bootconf = crate::Reg<bootconf::BootconfSpec>;
#[doc = "Immutable boot region configuration."]
pub mod bootconf;
#[doc = "Unspecified"]
pub use self::user::User;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod user;
#[doc = "OTP (rw) register accessor: Description collection: One time programmable memory\n\nYou can [`read`](crate::Reg::read) this register and get [`otp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp`]
module"]
#[doc(alias = "OTP")]
pub type Otp = crate::Reg<otp::OtpSpec>;
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
