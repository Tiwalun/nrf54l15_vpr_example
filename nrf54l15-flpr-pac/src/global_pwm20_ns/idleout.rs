#[doc = "Register `IDLEOUT` reader"]
pub type R = crate::R<IdleoutSpec>;
#[doc = "Register `IDLEOUT` writer"]
pub type W = crate::W<IdleoutSpec>;
#[doc = "Field `VAL_0` reader - Idle output value for PWM channel \\[0\\]"]
pub type Val0R = crate::BitReader;
#[doc = "Field `VAL_0` writer - Idle output value for PWM channel \\[0\\]"]
pub type Val0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAL_1` reader - Idle output value for PWM channel \\[1\\]"]
pub type Val1R = crate::BitReader;
#[doc = "Field `VAL_1` writer - Idle output value for PWM channel \\[1\\]"]
pub type Val1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAL_2` reader - Idle output value for PWM channel \\[2\\]"]
pub type Val2R = crate::BitReader;
#[doc = "Field `VAL_2` writer - Idle output value for PWM channel \\[2\\]"]
pub type Val2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VAL_3` reader - Idle output value for PWM channel \\[3\\]"]
pub type Val3R = crate::BitReader;
#[doc = "Field `VAL_3` writer - Idle output value for PWM channel \\[3\\]"]
pub type Val3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Idle output value for PWM channel \\[0\\]"]
    #[inline(always)]
    pub fn val_0(&self) -> Val0R {
        Val0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Idle output value for PWM channel \\[1\\]"]
    #[inline(always)]
    pub fn val_1(&self) -> Val1R {
        Val1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle output value for PWM channel \\[2\\]"]
    #[inline(always)]
    pub fn val_2(&self) -> Val2R {
        Val2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Idle output value for PWM channel \\[3\\]"]
    #[inline(always)]
    pub fn val_3(&self) -> Val3R {
        Val3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Idle output value for PWM channel \\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val_0(&mut self) -> Val0W<IdleoutSpec> {
        Val0W::new(self, 0)
    }
    #[doc = "Bit 1 - Idle output value for PWM channel \\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val_1(&mut self) -> Val1W<IdleoutSpec> {
        Val1W::new(self, 1)
    }
    #[doc = "Bit 2 - Idle output value for PWM channel \\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val_2(&mut self) -> Val2W<IdleoutSpec> {
        Val2W::new(self, 2)
    }
    #[doc = "Bit 3 - Idle output value for PWM channel \\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn val_3(&mut self) -> Val3W<IdleoutSpec> {
        Val3W::new(self, 3)
    }
}
#[doc = "Configure the output value on the PWM channel during idle\n\nYou can [`read`](crate::Reg::read) this register and get [`idleout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idleout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleoutSpec;
impl crate::RegisterSpec for IdleoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idleout::R`](R) reader structure"]
impl crate::Readable for IdleoutSpec {}
#[doc = "`write(|w| ..)` method takes [`idleout::W`](W) writer structure"]
impl crate::Writable for IdleoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLEOUT to value 0"]
impl crate::Resettable for IdleoutSpec {
    const RESET_VALUE: u32 = 0;
}
