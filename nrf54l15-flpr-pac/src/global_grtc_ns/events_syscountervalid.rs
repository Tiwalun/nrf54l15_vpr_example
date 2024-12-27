#[doc = "Register `EVENTS_SYSCOUNTERVALID` reader"]
pub type R = crate::R<EventsSyscountervalidSpec>;
#[doc = "Register `EVENTS_SYSCOUNTERVALID` writer"]
pub type W = crate::W<EventsSyscountervalidSpec>;
#[doc = "The SYSCOUNTER is in active state and value is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsSyscountervalid {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsSyscountervalid> for bool {
    #[inline(always)]
    fn from(variant: EventsSyscountervalid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_SYSCOUNTERVALID` reader - The SYSCOUNTER is in active state and value is valid"]
pub type EventsSyscountervalidR = crate::BitReader<EventsSyscountervalid>;
impl EventsSyscountervalidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsSyscountervalid {
        match self.bits {
            false => EventsSyscountervalid::NotGenerated,
            true => EventsSyscountervalid::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsSyscountervalid::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsSyscountervalid::Generated
    }
}
#[doc = "Field `EVENTS_SYSCOUNTERVALID` writer - The SYSCOUNTER is in active state and value is valid"]
pub type EventsSyscountervalidW<'a, REG> = crate::BitWriter<'a, REG, EventsSyscountervalid>;
impl<'a, REG> EventsSyscountervalidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSyscountervalid::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsSyscountervalid::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - The SYSCOUNTER is in active state and value is valid"]
    #[inline(always)]
    pub fn events_syscountervalid(&self) -> EventsSyscountervalidR {
        EventsSyscountervalidR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The SYSCOUNTER is in active state and value is valid"]
    #[inline(always)]
    #[must_use]
    pub fn events_syscountervalid(&mut self) -> EventsSyscountervalidW<EventsSyscountervalidSpec> {
        EventsSyscountervalidW::new(self, 0)
    }
}
#[doc = "The SYSCOUNTER is in active state and value is valid\n\nYou can [`read`](crate::Reg::read) this register and get [`events_syscountervalid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_syscountervalid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsSyscountervalidSpec;
impl crate::RegisterSpec for EventsSyscountervalidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_syscountervalid::R`](R) reader structure"]
impl crate::Readable for EventsSyscountervalidSpec {}
#[doc = "`write(|w| ..)` method takes [`events_syscountervalid::W`](W) writer structure"]
impl crate::Writable for EventsSyscountervalidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_SYSCOUNTERVALID to value 0"]
impl crate::Resettable for EventsSyscountervalidSpec {
    const RESET_VALUE: u32 = 0;
}
