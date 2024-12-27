#[doc = "Register `EVENTS_ECCERROR` reader"]
pub type R = crate::R<EventsEccerrorSpec>;
#[doc = "Register `EVENTS_ECCERROR` writer"]
pub type W = crate::W<EventsEccerrorSpec>;
#[doc = "Uncorrectable ECC error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsEccerror {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsEccerror> for bool {
    #[inline(always)]
    fn from(variant: EventsEccerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_ECCERROR` reader - Uncorrectable ECC error detected"]
pub type EventsEccerrorR = crate::BitReader<EventsEccerror>;
impl EventsEccerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsEccerror {
        match self.bits {
            false => EventsEccerror::NotGenerated,
            true => EventsEccerror::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsEccerror::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsEccerror::Generated
    }
}
#[doc = "Field `EVENTS_ECCERROR` writer - Uncorrectable ECC error detected"]
pub type EventsEccerrorW<'a, REG> = crate::BitWriter<'a, REG, EventsEccerror>;
impl<'a, REG> EventsEccerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEccerror::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsEccerror::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Uncorrectable ECC error detected"]
    #[inline(always)]
    pub fn events_eccerror(&self) -> EventsEccerrorR {
        EventsEccerrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Uncorrectable ECC error detected"]
    #[inline(always)]
    pub fn events_eccerror(&mut self) -> EventsEccerrorW<EventsEccerrorSpec> {
        EventsEccerrorW::new(self, 0)
    }
}
#[doc = "Uncorrectable ECC error detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_eccerror::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_eccerror::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsEccerrorSpec;
impl crate::RegisterSpec for EventsEccerrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_eccerror::R`](R) reader structure"]
impl crate::Readable for EventsEccerrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_eccerror::W`](W) writer structure"]
impl crate::Writable for EventsEccerrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_ECCERROR to value 0"]
impl crate::Resettable for EventsEccerrorSpec {
    const RESET_VALUE: u32 = 0;
}
