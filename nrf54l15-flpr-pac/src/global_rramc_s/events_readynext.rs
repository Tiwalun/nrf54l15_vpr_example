#[doc = "Register `EVENTS_READYNEXT` reader"]
pub type R = crate::R<EventsReadynextSpec>;
#[doc = "Register `EVENTS_READYNEXT` writer"]
pub type W = crate::W<EventsReadynextSpec>;
#[doc = "Ready to accept a new write operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsReadynext {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsReadynext> for bool {
    #[inline(always)]
    fn from(variant: EventsReadynext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_READYNEXT` reader - Ready to accept a new write operation"]
pub type EventsReadynextR = crate::BitReader<EventsReadynext>;
impl EventsReadynextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsReadynext {
        match self.bits {
            false => EventsReadynext::NotGenerated,
            true => EventsReadynext::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsReadynext::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsReadynext::Generated
    }
}
#[doc = "Field `EVENTS_READYNEXT` writer - Ready to accept a new write operation"]
pub type EventsReadynextW<'a, REG> = crate::BitWriter<'a, REG, EventsReadynext>;
impl<'a, REG> EventsReadynextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReadynext::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReadynext::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Ready to accept a new write operation"]
    #[inline(always)]
    pub fn events_readynext(&self) -> EventsReadynextR {
        EventsReadynextR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready to accept a new write operation"]
    #[inline(always)]
    pub fn events_readynext(&mut self) -> EventsReadynextW<EventsReadynextSpec> {
        EventsReadynextW::new(self, 0)
    }
}
#[doc = "Ready to accept a new write operation\n\nYou can [`read`](crate::Reg::read) this register and get [`events_readynext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_readynext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReadynextSpec;
impl crate::RegisterSpec for EventsReadynextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_readynext::R`](R) reader structure"]
impl crate::Readable for EventsReadynextSpec {}
#[doc = "`write(|w| ..)` method takes [`events_readynext::W`](W) writer structure"]
impl crate::Writable for EventsReadynextSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_READYNEXT to value 0"]
impl crate::Resettable for EventsReadynextSpec {
    const RESET_VALUE: u32 = 0;
}
