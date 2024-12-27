#[doc = "Register `INITWAITVAL` reader"]
pub type R = crate::R<InitwaitvalSpec>;
#[doc = "Register `INITWAITVAL` writer"]
pub type W = crate::W<InitwaitvalSpec>;
#[doc = "Field `INITWAITVAL` reader - Number of clock cycles to wait before sampling data from the noise source."]
pub type InitwaitvalR = crate::FieldReader<u16>;
#[doc = "Field `INITWAITVAL` writer - Number of clock cycles to wait before sampling data from the noise source."]
pub type InitwaitvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of clock cycles to wait before sampling data from the noise source."]
    #[inline(always)]
    pub fn initwaitval(&self) -> InitwaitvalR {
        InitwaitvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of clock cycles to wait before sampling data from the noise source."]
    #[inline(always)]
    #[must_use]
    pub fn initwaitval(&mut self) -> InitwaitvalW<InitwaitvalSpec> {
        InitwaitvalW::new(self, 0)
    }
}
#[doc = "Initial wait counter value.\n\nYou can [`read`](crate::Reg::read) this register and get [`initwaitval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initwaitval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitwaitvalSpec;
impl crate::RegisterSpec for InitwaitvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`initwaitval::R`](R) reader structure"]
impl crate::Readable for InitwaitvalSpec {}
#[doc = "`write(|w| ..)` method takes [`initwaitval::W`](W) writer structure"]
impl crate::Writable for InitwaitvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INITWAITVAL to value 0xffff"]
impl crate::Resettable for InitwaitvalSpec {
    const RESET_VALUE: u32 = 0xffff;
}
