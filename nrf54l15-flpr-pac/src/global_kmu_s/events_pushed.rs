#[doc = "Register `EVENTS_PUSHED` reader"]
pub type R = crate::R<EventsPushedSpec>;
#[doc = "Register `EVENTS_PUSHED` writer"]
pub type W = crate::W<EventsPushedSpec>;
#[doc = "Key slot successfully pushed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPushed {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPushed> for bool {
    #[inline(always)]
    fn from(variant: EventsPushed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PUSHED` reader - Key slot successfully pushed"]
pub type EventsPushedR = crate::BitReader<EventsPushed>;
impl EventsPushedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPushed {
        match self.bits {
            false => EventsPushed::NotGenerated,
            true => EventsPushed::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPushed::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPushed::Generated
    }
}
#[doc = "Field `EVENTS_PUSHED` writer - Key slot successfully pushed"]
pub type EventsPushedW<'a, REG> = crate::BitWriter<'a, REG, EventsPushed>;
impl<'a, REG> EventsPushedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPushed::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPushed::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Key slot successfully pushed"]
    #[inline(always)]
    pub fn events_pushed(&self) -> EventsPushedR {
        EventsPushedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key slot successfully pushed"]
    #[inline(always)]
    pub fn events_pushed(&mut self) -> EventsPushedW<EventsPushedSpec> {
        EventsPushedW::new(self, 0)
    }
}
#[doc = "Key slot successfully pushed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pushed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pushed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPushedSpec;
impl crate::RegisterSpec for EventsPushedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pushed::R`](R) reader structure"]
impl crate::Readable for EventsPushedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_pushed::W`](W) writer structure"]
impl crate::Writable for EventsPushedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_PUSHED to value 0"]
impl crate::Resettable for EventsPushedSpec {
    const RESET_VALUE: u32 = 0;
}
