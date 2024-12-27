#[doc = "Register `CPURUN` reader"]
pub type R = crate::R<CpurunSpec>;
#[doc = "Register `CPURUN` writer"]
pub type W = crate::W<CpurunSpec>;
#[doc = "Controls CPU running state after a core reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: CPU stopped. If this is the CPU state after a core reset, setting this bit will change the CPU state to CPU running."]
    Stopped = 0,
    #[doc = "1: CPU running. If this is the CPU state after a core reset, clearing this bit will change the CPU state to CPU stopped after a core reset."]
    Running = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Controls CPU running state after a core reset."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Stopped,
            true => En::Running,
        }
    }
    #[doc = "CPU stopped. If this is the CPU state after a core reset, setting this bit will change the CPU state to CPU running."]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == En::Stopped
    }
    #[doc = "CPU running. If this is the CPU state after a core reset, clearing this bit will change the CPU state to CPU stopped after a core reset."]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == En::Running
    }
}
#[doc = "Field `EN` writer - Controls CPU running state after a core reset."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU stopped. If this is the CPU state after a core reset, setting this bit will change the CPU state to CPU running."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(En::Stopped)
    }
    #[doc = "CPU running. If this is the CPU state after a core reset, clearing this bit will change the CPU state to CPU stopped after a core reset."]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(En::Running)
    }
}
impl R {
    #[doc = "Bit 0 - Controls CPU running state after a core reset."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls CPU running state after a core reset."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CpurunSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "State of the CPU after a core reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpurun::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpurun::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpurunSpec;
impl crate::RegisterSpec for CpurunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpurun::R`](R) reader structure"]
impl crate::Readable for CpurunSpec {}
#[doc = "`write(|w| ..)` method takes [`cpurun::W`](W) writer structure"]
impl crate::Writable for CpurunSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPURUN to value 0"]
impl crate::Resettable for CpurunSpec {
    const RESET_VALUE: u32 = 0;
}
