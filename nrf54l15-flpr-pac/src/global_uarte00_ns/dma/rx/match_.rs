#[repr(C)]
#[doc = "Registers to control the behavior of the pattern matcher engine"]
#[doc(alias = "MATCH")]
pub struct Match {
    config: Config,
    candidate: [Candidate; 4],
}
impl Match {
    #[doc = "0x00 - Configure individual match events"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04..0x14 - Description collection: The data to look for - any match will trigger the MATCH\\[n\\]
event, if enabled."]
    #[inline(always)]
    pub const fn candidate(&self, n: usize) -> &Candidate {
        &self.candidate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - Description collection: The data to look for - any match will trigger the MATCH\\[n\\]
event, if enabled."]
    #[inline(always)]
    pub fn candidate_iter(&self) -> impl Iterator<Item = &Candidate> {
        self.candidate.iter()
    }
}
#[doc = "CONFIG (rw) register accessor: Configure individual match events\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configure individual match events"]
pub mod config;
#[doc = "CANDIDATE (rw) register accessor: Description collection: The data to look for - any match will trigger the MATCH\\[n\\]
event, if enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`candidate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`candidate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@candidate`]
module"]
#[doc(alias = "CANDIDATE")]
pub type Candidate = crate::Reg<candidate::CandidateSpec>;
#[doc = "Description collection: The data to look for - any match will trigger the MATCH\\[n\\]
event, if enabled."]
pub mod candidate;
