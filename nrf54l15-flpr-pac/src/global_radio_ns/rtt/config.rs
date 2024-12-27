#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Enable RTT Functionality. Only valid for BLE 1MBPS and 2MBPS mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable RTT Block"]
    Disabled = 0,
    #[doc = "1: Enable RTT Block"]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable RTT Functionality. Only valid for BLE 1MBPS and 2MBPS mode"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "Disable RTT Block"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Enable RTT Block"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - Enable RTT Functionality. Only valid for BLE 1MBPS and 2MBPS mode"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable RTT Block"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Enable RTT Block"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
#[doc = "Enabling/Disable ping over the entire access address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enfullaa {
    #[doc = "0: Disable ping over the entire access address, i.e., enable only over the first 16-bit access address"]
    Disabled = 0,
    #[doc = "1: Enable ping over the entire access address"]
    Enabled = 1,
}
impl From<Enfullaa> for bool {
    #[inline(always)]
    fn from(variant: Enfullaa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENFULLAA` reader - Enabling/Disable ping over the entire access address."]
pub type EnfullaaR = crate::BitReader<Enfullaa>;
impl EnfullaaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enfullaa {
        match self.bits {
            false => Enfullaa::Disabled,
            true => Enfullaa::Enabled,
        }
    }
    #[doc = "Disable ping over the entire access address, i.e., enable only over the first 16-bit access address"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enfullaa::Disabled
    }
    #[doc = "Enable ping over the entire access address"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enfullaa::Enabled
    }
}
#[doc = "Field `ENFULLAA` writer - Enabling/Disable ping over the entire access address."]
pub type EnfullaaW<'a, REG> = crate::BitWriter<'a, REG, Enfullaa>;
impl<'a, REG> EnfullaaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ping over the entire access address, i.e., enable only over the first 16-bit access address"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enfullaa::Disabled)
    }
    #[doc = "Enable ping over the entire access address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enfullaa::Enabled)
    }
}
#[doc = "Role as a Initiator or Reflector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    #[doc = "0: Initiator"]
    Initiator = 0,
    #[doc = "1: Reflector"]
    Reflector = 1,
}
impl From<Role> for bool {
    #[inline(always)]
    fn from(variant: Role) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROLE` reader - Role as a Initiator or Reflector."]
pub type RoleR = crate::BitReader<Role>;
impl RoleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Role {
        match self.bits {
            false => Role::Initiator,
            true => Role::Reflector,
        }
    }
    #[doc = "Initiator"]
    #[inline(always)]
    pub fn is_initiator(&self) -> bool {
        *self == Role::Initiator
    }
    #[doc = "Reflector"]
    #[inline(always)]
    pub fn is_reflector(&self) -> bool {
        *self == Role::Reflector
    }
}
#[doc = "Field `ROLE` writer - Role as a Initiator or Reflector."]
pub type RoleW<'a, REG> = crate::BitWriter<'a, REG, Role>;
impl<'a, REG> RoleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initiator"]
    #[inline(always)]
    pub fn initiator(self) -> &'a mut crate::W<REG> {
        self.variant(Role::Initiator)
    }
    #[doc = "Reflector"]
    #[inline(always)]
    pub fn reflector(self) -> &'a mut crate::W<REG> {
        self.variant(Role::Reflector)
    }
}
#[doc = "Field `NUMSEGMENTS` reader - Number of 16bit payload segments available for ToA detection. Allowed values are 0, 2, 4, 6 and 8."]
pub type NumsegmentsR = crate::FieldReader;
#[doc = "Field `NUMSEGMENTS` writer - Number of 16bit payload segments available for ToA detection. Allowed values are 0, 2, 4, 6 and 8."]
pub type NumsegmentsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EFSDELAY` reader - Early Frame Sync Delay, i.e., number of cycles to wait for access address to anchor correctly. For 2MBPSBLE mode, the EFSDELAY value is 64 (2us) and for 1MBPSBLE mode, it can be 256 (8us)."]
pub type EfsdelayR = crate::FieldReader<u16>;
#[doc = "Field `EFSDELAY` writer - Early Frame Sync Delay, i.e., number of cycles to wait for access address to anchor correctly. For 2MBPSBLE mode, the EFSDELAY value is 64 (2us) and for 1MBPSBLE mode, it can be 256 (8us)."]
pub type EfsdelayW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - Enable RTT Functionality. Only valid for BLE 1MBPS and 2MBPS mode"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enabling/Disable ping over the entire access address."]
    #[inline(always)]
    pub fn enfullaa(&self) -> EnfullaaR {
        EnfullaaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Role as a Initiator or Reflector."]
    #[inline(always)]
    pub fn role(&self) -> RoleR {
        RoleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Number of 16bit payload segments available for ToA detection. Allowed values are 0, 2, 4, 6 and 8."]
    #[inline(always)]
    pub fn numsegments(&self) -> NumsegmentsR {
        NumsegmentsR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 8:16 - Early Frame Sync Delay, i.e., number of cycles to wait for access address to anchor correctly. For 2MBPSBLE mode, the EFSDELAY value is 64 (2us) and for 1MBPSBLE mode, it can be 256 (8us)."]
    #[inline(always)]
    pub fn efsdelay(&self) -> EfsdelayR {
        EfsdelayR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RTT Functionality. Only valid for BLE 1MBPS and 2MBPS mode"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ConfigSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enabling/Disable ping over the entire access address."]
    #[inline(always)]
    #[must_use]
    pub fn enfullaa(&mut self) -> EnfullaaW<ConfigSpec> {
        EnfullaaW::new(self, 1)
    }
    #[doc = "Bit 2 - Role as a Initiator or Reflector."]
    #[inline(always)]
    #[must_use]
    pub fn role(&mut self) -> RoleW<ConfigSpec> {
        RoleW::new(self, 2)
    }
    #[doc = "Bits 3:6 - Number of 16bit payload segments available for ToA detection. Allowed values are 0, 2, 4, 6 and 8."]
    #[inline(always)]
    #[must_use]
    pub fn numsegments(&mut self) -> NumsegmentsW<ConfigSpec> {
        NumsegmentsW::new(self, 3)
    }
    #[doc = "Bits 8:16 - Early Frame Sync Delay, i.e., number of cycles to wait for access address to anchor correctly. For 2MBPSBLE mode, the EFSDELAY value is 64 (2us) and for 1MBPSBLE mode, it can be 256 (8us)."]
    #[inline(always)]
    #[must_use]
    pub fn efsdelay(&mut self) -> EfsdelayW<ConfigSpec> {
        EfsdelayW::new(self, 8)
    }
}
#[doc = "RTT Config.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
