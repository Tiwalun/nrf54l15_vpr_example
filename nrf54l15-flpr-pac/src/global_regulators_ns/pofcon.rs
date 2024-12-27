#[doc = "Register `POFCON` reader"]
pub type R = crate::R<PofconSpec>;
#[doc = "Register `POFCON` writer"]
pub type W = crate::W<PofconSpec>;
#[doc = "Enable or disable power-fail comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pof {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pof> for bool {
    #[inline(always)]
    fn from(variant: Pof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POF` reader - Enable or disable power-fail comparator"]
pub type PofR = crate::BitReader<Pof>;
impl PofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pof {
        match self.bits {
            false => Pof::Disabled,
            true => Pof::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pof::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pof::Enabled
    }
}
#[doc = "Field `POF` writer - Enable or disable power-fail comparator"]
pub type PofW<'a, REG> = crate::BitWriter<'a, REG, Pof>;
impl<'a, REG> PofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Enabled)
    }
}
#[doc = "Power-fail comparator threshold setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Threshold {
    #[doc = "0: Set threshold to 1.7 V"]
    V17 = 0,
    #[doc = "1: Set threshold to 1.8 V"]
    V18 = 1,
    #[doc = "2: Set threshold to 1.9 V"]
    V19 = 2,
    #[doc = "3: Set threshold to 2.0 V"]
    V20 = 3,
    #[doc = "4: Set threshold to 2.1 V"]
    V21 = 4,
    #[doc = "5: Set threshold to 2.2 V"]
    V22 = 5,
    #[doc = "6: Set threshold to 2.3 V"]
    V23 = 6,
    #[doc = "7: Set threshold to 2.4 V"]
    V24 = 7,
    #[doc = "8: Set threshold to 2.5 V"]
    V25 = 8,
    #[doc = "9: Set threshold to 2.6 V"]
    V26 = 9,
    #[doc = "10: Set threshold to 2.7 V"]
    V27 = 10,
    #[doc = "11: Set threshold to 2.8 V"]
    V28 = 11,
}
impl From<Threshold> for u8 {
    #[inline(always)]
    fn from(variant: Threshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Threshold {
    type Ux = u8;
}
impl crate::IsEnum for Threshold {}
#[doc = "Field `THRESHOLD` reader - Power-fail comparator threshold setting"]
pub type ThresholdR = crate::FieldReader<Threshold>;
impl ThresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Threshold> {
        match self.bits {
            0 => Some(Threshold::V17),
            1 => Some(Threshold::V18),
            2 => Some(Threshold::V19),
            3 => Some(Threshold::V20),
            4 => Some(Threshold::V21),
            5 => Some(Threshold::V22),
            6 => Some(Threshold::V23),
            7 => Some(Threshold::V24),
            8 => Some(Threshold::V25),
            9 => Some(Threshold::V26),
            10 => Some(Threshold::V27),
            11 => Some(Threshold::V28),
            _ => None,
        }
    }
    #[doc = "Set threshold to 1.7 V"]
    #[inline(always)]
    pub fn is_v17(&self) -> bool {
        *self == Threshold::V17
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == Threshold::V18
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn is_v19(&self) -> bool {
        *self == Threshold::V19
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn is_v20(&self) -> bool {
        *self == Threshold::V20
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == Threshold::V21
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn is_v22(&self) -> bool {
        *self == Threshold::V22
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == Threshold::V23
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn is_v24(&self) -> bool {
        *self == Threshold::V24
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == Threshold::V25
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn is_v26(&self) -> bool {
        *self == Threshold::V26
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == Threshold::V27
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == Threshold::V28
    }
}
#[doc = "Field `THRESHOLD` writer - Power-fail comparator threshold setting"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Threshold>;
impl<'a, REG> ThresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set threshold to 1.7 V"]
    #[inline(always)]
    pub fn v17(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V17)
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V18)
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn v19(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn v20(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn v21(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn v22(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn v23(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn v24(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn v26(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut crate::W<REG> {
        self.variant(Threshold::V28)
    }
}
#[doc = "Disable the POFWARN power-fail warning event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eventdisable {
    #[doc = "0: POFWARN event is generated"]
    Enabled = 0,
    #[doc = "1: POFWARN event is not generated"]
    Disabled = 1,
}
impl From<Eventdisable> for bool {
    #[inline(always)]
    fn from(variant: Eventdisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTDISABLE` reader - Disable the POFWARN power-fail warning event"]
pub type EventdisableR = crate::BitReader<Eventdisable>;
impl EventdisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eventdisable {
        match self.bits {
            false => Eventdisable::Enabled,
            true => Eventdisable::Disabled,
        }
    }
    #[doc = "POFWARN event is generated"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eventdisable::Enabled
    }
    #[doc = "POFWARN event is not generated"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eventdisable::Disabled
    }
}
#[doc = "Field `EVENTDISABLE` writer - Disable the POFWARN power-fail warning event"]
pub type EventdisableW<'a, REG> = crate::BitWriter<'a, REG, Eventdisable>;
impl<'a, REG> EventdisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POFWARN event is generated"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eventdisable::Enabled)
    }
    #[doc = "POFWARN event is not generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eventdisable::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable power-fail comparator"]
    #[inline(always)]
    pub fn pof(&self) -> PofR {
        PofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Disable the POFWARN power-fail warning event"]
    #[inline(always)]
    pub fn eventdisable(&self) -> EventdisableR {
        EventdisableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable power-fail comparator"]
    #[inline(always)]
    #[must_use]
    pub fn pof(&mut self) -> PofW<PofconSpec> {
        PofW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<PofconSpec> {
        ThresholdW::new(self, 1)
    }
    #[doc = "Bit 7 - Disable the POFWARN power-fail warning event"]
    #[inline(always)]
    #[must_use]
    pub fn eventdisable(&mut self) -> EventdisableW<PofconSpec> {
        EventdisableW::new(self, 7)
    }
}
#[doc = "Power-fail comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pofcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pofcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PofconSpec;
impl crate::RegisterSpec for PofconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pofcon::R`](R) reader structure"]
impl crate::Readable for PofconSpec {}
#[doc = "`write(|w| ..)` method takes [`pofcon::W`](W) writer structure"]
impl crate::Writable for PofconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POFCON to value 0"]
impl crate::Resettable for PofconSpec {
    const RESET_VALUE: u32 = 0;
}
