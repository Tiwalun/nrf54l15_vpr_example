#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event LASTTX and task DMA.RX.START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LasttxDmaRxStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LasttxDmaRxStart> for bool {
    #[inline(always)]
    fn from(variant: LasttxDmaRxStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_DMA_RX_START` reader - Shortcut between event LASTTX and task DMA.RX.START"]
pub type LasttxDmaRxStartR = crate::BitReader<LasttxDmaRxStart>;
impl LasttxDmaRxStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LasttxDmaRxStart {
        match self.bits {
            false => LasttxDmaRxStart::Disabled,
            true => LasttxDmaRxStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LasttxDmaRxStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LasttxDmaRxStart::Enabled
    }
}
#[doc = "Field `LASTTX_DMA_RX_START` writer - Shortcut between event LASTTX and task DMA.RX.START"]
pub type LasttxDmaRxStartW<'a, REG> = crate::BitWriter<'a, REG, LasttxDmaRxStart>;
impl<'a, REG> LasttxDmaRxStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxDmaRxStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxDmaRxStart::Enabled)
    }
}
#[doc = "Shortcut between event LASTTX and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LasttxSuspend {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LasttxSuspend> for bool {
    #[inline(always)]
    fn from(variant: LasttxSuspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_SUSPEND` reader - Shortcut between event LASTTX and task SUSPEND"]
pub type LasttxSuspendR = crate::BitReader<LasttxSuspend>;
impl LasttxSuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LasttxSuspend {
        match self.bits {
            false => LasttxSuspend::Disabled,
            true => LasttxSuspend::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LasttxSuspend::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LasttxSuspend::Enabled
    }
}
#[doc = "Field `LASTTX_SUSPEND` writer - Shortcut between event LASTTX and task SUSPEND"]
pub type LasttxSuspendW<'a, REG> = crate::BitWriter<'a, REG, LasttxSuspend>;
impl<'a, REG> LasttxSuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxSuspend::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxSuspend::Enabled)
    }
}
#[doc = "Shortcut between event LASTTX and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LasttxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LasttxStop> for bool {
    #[inline(always)]
    fn from(variant: LasttxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTTX_STOP` reader - Shortcut between event LASTTX and task STOP"]
pub type LasttxStopR = crate::BitReader<LasttxStop>;
impl LasttxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LasttxStop {
        match self.bits {
            false => LasttxStop::Disabled,
            true => LasttxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LasttxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LasttxStop::Enabled
    }
}
#[doc = "Field `LASTTX_STOP` writer - Shortcut between event LASTTX and task STOP"]
pub type LasttxStopW<'a, REG> = crate::BitWriter<'a, REG, LasttxStop>;
impl<'a, REG> LasttxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LasttxStop::Enabled)
    }
}
#[doc = "Shortcut between event LASTRX and task DMA.TX.START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastrxDmaTxStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LastrxDmaTxStart> for bool {
    #[inline(always)]
    fn from(variant: LastrxDmaTxStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_DMA_TX_START` reader - Shortcut between event LASTRX and task DMA.TX.START"]
pub type LastrxDmaTxStartR = crate::BitReader<LastrxDmaTxStart>;
impl LastrxDmaTxStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastrxDmaTxStart {
        match self.bits {
            false => LastrxDmaTxStart::Disabled,
            true => LastrxDmaTxStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LastrxDmaTxStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LastrxDmaTxStart::Enabled
    }
}
#[doc = "Field `LASTRX_DMA_TX_START` writer - Shortcut between event LASTRX and task DMA.TX.START"]
pub type LastrxDmaTxStartW<'a, REG> = crate::BitWriter<'a, REG, LastrxDmaTxStart>;
impl<'a, REG> LastrxDmaTxStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxDmaTxStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxDmaTxStart::Enabled)
    }
}
#[doc = "Shortcut between event LASTRX and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastrxStop {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<LastrxStop> for bool {
    #[inline(always)]
    fn from(variant: LastrxStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTRX_STOP` reader - Shortcut between event LASTRX and task STOP"]
pub type LastrxStopR = crate::BitReader<LastrxStop>;
impl LastrxStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastrxStop {
        match self.bits {
            false => LastrxStop::Disabled,
            true => LastrxStop::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LastrxStop::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LastrxStop::Enabled
    }
}
#[doc = "Field `LASTRX_STOP` writer - Shortcut between event LASTRX and task STOP"]
pub type LastrxStopW<'a, REG> = crate::BitWriter<'a, REG, LastrxStop>;
impl<'a, REG> LastrxStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxStop::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LastrxStop::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[1\\]
Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch0DmaRxEnablematch1 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch0DmaRxEnablematch1> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch0DmaRxEnablematch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_ENABLEMATCH1` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[1\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch0DmaRxEnablematch1R = crate::BitReader<DmaRxMatch0DmaRxEnablematch1>;
impl DmaRxMatch0DmaRxEnablematch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch0DmaRxEnablematch1 {
        match self.bits {
            false => DmaRxMatch0DmaRxEnablematch1::Disabled,
            true => DmaRxMatch0DmaRxEnablematch1::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxEnablematch1::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxEnablematch1::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_ENABLEMATCH1` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[1\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch0DmaRxEnablematch1W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch0DmaRxEnablematch1>;
impl<'a, REG> DmaRxMatch0DmaRxEnablematch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxEnablematch1::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxEnablematch1::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[2\\]
Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch1DmaRxEnablematch2 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch1DmaRxEnablematch2> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch1DmaRxEnablematch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_ENABLEMATCH2` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[2\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch1DmaRxEnablematch2R = crate::BitReader<DmaRxMatch1DmaRxEnablematch2>;
impl DmaRxMatch1DmaRxEnablematch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch1DmaRxEnablematch2 {
        match self.bits {
            false => DmaRxMatch1DmaRxEnablematch2::Disabled,
            true => DmaRxMatch1DmaRxEnablematch2::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxEnablematch2::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxEnablematch2::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_ENABLEMATCH2` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[2\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch1DmaRxEnablematch2W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch1DmaRxEnablematch2>;
impl<'a, REG> DmaRxMatch1DmaRxEnablematch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxEnablematch2::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxEnablematch2::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[3\\]
Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch2DmaRxEnablematch3 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch2DmaRxEnablematch3> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch2DmaRxEnablematch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_ENABLEMATCH3` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[3\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch2DmaRxEnablematch3R = crate::BitReader<DmaRxMatch2DmaRxEnablematch3>;
impl DmaRxMatch2DmaRxEnablematch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch2DmaRxEnablematch3 {
        match self.bits {
            false => DmaRxMatch2DmaRxEnablematch3::Disabled,
            true => DmaRxMatch2DmaRxEnablematch3::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxEnablematch3::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxEnablematch3::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_ENABLEMATCH3` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[3\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch2DmaRxEnablematch3W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch2DmaRxEnablematch3>;
impl<'a, REG> DmaRxMatch2DmaRxEnablematch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxEnablematch3::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxEnablematch3::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[0\\]
Allows daisy-chaining match events.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch3DmaRxEnablematch0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch3DmaRxEnablematch0> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch3DmaRxEnablematch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_ENABLEMATCH0` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[0\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch3DmaRxEnablematch0R = crate::BitReader<DmaRxMatch3DmaRxEnablematch0>;
impl DmaRxMatch3DmaRxEnablematch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch3DmaRxEnablematch0 {
        match self.bits {
            false => DmaRxMatch3DmaRxEnablematch0::Disabled,
            true => DmaRxMatch3DmaRxEnablematch0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxEnablematch0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxEnablematch0::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_ENABLEMATCH0` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[0\\]
Allows daisy-chaining match events."]
pub type DmaRxMatch3DmaRxEnablematch0W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch3DmaRxEnablematch0>;
impl<'a, REG> DmaRxMatch3DmaRxEnablematch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxEnablematch0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxEnablematch0::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch0DmaRxDisablematch0 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch0DmaRxDisablematch0> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch0DmaRxDisablematch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_DISABLEMATCH0` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch0DmaRxDisablematch0R = crate::BitReader<DmaRxMatch0DmaRxDisablematch0>;
impl DmaRxMatch0DmaRxDisablematch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch0DmaRxDisablematch0 {
        match self.bits {
            false => DmaRxMatch0DmaRxDisablematch0::Disabled,
            true => DmaRxMatch0DmaRxDisablematch0::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxDisablematch0::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch0DmaRxDisablematch0::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH0_DMA_RX_DISABLEMATCH0` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch0DmaRxDisablematch0W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch0DmaRxDisablematch0>;
impl<'a, REG> DmaRxMatch0DmaRxDisablematch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxDisablematch0::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch0DmaRxDisablematch0::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch1DmaRxDisablematch1 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch1DmaRxDisablematch1> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch1DmaRxDisablematch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_DISABLEMATCH1` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch1DmaRxDisablematch1R = crate::BitReader<DmaRxMatch1DmaRxDisablematch1>;
impl DmaRxMatch1DmaRxDisablematch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch1DmaRxDisablematch1 {
        match self.bits {
            false => DmaRxMatch1DmaRxDisablematch1::Disabled,
            true => DmaRxMatch1DmaRxDisablematch1::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxDisablematch1::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch1DmaRxDisablematch1::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH1_DMA_RX_DISABLEMATCH1` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch1DmaRxDisablematch1W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch1DmaRxDisablematch1>;
impl<'a, REG> DmaRxMatch1DmaRxDisablematch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxDisablematch1::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch1DmaRxDisablematch1::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch2DmaRxDisablematch2 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch2DmaRxDisablematch2> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch2DmaRxDisablematch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_DISABLEMATCH2` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch2DmaRxDisablematch2R = crate::BitReader<DmaRxMatch2DmaRxDisablematch2>;
impl DmaRxMatch2DmaRxDisablematch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch2DmaRxDisablematch2 {
        match self.bits {
            false => DmaRxMatch2DmaRxDisablematch2::Disabled,
            true => DmaRxMatch2DmaRxDisablematch2::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxDisablematch2::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch2DmaRxDisablematch2::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH2_DMA_RX_DISABLEMATCH2` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch2DmaRxDisablematch2W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch2DmaRxDisablematch2>;
impl<'a, REG> DmaRxMatch2DmaRxDisablematch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxDisablematch2::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch2DmaRxDisablematch2::Enabled)
    }
}
#[doc = "Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaRxMatch3DmaRxDisablematch3 {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DmaRxMatch3DmaRxDisablematch3> for bool {
    #[inline(always)]
    fn from(variant: DmaRxMatch3DmaRxDisablematch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_DISABLEMATCH3` reader - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch3DmaRxDisablematch3R = crate::BitReader<DmaRxMatch3DmaRxDisablematch3>;
impl DmaRxMatch3DmaRxDisablematch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaRxMatch3DmaRxDisablematch3 {
        match self.bits {
            false => DmaRxMatch3DmaRxDisablematch3::Disabled,
            true => DmaRxMatch3DmaRxDisablematch3::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxDisablematch3::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DmaRxMatch3DmaRxDisablematch3::Enabled
    }
}
#[doc = "Field `DMA_RX_MATCH3_DMA_RX_DISABLEMATCH3` writer - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
pub type DmaRxMatch3DmaRxDisablematch3W<'a, REG> =
    crate::BitWriter<'a, REG, DmaRxMatch3DmaRxDisablematch3>;
impl<'a, REG> DmaRxMatch3DmaRxDisablematch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxDisablematch3::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DmaRxMatch3DmaRxDisablematch3::Enabled)
    }
}
impl R {
    #[doc = "Bit 7 - Shortcut between event LASTTX and task DMA.RX.START"]
    #[inline(always)]
    pub fn lasttx_dma_rx_start(&self) -> LasttxDmaRxStartR {
        LasttxDmaRxStartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event LASTTX and task SUSPEND"]
    #[inline(always)]
    pub fn lasttx_suspend(&self) -> LasttxSuspendR {
        LasttxSuspendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shortcut between event LASTTX and task STOP"]
    #[inline(always)]
    pub fn lasttx_stop(&self) -> LasttxStopR {
        LasttxStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shortcut between event LASTRX and task DMA.TX.START"]
    #[inline(always)]
    pub fn lastrx_dma_tx_start(&self) -> LastrxDmaTxStartR {
        LastrxDmaTxStartR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Shortcut between event LASTRX and task STOP"]
    #[inline(always)]
    pub fn lastrx_stop(&self) -> LastrxStopR {
        LastrxStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[1\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_enablematch1(&self) -> DmaRxMatch0DmaRxEnablematch1R {
        DmaRxMatch0DmaRxEnablematch1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[2\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_enablematch2(&self) -> DmaRxMatch1DmaRxEnablematch2R {
        DmaRxMatch1DmaRxEnablematch2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[3\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_enablematch3(&self) -> DmaRxMatch2DmaRxEnablematch3R {
        DmaRxMatch2DmaRxEnablematch3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[0\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_enablematch0(&self) -> DmaRxMatch3DmaRxEnablematch0R {
        DmaRxMatch3DmaRxEnablematch0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_disablematch0(&self) -> DmaRxMatch0DmaRxDisablematch0R {
        DmaRxMatch0DmaRxDisablematch0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_disablematch1(&self) -> DmaRxMatch1DmaRxDisablematch1R {
        DmaRxMatch1DmaRxDisablematch1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_disablematch2(&self) -> DmaRxMatch2DmaRxDisablematch2R {
        DmaRxMatch2DmaRxDisablematch2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_disablematch3(&self) -> DmaRxMatch3DmaRxDisablematch3R {
        DmaRxMatch3DmaRxDisablematch3R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Shortcut between event LASTTX and task DMA.RX.START"]
    #[inline(always)]
    pub fn lasttx_dma_rx_start(&mut self) -> LasttxDmaRxStartW<ShortsSpec> {
        LasttxDmaRxStartW::new(self, 7)
    }
    #[doc = "Bit 8 - Shortcut between event LASTTX and task SUSPEND"]
    #[inline(always)]
    pub fn lasttx_suspend(&mut self) -> LasttxSuspendW<ShortsSpec> {
        LasttxSuspendW::new(self, 8)
    }
    #[doc = "Bit 9 - Shortcut between event LASTTX and task STOP"]
    #[inline(always)]
    pub fn lasttx_stop(&mut self) -> LasttxStopW<ShortsSpec> {
        LasttxStopW::new(self, 9)
    }
    #[doc = "Bit 10 - Shortcut between event LASTRX and task DMA.TX.START"]
    #[inline(always)]
    pub fn lastrx_dma_tx_start(&mut self) -> LastrxDmaTxStartW<ShortsSpec> {
        LastrxDmaTxStartW::new(self, 10)
    }
    #[doc = "Bit 12 - Shortcut between event LASTRX and task STOP"]
    #[inline(always)]
    pub fn lastrx_stop(&mut self) -> LastrxStopW<ShortsSpec> {
        LastrxStopW::new(self, 12)
    }
    #[doc = "Bit 21 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[1\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_enablematch1(
        &mut self,
    ) -> DmaRxMatch0DmaRxEnablematch1W<ShortsSpec> {
        DmaRxMatch0DmaRxEnablematch1W::new(self, 21)
    }
    #[doc = "Bit 22 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[2\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_enablematch2(
        &mut self,
    ) -> DmaRxMatch1DmaRxEnablematch2W<ShortsSpec> {
        DmaRxMatch1DmaRxEnablematch2W::new(self, 22)
    }
    #[doc = "Bit 23 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[3\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_enablematch3(
        &mut self,
    ) -> DmaRxMatch2DmaRxEnablematch3W<ShortsSpec> {
        DmaRxMatch2DmaRxEnablematch3W::new(self, 23)
    }
    #[doc = "Bit 24 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.ENABLEMATCH\\[0\\]
Allows daisy-chaining match events."]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_enablematch0(
        &mut self,
    ) -> DmaRxMatch3DmaRxEnablematch0W<ShortsSpec> {
        DmaRxMatch3DmaRxEnablematch0W::new(self, 24)
    }
    #[doc = "Bit 25 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match0_dma_rx_disablematch0(
        &mut self,
    ) -> DmaRxMatch0DmaRxDisablematch0W<ShortsSpec> {
        DmaRxMatch0DmaRxDisablematch0W::new(self, 25)
    }
    #[doc = "Bit 26 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match1_dma_rx_disablematch1(
        &mut self,
    ) -> DmaRxMatch1DmaRxDisablematch1W<ShortsSpec> {
        DmaRxMatch1DmaRxDisablematch1W::new(self, 26)
    }
    #[doc = "Bit 27 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match2_dma_rx_disablematch2(
        &mut self,
    ) -> DmaRxMatch2DmaRxDisablematch2W<ShortsSpec> {
        DmaRxMatch2DmaRxDisablematch2W::new(self, 27)
    }
    #[doc = "Bit 28 - Shortcut between event DMA.RX.MATCH\\[n\\]
and task DMA.RX.DISABLEMATCH\\[n\\]"]
    #[inline(always)]
    pub fn dma_rx_match3_dma_rx_disablematch3(
        &mut self,
    ) -> DmaRxMatch3DmaRxDisablematch3W<ShortsSpec> {
        DmaRxMatch3DmaRxDisablematch3W::new(self, 28)
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
