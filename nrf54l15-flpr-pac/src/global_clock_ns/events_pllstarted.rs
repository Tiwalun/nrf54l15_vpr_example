#[doc = "Register `EVENTS_PLLSTARTED` reader"]
pub type R = crate::R<EventsPllstartedSpec>;
#[doc = "Register `EVENTS_PLLSTARTED` writer"]
pub type W = crate::W<EventsPllstartedSpec>;
#[doc = "PLL started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPllstarted {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPllstarted> for bool {
    #[inline(always)]
    fn from(variant: EventsPllstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PLLSTARTED` reader - PLL started"]
pub type EventsPllstartedR = crate::BitReader<EventsPllstarted>;
impl EventsPllstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPllstarted {
        match self.bits {
            false => EventsPllstarted::NotGenerated,
            true => EventsPllstarted::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPllstarted::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPllstarted::Generated
    }
}
#[doc = "Field `EVENTS_PLLSTARTED` writer - PLL started"]
pub type EventsPllstartedW<'a, REG> = crate::BitWriter<'a, REG, EventsPllstarted>;
impl<'a, REG> EventsPllstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPllstarted::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPllstarted::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - PLL started"]
    #[inline(always)]
    pub fn events_pllstarted(&self) -> EventsPllstartedR {
        EventsPllstartedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL started"]
    #[inline(always)]
    #[must_use]
    pub fn events_pllstarted(&mut self) -> EventsPllstartedW<EventsPllstartedSpec> {
        EventsPllstartedW::new(self, 0)
    }
}
#[doc = "PLL started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pllstarted::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pllstarted::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPllstartedSpec;
impl crate::RegisterSpec for EventsPllstartedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pllstarted::R`](R) reader structure"]
impl crate::Readable for EventsPllstartedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pllstarted::W`](W) writer structure"]
impl crate::Writable for EventsPllstartedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_PLLSTARTED to value 0"]
impl crate::Resettable for EventsPllstartedSpec {
    const RESET_VALUE: u32 = 0;
}
