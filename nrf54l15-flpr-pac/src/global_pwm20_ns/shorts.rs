#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event SEQEND\\[n\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend0Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Seqend0Stop> for bool {
    #[inline(always)]
    fn from(variant: Seqend0Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0_STOP` reader - Shortcut between event SEQEND\\[n\\]
and task STOP"]
pub type Seqend0StopR = crate::BitReader<Seqend0Stop>;
impl Seqend0StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend0Stop {
        match self.bits {
            false => Seqend0Stop::Disabled,
            true => Seqend0Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend0Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend0Stop::Enabled
    }
}
#[doc = "Field `SEQEND0_STOP` writer - Shortcut between event SEQEND\\[n\\]
and task STOP"]
pub type Seqend0StopW<'a, REG> = crate::BitWriter<'a, REG, Seqend0Stop>;
impl<'a, REG> Seqend0StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend0Stop::Enabled)
    }
}
#[doc = "Shortcut between event SEQEND\\[n\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seqend1Stop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<Seqend1Stop> for bool {
    #[inline(always)]
    fn from(variant: Seqend1Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1_STOP` reader - Shortcut between event SEQEND\\[n\\]
and task STOP"]
pub type Seqend1StopR = crate::BitReader<Seqend1Stop>;
impl Seqend1StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seqend1Stop {
        match self.bits {
            false => Seqend1Stop::Disabled,
            true => Seqend1Stop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Seqend1Stop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Seqend1Stop::Enabled
    }
}
#[doc = "Field `SEQEND1_STOP` writer - Shortcut between event SEQEND\\[n\\]
and task STOP"]
pub type Seqend1StopW<'a, REG> = crate::BitWriter<'a, REG, Seqend1Stop>;
impl<'a, REG> Seqend1StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1Stop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Seqend1Stop::Enabled)
    }
}
#[doc = "Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneDmaSeq0Start {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LoopsdoneDmaSeq0Start> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneDmaSeq0Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_DMA_SEQ0_START` reader - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
pub type LoopsdoneDmaSeq0StartR = crate::BitReader<LoopsdoneDmaSeq0Start>;
impl LoopsdoneDmaSeq0StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopsdoneDmaSeq0Start {
        match self.bits {
            false => LoopsdoneDmaSeq0Start::Disabled,
            true => LoopsdoneDmaSeq0Start::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LoopsdoneDmaSeq0Start::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LoopsdoneDmaSeq0Start::Enabled
    }
}
#[doc = "Field `LOOPSDONE_DMA_SEQ0_START` writer - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
pub type LoopsdoneDmaSeq0StartW<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneDmaSeq0Start>;
impl<'a, REG> LoopsdoneDmaSeq0StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneDmaSeq0Start::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneDmaSeq0Start::Enabled)
    }
}
#[doc = "Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneDmaSeq1Start {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LoopsdoneDmaSeq1Start> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneDmaSeq1Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_DMA_SEQ1_START` reader - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
pub type LoopsdoneDmaSeq1StartR = crate::BitReader<LoopsdoneDmaSeq1Start>;
impl LoopsdoneDmaSeq1StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopsdoneDmaSeq1Start {
        match self.bits {
            false => LoopsdoneDmaSeq1Start::Disabled,
            true => LoopsdoneDmaSeq1Start::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LoopsdoneDmaSeq1Start::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LoopsdoneDmaSeq1Start::Enabled
    }
}
#[doc = "Field `LOOPSDONE_DMA_SEQ1_START` writer - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
pub type LoopsdoneDmaSeq1StartW<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneDmaSeq1Start>;
impl<'a, REG> LoopsdoneDmaSeq1StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneDmaSeq1Start::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneDmaSeq1Start::Enabled)
    }
}
#[doc = "Shortcut between event LOOPSDONE and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopsdoneStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LoopsdoneStop> for bool {
    #[inline(always)]
    fn from(variant: LoopsdoneStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE_STOP` reader - Shortcut between event LOOPSDONE and task STOP"]
pub type LoopsdoneStopR = crate::BitReader<LoopsdoneStop>;
impl LoopsdoneStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopsdoneStop {
        match self.bits {
            false => LoopsdoneStop::Disabled,
            true => LoopsdoneStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LoopsdoneStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LoopsdoneStop::Enabled
    }
}
#[doc = "Field `LOOPSDONE_STOP` writer - Shortcut between event LOOPSDONE and task STOP"]
pub type LoopsdoneStopW<'a, REG> = crate::BitWriter<'a, REG, LoopsdoneStop>;
impl<'a, REG> LoopsdoneStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LoopsdoneStop::Enabled)
    }
}
#[doc = "Shortcut between event RAMUNDERFLOW and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamunderflowStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<RamunderflowStop> for bool {
    #[inline(always)]
    fn from(variant: RamunderflowStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMUNDERFLOW_STOP` reader - Shortcut between event RAMUNDERFLOW and task STOP"]
pub type RamunderflowStopR = crate::BitReader<RamunderflowStop>;
impl RamunderflowStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamunderflowStop {
        match self.bits {
            false => RamunderflowStop::Disabled,
            true => RamunderflowStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RamunderflowStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RamunderflowStop::Enabled
    }
}
#[doc = "Field `RAMUNDERFLOW_STOP` writer - Shortcut between event RAMUNDERFLOW and task STOP"]
pub type RamunderflowStopW<'a, REG> = crate::BitWriter<'a, REG, RamunderflowStop>;
impl<'a, REG> RamunderflowStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RamunderflowStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RamunderflowStop::Enabled)
    }
}
#[doc = "Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaSeq0BuserrorStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaSeq0BuserrorStop> for bool {
    #[inline(always)]
    fn from(variant: DmaSeq0BuserrorStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_SEQ0_BUSERROR_STOP` reader - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
pub type DmaSeq0BuserrorStopR = crate::BitReader<DmaSeq0BuserrorStop>;
impl DmaSeq0BuserrorStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaSeq0BuserrorStop {
        match self.bits {
            false => DmaSeq0BuserrorStop::Disabled,
            true => DmaSeq0BuserrorStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaSeq0BuserrorStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaSeq0BuserrorStop::Enabled
    }
}
#[doc = "Field `DMA_SEQ0_BUSERROR_STOP` writer - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
pub type DmaSeq0BuserrorStopW<'a, REG> = crate::BitWriter<'a, REG, DmaSeq0BuserrorStop>;
impl<'a, REG> DmaSeq0BuserrorStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaSeq0BuserrorStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaSeq0BuserrorStop::Enabled)
    }
}
#[doc = "Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaSeq1BuserrorStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaSeq1BuserrorStop> for bool {
    #[inline(always)]
    fn from(variant: DmaSeq1BuserrorStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_SEQ1_BUSERROR_STOP` reader - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
pub type DmaSeq1BuserrorStopR = crate::BitReader<DmaSeq1BuserrorStop>;
impl DmaSeq1BuserrorStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaSeq1BuserrorStop {
        match self.bits {
            false => DmaSeq1BuserrorStop::Disabled,
            true => DmaSeq1BuserrorStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaSeq1BuserrorStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaSeq1BuserrorStop::Enabled
    }
}
#[doc = "Field `DMA_SEQ1_BUSERROR_STOP` writer - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
pub type DmaSeq1BuserrorStopW<'a, REG> = crate::BitWriter<'a, REG, DmaSeq1BuserrorStop>;
impl<'a, REG> DmaSeq1BuserrorStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaSeq1BuserrorStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaSeq1BuserrorStop::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[n\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend0_stop(&self) -> Seqend0StopR {
        Seqend0StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[n\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend1_stop(&self) -> Seqend1StopR {
        Seqend1StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
    #[inline(always)]
    pub fn loopsdone_dma_seq0_start(&self) -> LoopsdoneDmaSeq0StartR {
        LoopsdoneDmaSeq0StartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
    #[inline(always)]
    pub fn loopsdone_dma_seq1_start(&self) -> LoopsdoneDmaSeq1StartR {
        LoopsdoneDmaSeq1StartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    pub fn loopsdone_stop(&self) -> LoopsdoneStopR {
        LoopsdoneStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event RAMUNDERFLOW and task STOP"]
    #[inline(always)]
    pub fn ramunderflow_stop(&self) -> RamunderflowStopR {
        RamunderflowStopR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
    #[inline(always)]
    pub fn dma_seq0_buserror_stop(&self) -> DmaSeq0BuserrorStopR {
        DmaSeq0BuserrorStopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
    #[inline(always)]
    pub fn dma_seq1_buserror_stop(&self) -> DmaSeq1BuserrorStopR {
        DmaSeq1BuserrorStopR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[n\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn seqend0_stop(&mut self) -> Seqend0StopW<ShortsSpec> {
        Seqend0StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[n\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn seqend1_stop(&mut self) -> Seqend1StopW<ShortsSpec> {
        Seqend1StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_dma_seq0_start(&mut self) -> LoopsdoneDmaSeq0StartW<ShortsSpec> {
        LoopsdoneDmaSeq0StartW::new(self, 2)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task DMA.SEQ\\[n\\].START"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_dma_seq1_start(&mut self) -> LoopsdoneDmaSeq1StartW<ShortsSpec> {
        LoopsdoneDmaSeq1StartW::new(self, 3)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_stop(&mut self) -> LoopsdoneStopW<ShortsSpec> {
        LoopsdoneStopW::new(self, 4)
    }
    #[doc = "Bit 5 - Shortcut between event RAMUNDERFLOW and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn ramunderflow_stop(&mut self) -> RamunderflowStopW<ShortsSpec> {
        RamunderflowStopW::new(self, 5)
    }
    #[doc = "Bit 6 - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dma_seq0_buserror_stop(&mut self) -> DmaSeq0BuserrorStopW<ShortsSpec> {
        DmaSeq0BuserrorStopW::new(self, 6)
    }
    #[doc = "Bit 7 - Shortcut between event DMA.SEQ\\[n\\].BUSERROR and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dma_seq1_buserror_stop(&mut self) -> DmaSeq1BuserrorStopW<ShortsSpec> {
        DmaSeq1BuserrorStopW::new(self, 7)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {
    const RESET_VALUE: u32 = 0;
}
