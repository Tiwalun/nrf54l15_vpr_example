#[doc = "Register `EVENTS_CRYPTOMASTER` reader"]
pub type R = crate::R<EventsCryptomasterSpec>;
#[doc = "Register `EVENTS_CRYPTOMASTER` writer"]
pub type W = crate::W<EventsCryptomasterSpec>;
#[doc = "Event indicating that interrupt triggered at Cryptomaster\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsCryptomaster {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsCryptomaster> for bool {
    #[inline(always)]
    fn from(variant: EventsCryptomaster) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CRYPTOMASTER` reader - Event indicating that interrupt triggered at Cryptomaster"]
pub type EventsCryptomasterR = crate::BitReader<EventsCryptomaster>;
impl EventsCryptomasterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsCryptomaster {
        match self.bits {
            false => EventsCryptomaster::NotGenerated,
            true => EventsCryptomaster::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsCryptomaster::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsCryptomaster::Generated
    }
}
#[doc = "Field `EVENTS_CRYPTOMASTER` writer - Event indicating that interrupt triggered at Cryptomaster"]
pub type EventsCryptomasterW<'a, REG> = crate::BitWriter<'a, REG, EventsCryptomaster>;
impl<'a, REG> EventsCryptomasterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCryptomaster::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsCryptomaster::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event indicating that interrupt triggered at Cryptomaster"]
    #[inline(always)]
    pub fn events_cryptomaster(&self) -> EventsCryptomasterR {
        EventsCryptomasterR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event indicating that interrupt triggered at Cryptomaster"]
    #[inline(always)]
    #[must_use]
    pub fn events_cryptomaster(&mut self) -> EventsCryptomasterW<EventsCryptomasterSpec> {
        EventsCryptomasterW::new(self, 0)
    }
}
#[doc = "Event indicating that interrupt triggered at Cryptomaster\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cryptomaster::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cryptomaster::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsCryptomasterSpec;
impl crate::RegisterSpec for EventsCryptomasterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_cryptomaster::R`](R) reader structure"]
impl crate::Readable for EventsCryptomasterSpec {}
#[doc = "`write(|w| ..)` method takes [`events_cryptomaster::W`](W) writer structure"]
impl crate::Writable for EventsCryptomasterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_CRYPTOMASTER to value 0"]
impl crate::Resettable for EventsCryptomasterSpec {
    const RESET_VALUE: u32 = 0;
}
