#[doc = "Register `EVENTS_TAMPER` reader"]
pub type R = crate::R<EventsTamperSpec>;
#[doc = "Register `EVENTS_TAMPER` writer"]
pub type W = crate::W<EventsTamperSpec>;
#[doc = "Tamper controller detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTamper {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTamper> for bool {
    #[inline(always)]
    fn from(variant: EventsTamper) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TAMPER` reader - Tamper controller detected an error."]
pub type EventsTamperR = crate::BitReader<EventsTamper>;
impl EventsTamperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTamper {
        match self.bits {
            false => EventsTamper::NotGenerated,
            true => EventsTamper::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTamper::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTamper::Generated
    }
}
#[doc = "Field `EVENTS_TAMPER` writer - Tamper controller detected an error."]
pub type EventsTamperW<'a, REG> = crate::BitWriter<'a, REG, EventsTamper>;
impl<'a, REG> EventsTamperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTamper::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTamper::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper controller detected an error."]
    #[inline(always)]
    pub fn events_tamper(&self) -> EventsTamperR {
        EventsTamperR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper controller detected an error."]
    #[inline(always)]
    #[must_use]
    pub fn events_tamper(&mut self) -> EventsTamperW<EventsTamperSpec> {
        EventsTamperW::new(self, 0)
    }
}
#[doc = "Tamper controller detected an error.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_tamper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_tamper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTamperSpec;
impl crate::RegisterSpec for EventsTamperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_tamper::R`](R) reader structure"]
impl crate::Readable for EventsTamperSpec {}
#[doc = "`write(|w| ..)` method takes [`events_tamper::W`](W) writer structure"]
impl crate::Writable for EventsTamperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_TAMPER to value 0"]
impl crate::Resettable for EventsTamperSpec {
    const RESET_VALUE: u32 = 0;
}
