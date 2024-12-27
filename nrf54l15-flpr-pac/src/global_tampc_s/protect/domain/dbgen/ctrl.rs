#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Set value of dbgen signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    #[doc = "0: Signal is logic 0."]
    Low = 0,
    #[doc = "1: Signal is logic 1."]
    High = 1,
}
impl From<Value> for bool {
    #[inline(always)]
    fn from(variant: Value) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALUE` reader - Set value of dbgen signal."]
pub type ValueR = crate::BitReader<Value>;
impl ValueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Value {
        match self.bits {
            false => Value::Low,
            true => Value::High,
        }
    }
    #[doc = "Signal is logic 0."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Value::Low
    }
    #[doc = "Signal is logic 1."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Value::High
    }
}
#[doc = "Field `VALUE` writer - Set value of dbgen signal."]
pub type ValueW<'a, REG> = crate::BitWriter<'a, REG, Value>;
impl<'a, REG> ValueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Signal is logic 0."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Value::Low)
    }
    #[doc = "Signal is logic 1."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Value::High)
    }
}
#[doc = "Lock this register to prevent changes to the VALUE field until next reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Lock disabled."]
    Disabled = 0,
    #[doc = "1: Lock enabled."]
    Enabled = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` writer - Lock this register to prevent changes to the VALUE field until next reset."]
pub type LockW<'a, REG> = crate::BitWriter1S<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Disabled)
    }
    #[doc = "Lock enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Enabled)
    }
}
#[doc = "The write protection must be cleared to allow updates to the VALUE field.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Writeprotection {
    #[doc = "0: Read: Write protection is disabled."]
    Disabled = 0,
    #[doc = "1: Read: Write protection is enabled."]
    Enabled = 1,
    #[doc = "15: Write: Value to clear write protection."]
    Clear = 15,
}
impl From<Writeprotection> for u8 {
    #[inline(always)]
    fn from(variant: Writeprotection) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Writeprotection {
    type Ux = u8;
}
impl crate::IsEnum for Writeprotection {}
#[doc = "Field `WRITEPROTECTION` reader - The write protection must be cleared to allow updates to the VALUE field."]
pub type WriteprotectionR = crate::FieldReader<Writeprotection>;
impl WriteprotectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Writeprotection> {
        match self.bits {
            0 => Some(Writeprotection::Disabled),
            1 => Some(Writeprotection::Enabled),
            15 => Some(Writeprotection::Clear),
            _ => None,
        }
    }
    #[doc = "Read: Write protection is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Writeprotection::Disabled
    }
    #[doc = "Read: Write protection is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Writeprotection::Enabled
    }
    #[doc = "Write: Value to clear write protection."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Writeprotection::Clear
    }
}
#[doc = "Field `WRITEPROTECTION` writer - The write protection must be cleared to allow updates to the VALUE field."]
pub type WriteprotectionW<'a, REG> = crate::FieldWriter<'a, REG, 4, Writeprotection>;
impl<'a, REG> WriteprotectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read: Write protection is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Writeprotection::Disabled)
    }
    #[doc = "Read: Write protection is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Writeprotection::Enabled)
    }
    #[doc = "Write: Value to clear write protection."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Writeprotection::Clear)
    }
}
#[doc = "Required write key for upper 16 bits. Must be included in all register write operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    #[doc = "20730: Write key value."]
    Key = 20730,
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u16;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Required write key for upper 16 bits. Must be included in all register write operations."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Write key value."]
    #[inline(always)]
    pub fn key(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Key)
    }
}
impl R {
    #[doc = "Bit 0 - Set value of dbgen signal."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - The write protection must be cleared to allow updates to the VALUE field."]
    #[inline(always)]
    pub fn writeprotection(&self) -> WriteprotectionR {
        WriteprotectionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set value of dbgen signal."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<CtrlSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 1 - Lock this register to prevent changes to the VALUE field until next reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<CtrlSpec> {
        LockW::new(self, 1)
    }
    #[doc = "Bits 4:7 - The write protection must be cleared to allow updates to the VALUE field."]
    #[inline(always)]
    #[must_use]
    pub fn writeprotection(&mut self) -> WriteprotectionW<CtrlSpec> {
        WriteprotectionW::new(self, 4)
    }
    #[doc = "Bits 16:31 - Required write key for upper 16 bits. Must be included in all register write operations."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CtrlSpec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Description cluster: Control register for invasive (halting) debug enable for the local debug components within domain n.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets CTRL to value 0x10"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
