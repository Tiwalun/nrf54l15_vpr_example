#[doc = "Register `EVENTS_PKEIKG` reader"]
pub type R = crate::R<EventsPkeikgSpec>;
#[doc = "Register `EVENTS_PKEIKG` writer"]
pub type W = crate::W<EventsPkeikgSpec>;
#[doc = "Event indicating that interrupt triggered at PKE or IKG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPkeikg {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPkeikg> for bool {
    #[inline(always)]
    fn from(variant: EventsPkeikg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PKEIKG` reader - Event indicating that interrupt triggered at PKE or IKG"]
pub type EventsPkeikgR = crate::BitReader<EventsPkeikg>;
impl EventsPkeikgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPkeikg {
        match self.bits {
            false => EventsPkeikg::NotGenerated,
            true => EventsPkeikg::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPkeikg::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPkeikg::Generated
    }
}
#[doc = "Field `EVENTS_PKEIKG` writer - Event indicating that interrupt triggered at PKE or IKG"]
pub type EventsPkeikgW<'a, REG> = crate::BitWriter<'a, REG, EventsPkeikg>;
impl<'a, REG> EventsPkeikgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPkeikg::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPkeikg::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event indicating that interrupt triggered at PKE or IKG"]
    #[inline(always)]
    pub fn events_pkeikg(&self) -> EventsPkeikgR {
        EventsPkeikgR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event indicating that interrupt triggered at PKE or IKG"]
    #[inline(always)]
    #[must_use]
    pub fn events_pkeikg(&mut self) -> EventsPkeikgW<EventsPkeikgSpec> {
        EventsPkeikgW::new(self, 0)
    }
}
#[doc = "Event indicating that interrupt triggered at PKE or IKG\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pkeikg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pkeikg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPkeikgSpec;
impl crate::RegisterSpec for EventsPkeikgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pkeikg::R`](R) reader structure"]
impl crate::Readable for EventsPkeikgSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pkeikg::W`](W) writer structure"]
impl crate::Writable for EventsPkeikgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_PKEIKG to value 0"]
impl crate::Resettable for EventsPkeikgSpec {
    const RESET_VALUE: u32 = 0;
}
