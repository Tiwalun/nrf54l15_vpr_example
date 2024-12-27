#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `ACCESSTIMEOUT` reader - Access timeout, in 31.25 ns units, used for going into standby power mode or remain active on wake up"]
pub type AccesstimeoutR = crate::FieldReader<u16>;
#[doc = "Field `ACCESSTIMEOUT` writer - Access timeout, in 31.25 ns units, used for going into standby power mode or remain active on wake up"]
pub type AccesstimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Power on failure warning handling configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pof {
    #[doc = "0: Wait until the current RRAM write finishes"]
    Wait = 0,
    #[doc = "1: Abort the current RRAM write"]
    Abort = 1,
}
impl From<Pof> for bool {
    #[inline(always)]
    fn from(variant: Pof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POF` reader - Power on failure warning handling configuration"]
pub type PofR = crate::BitReader<Pof>;
impl PofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pof {
        match self.bits {
            false => Pof::Wait,
            true => Pof::Abort,
        }
    }
    #[doc = "Wait until the current RRAM write finishes"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Pof::Wait
    }
    #[doc = "Abort the current RRAM write"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == Pof::Abort
    }
}
#[doc = "Field `POF` writer - Power on failure warning handling configuration"]
pub type PofW<'a, REG> = crate::BitWriter<'a, REG, Pof>;
impl<'a, REG> PofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wait until the current RRAM write finishes"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Wait)
    }
    #[doc = "Abort the current RRAM write"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Abort)
    }
}
impl R {
    #[doc = "Bits 0:15 - Access timeout, in 31.25 ns units, used for going into standby power mode or remain active on wake up"]
    #[inline(always)]
    pub fn accesstimeout(&self) -> AccesstimeoutR {
        AccesstimeoutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Power on failure warning handling configuration"]
    #[inline(always)]
    pub fn pof(&self) -> PofR {
        PofR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Access timeout, in 31.25 ns units, used for going into standby power mode or remain active on wake up"]
    #[inline(always)]
    pub fn accesstimeout(&mut self) -> AccesstimeoutW<ConfigSpec> {
        AccesstimeoutW::new(self, 0)
    }
    #[doc = "Bit 16 - Power on failure warning handling configuration"]
    #[inline(always)]
    pub fn pof(&mut self) -> PofW<ConfigSpec> {
        PofW::new(self, 16)
    }
}
#[doc = "Power configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONFIG to value 0x0100"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0100;
}
