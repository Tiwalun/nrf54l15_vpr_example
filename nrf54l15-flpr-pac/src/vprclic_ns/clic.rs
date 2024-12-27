#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CLIC")]
pub struct Clic {
    cliccfg: Cliccfg,
    clicinfo: Clicinfo,
    _reserved2: [u8; 0x0ff8],
    clicint: [Clicint; 271],
}
impl Clic {
    #[doc = "0x00 - CLIC configuration."]
    #[inline(always)]
    pub const fn cliccfg(&self) -> &Cliccfg {
        &self.cliccfg
    }
    #[doc = "0x04 - CLIC information."]
    #[inline(always)]
    pub const fn clicinfo(&self) -> &Clicinfo {
        &self.clicinfo
    }
    #[doc = "0x1000..0x143c - Description collection: Interrupt control register for IRQ number \\[n\\]."]
    #[inline(always)]
    pub const fn clicint(&self, n: usize) -> &Clicint {
        &self.clicint[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x143c - Description collection: Interrupt control register for IRQ number \\[n\\]."]
    #[inline(always)]
    pub fn clicint_iter(&self) -> impl Iterator<Item = &Clicint> {
        self.clicint.iter()
    }
}
#[doc = "CLICCFG (r) register accessor: CLIC configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`cliccfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cliccfg`]
module"]
#[doc(alias = "CLICCFG")]
pub type Cliccfg = crate::Reg<cliccfg::CliccfgSpec>;
#[doc = "CLIC configuration."]
pub mod cliccfg;
#[doc = "CLICINFO (r) register accessor: CLIC information.\n\nYou can [`read`](crate::Reg::read) this register and get [`clicinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicinfo`]
module"]
#[doc(alias = "CLICINFO")]
pub type Clicinfo = crate::Reg<clicinfo::ClicinfoSpec>;
#[doc = "CLIC information."]
pub mod clicinfo;
#[doc = "CLICINT (rw) register accessor: Description collection: Interrupt control register for IRQ number \\[n\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`clicint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clicint`]
module"]
#[doc(alias = "CLICINT")]
pub type Clicint = crate::Reg<clicint::ClicintSpec>;
#[doc = "Description collection: Interrupt control register for IRQ number \\[n\\]."]
pub mod clicint;
