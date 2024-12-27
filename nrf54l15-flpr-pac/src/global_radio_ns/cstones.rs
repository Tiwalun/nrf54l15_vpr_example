#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "CSTONES")]
pub struct Cstones {
    mode: Mode,
    numsamples: Numsamples,
    nextfrequency: Nextfrequency,
    ffoin: Ffoin,
    ffosource: Ffosource,
    faepeer: Faepeer,
    phaseshift: Phaseshift,
    numsamplescoeff: Numsamplescoeff,
    pct16: Pct16,
    magphasemean: Magphasemean,
    iqrawmean: Iqrawmean,
    magstd: Magstd,
    cnacc: Cnacc,
    ffoest: Ffoest,
    downsample: Downsample,
    finetunenext: Finetunenext,
    cfophase: Cfophase,
    freqoffset: Freqoffset,
    pct11: Pct11,
    lfaenext: Lfaenext,
}
impl Cstones {
    #[doc = "0x00 - Selects the mode(s) that are activated on the start signal"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - Number of input samples at 2MHz sample rate"]
    #[inline(always)]
    pub const fn numsamples(&self) -> &Numsamples {
        &self.numsamples
    }
    #[doc = "0x08 - The value of FREQUENCY that will be used in the next step"]
    #[inline(always)]
    pub const fn nextfrequency(&self) -> &Nextfrequency {
        &self.nextfrequency
    }
    #[doc = "0x0c - Override value of FFO (Fractional Frequency Offset) if not to be based on the frequency estimate derived from CnAcc (autocorrelation of the scaled input signal) value"]
    #[inline(always)]
    pub const fn ffoin(&self) -> &Ffoin {
        &self.ffoin
    }
    #[doc = "0x10 - Source of FFO"]
    #[inline(always)]
    pub const fn ffosource(&self) -> &Ffosource {
        &self.ffosource
    }
    #[doc = "0x14 - FAEPEER (Frequency Actuation Error) of peer if known. Used during Mode 0 steps."]
    #[inline(always)]
    pub const fn faepeer(&self) -> &Faepeer {
        &self.faepeer
    }
    #[doc = "0x18 - Parameter used in TPM, provided by software"]
    #[inline(always)]
    pub const fn phaseshift(&self) -> &Phaseshift {
        &self.phaseshift
    }
    #[doc = "0x1c - Parameter used in TPM, provided by software"]
    #[inline(always)]
    pub const fn numsamplescoeff(&self) -> &Numsamplescoeff {
        &self.numsamplescoeff
    }
    #[doc = "0x20 - Mean magnitude and mean phase converted to IQ"]
    #[inline(always)]
    pub const fn pct16(&self) -> &Pct16 {
        &self.pct16
    }
    #[doc = "0x24 - Mean magnitude and phase of the signal before it is converted to PCT16"]
    #[inline(always)]
    pub const fn magphasemean(&self) -> &Magphasemean {
        &self.magphasemean
    }
    #[doc = "0x28 - Mean of IQ values"]
    #[inline(always)]
    pub const fn iqrawmean(&self) -> &Iqrawmean {
        &self.iqrawmean
    }
    #[doc = "0x2c - Magnitude standard deviation approximation"]
    #[inline(always)]
    pub const fn magstd(&self) -> &Magstd {
        &self.magstd
    }
    #[doc = "0x30 - Output of the autocorrelation of the accumulated IQ signal"]
    #[inline(always)]
    pub const fn cnacc(&self) -> &Cnacc {
        &self.cnacc
    }
    #[doc = "0x34 - FFO estimate"]
    #[inline(always)]
    pub const fn ffoest(&self) -> &Ffoest {
        &self.ffoest
    }
    #[doc = "0x38 - Turn on/off down sample of input IQ-signals"]
    #[inline(always)]
    pub const fn downsample(&self) -> &Downsample {
        &self.downsample
    }
    #[doc = "0x3c - Number of full ADPLL finetune steps"]
    #[inline(always)]
    pub const fn finetunenext(&self) -> &Finetunenext {
        &self.finetunenext
    }
    #[doc = "0x40 - Cordic output of CnAcc"]
    #[inline(always)]
    pub const fn cfophase(&self) -> &Cfophase {
        &self.cfophase
    }
    #[doc = "0x44 - Frequency offset estimate"]
    #[inline(always)]
    pub const fn freqoffset(&self) -> &Freqoffset {
        &self.freqoffset
    }
    #[doc = "0x48 - Mean magnitude and mean phase converted to IQ. IQ values limited to \\[-1024,1023\\]."]
    #[inline(always)]
    pub const fn pct11(&self) -> &Pct11 {
        &self.pct11
    }
    #[doc = "0x4c - Quantization error between ADPLL frequency and the desired value of FFO * RF Frequency. Values limited to \\[-64,63\\]
with units 7.6294 Hz."]
    #[inline(always)]
    pub const fn lfaenext(&self) -> &Lfaenext {
        &self.lfaenext
    }
}
#[doc = "MODE (rw) register accessor: Selects the mode(s) that are activated on the start signal\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Selects the mode(s) that are activated on the start signal"]
pub mod mode;
#[doc = "NUMSAMPLES (rw) register accessor: Number of input samples at 2MHz sample rate\n\nYou can [`read`](crate::Reg::read) this register and get [`numsamples::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`numsamples::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@numsamples`]
module"]
#[doc(alias = "NUMSAMPLES")]
pub type Numsamples = crate::Reg<numsamples::NumsamplesSpec>;
#[doc = "Number of input samples at 2MHz sample rate"]
pub mod numsamples;
#[doc = "NEXTFREQUENCY (rw) register accessor: The value of FREQUENCY that will be used in the next step\n\nYou can [`read`](crate::Reg::read) this register and get [`nextfrequency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nextfrequency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nextfrequency`]
module"]
#[doc(alias = "NEXTFREQUENCY")]
pub type Nextfrequency = crate::Reg<nextfrequency::NextfrequencySpec>;
#[doc = "The value of FREQUENCY that will be used in the next step"]
pub mod nextfrequency;
#[doc = "FFOIN (rw) register accessor: Override value of FFO (Fractional Frequency Offset) if not to be based on the frequency estimate derived from CnAcc (autocorrelation of the scaled input signal) value\n\nYou can [`read`](crate::Reg::read) this register and get [`ffoin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffoin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffoin`]
module"]
#[doc(alias = "FFOIN")]
pub type Ffoin = crate::Reg<ffoin::FfoinSpec>;
#[doc = "Override value of FFO (Fractional Frequency Offset) if not to be based on the frequency estimate derived from CnAcc (autocorrelation of the scaled input signal) value"]
pub mod ffoin;
#[doc = "FFOSOURCE (rw) register accessor: Source of FFO\n\nYou can [`read`](crate::Reg::read) this register and get [`ffosource::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffosource::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffosource`]
module"]
#[doc(alias = "FFOSOURCE")]
pub type Ffosource = crate::Reg<ffosource::FfosourceSpec>;
#[doc = "Source of FFO"]
pub mod ffosource;
#[doc = "FAEPEER (rw) register accessor: FAEPEER (Frequency Actuation Error) of peer if known. Used during Mode 0 steps.\n\nYou can [`read`](crate::Reg::read) this register and get [`faepeer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faepeer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faepeer`]
module"]
#[doc(alias = "FAEPEER")]
pub type Faepeer = crate::Reg<faepeer::FaepeerSpec>;
#[doc = "FAEPEER (Frequency Actuation Error) of peer if known. Used during Mode 0 steps."]
pub mod faepeer;
#[doc = "PHASESHIFT (rw) register accessor: Parameter used in TPM, provided by software\n\nYou can [`read`](crate::Reg::read) this register and get [`phaseshift::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phaseshift::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phaseshift`]
module"]
#[doc(alias = "PHASESHIFT")]
pub type Phaseshift = crate::Reg<phaseshift::PhaseshiftSpec>;
#[doc = "Parameter used in TPM, provided by software"]
pub mod phaseshift;
#[doc = "NUMSAMPLESCOEFF (rw) register accessor: Parameter used in TPM, provided by software\n\nYou can [`read`](crate::Reg::read) this register and get [`numsamplescoeff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`numsamplescoeff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@numsamplescoeff`]
module"]
#[doc(alias = "NUMSAMPLESCOEFF")]
pub type Numsamplescoeff = crate::Reg<numsamplescoeff::NumsamplescoeffSpec>;
#[doc = "Parameter used in TPM, provided by software"]
pub mod numsamplescoeff;
#[doc = "PCT16 (r) register accessor: Mean magnitude and mean phase converted to IQ\n\nYou can [`read`](crate::Reg::read) this register and get [`pct16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pct16`]
module"]
#[doc(alias = "PCT16")]
pub type Pct16 = crate::Reg<pct16::Pct16Spec>;
#[doc = "Mean magnitude and mean phase converted to IQ"]
pub mod pct16;
#[doc = "MAGPHASEMEAN (r) register accessor: Mean magnitude and phase of the signal before it is converted to PCT16\n\nYou can [`read`](crate::Reg::read) this register and get [`magphasemean::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@magphasemean`]
module"]
#[doc(alias = "MAGPHASEMEAN")]
pub type Magphasemean = crate::Reg<magphasemean::MagphasemeanSpec>;
#[doc = "Mean magnitude and phase of the signal before it is converted to PCT16"]
pub mod magphasemean;
#[doc = "IQRAWMEAN (r) register accessor: Mean of IQ values\n\nYou can [`read`](crate::Reg::read) this register and get [`iqrawmean::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iqrawmean`]
module"]
#[doc(alias = "IQRAWMEAN")]
pub type Iqrawmean = crate::Reg<iqrawmean::IqrawmeanSpec>;
#[doc = "Mean of IQ values"]
pub mod iqrawmean;
#[doc = "MAGSTD (r) register accessor: Magnitude standard deviation approximation\n\nYou can [`read`](crate::Reg::read) this register and get [`magstd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@magstd`]
module"]
#[doc(alias = "MAGSTD")]
pub type Magstd = crate::Reg<magstd::MagstdSpec>;
#[doc = "Magnitude standard deviation approximation"]
pub mod magstd;
#[doc = "CNACC (r) register accessor: Output of the autocorrelation of the accumulated IQ signal\n\nYou can [`read`](crate::Reg::read) this register and get [`cnacc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnacc`]
module"]
#[doc(alias = "CNACC")]
pub type Cnacc = crate::Reg<cnacc::CnaccSpec>;
#[doc = "Output of the autocorrelation of the accumulated IQ signal"]
pub mod cnacc;
#[doc = "FFOEST (r) register accessor: FFO estimate\n\nYou can [`read`](crate::Reg::read) this register and get [`ffoest::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffoest`]
module"]
#[doc(alias = "FFOEST")]
pub type Ffoest = crate::Reg<ffoest::FfoestSpec>;
#[doc = "FFO estimate"]
pub mod ffoest;
#[doc = "DOWNSAMPLE (rw) register accessor: Turn on/off down sample of input IQ-signals\n\nYou can [`read`](crate::Reg::read) this register and get [`downsample::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`downsample::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@downsample`]
module"]
#[doc(alias = "DOWNSAMPLE")]
pub type Downsample = crate::Reg<downsample::DownsampleSpec>;
#[doc = "Turn on/off down sample of input IQ-signals"]
pub mod downsample;
#[doc = "FINETUNENEXT (r) register accessor: Number of full ADPLL finetune steps\n\nYou can [`read`](crate::Reg::read) this register and get [`finetunenext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@finetunenext`]
module"]
#[doc(alias = "FINETUNENEXT")]
pub type Finetunenext = crate::Reg<finetunenext::FinetunenextSpec>;
#[doc = "Number of full ADPLL finetune steps"]
pub mod finetunenext;
#[doc = "CFOPHASE (r) register accessor: Cordic output of CnAcc\n\nYou can [`read`](crate::Reg::read) this register and get [`cfophase::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfophase`]
module"]
#[doc(alias = "CFOPHASE")]
pub type Cfophase = crate::Reg<cfophase::CfophaseSpec>;
#[doc = "Cordic output of CnAcc"]
pub mod cfophase;
#[doc = "FREQOFFSET (r) register accessor: Frequency offset estimate\n\nYou can [`read`](crate::Reg::read) this register and get [`freqoffset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqoffset`]
module"]
#[doc(alias = "FREQOFFSET")]
pub type Freqoffset = crate::Reg<freqoffset::FreqoffsetSpec>;
#[doc = "Frequency offset estimate"]
pub mod freqoffset;
#[doc = "PCT11 (r) register accessor: Mean magnitude and mean phase converted to IQ. IQ values limited to \\[-1024,1023\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`pct11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pct11`]
module"]
#[doc(alias = "PCT11")]
pub type Pct11 = crate::Reg<pct11::Pct11Spec>;
#[doc = "Mean magnitude and mean phase converted to IQ. IQ values limited to \\[-1024,1023\\]."]
pub mod pct11;
#[doc = "LFAENEXT (r) register accessor: Quantization error between ADPLL frequency and the desired value of FFO * RF Frequency. Values limited to \\[-64,63\\]
with units 7.6294 Hz.\n\nYou can [`read`](crate::Reg::read) this register and get [`lfaenext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfaenext`]
module"]
#[doc(alias = "LFAENEXT")]
pub type Lfaenext = crate::Reg<lfaenext::LfaenextSpec>;
#[doc = "Quantization error between ADPLL frequency and the desired value of FFO * RF Frequency. Values limited to \\[-64,63\\]
with units 7.6294 Hz."]
pub mod lfaenext;
