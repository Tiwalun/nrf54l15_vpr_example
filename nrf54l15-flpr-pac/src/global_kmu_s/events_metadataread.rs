#[doc = "Register `EVENTS_METADATAREAD` reader"]
pub type R = crate::R<EventsMetadatareadSpec>;
#[doc = "Register `EVENTS_METADATAREAD` writer"]
pub type W = crate::W<EventsMetadatareadSpec>;
#[doc = "Key slot metadata has been read into METADATA register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsMetadataread {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsMetadataread> for bool {
    #[inline(always)]
    fn from(variant: EventsMetadataread) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_METADATAREAD` reader - Key slot metadata has been read into METADATA register"]
pub type EventsMetadatareadR = crate::BitReader<EventsMetadataread>;
impl EventsMetadatareadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsMetadataread {
        match self.bits {
            false => EventsMetadataread::NotGenerated,
            true => EventsMetadataread::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsMetadataread::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsMetadataread::Generated
    }
}
#[doc = "Field `EVENTS_METADATAREAD` writer - Key slot metadata has been read into METADATA register"]
pub type EventsMetadatareadW<'a, REG> = crate::BitWriter<'a, REG, EventsMetadataread>;
impl<'a, REG> EventsMetadatareadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsMetadataread::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsMetadataread::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Key slot metadata has been read into METADATA register"]
    #[inline(always)]
    pub fn events_metadataread(&self) -> EventsMetadatareadR {
        EventsMetadatareadR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key slot metadata has been read into METADATA register"]
    #[inline(always)]
    pub fn events_metadataread(&mut self) -> EventsMetadatareadW<EventsMetadatareadSpec> {
        EventsMetadatareadW::new(self, 0)
    }
}
#[doc = "Key slot metadata has been read into METADATA register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_metadataread::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_metadataread::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsMetadatareadSpec;
impl crate::RegisterSpec for EventsMetadatareadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_metadataread::R`](R) reader structure"]
impl crate::Readable for EventsMetadatareadSpec {}
#[doc = "`write(|w| ..)` method takes [`events_metadataread::W`](W) writer structure"]
impl crate::Writable for EventsMetadatareadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_METADATAREAD to value 0"]
impl crate::Resettable for EventsMetadatareadSpec {
    const RESET_VALUE: u32 = 0;
}
