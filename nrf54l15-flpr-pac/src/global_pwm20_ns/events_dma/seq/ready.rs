#[doc = "Register `READY` reader"]
pub type R = crate::R<ReadySpec>;
#[doc = "Register `READY` writer"]
pub type W = crate::W<ReadySpec>;
#[doc = "Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::NotGenerated,
            true => Ready::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Ready::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Ready::Generated
    }
}
#[doc = "Field `READY` writer - Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, Ready>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence."]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<ReadySpec> {
        ReadyW::new(self, 0)
    }
}
#[doc = "Description cluster: Generated when EasyDMA has buffered the .PTR and .MAXCNT registers for the channel, allowing them to be written to prepare for the next sequence.\n\nYou can [`read`](crate::Reg::read) this register and get [`ready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadySpec;
impl crate::RegisterSpec for ReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ready::R`](R) reader structure"]
impl crate::Readable for ReadySpec {}
#[doc = "`write(|w| ..)` method takes [`ready::W`](W) writer structure"]
impl crate::Writable for ReadySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets READY to value 0"]
impl crate::Resettable for ReadySpec {
    const RESET_VALUE: u32 = 0;
}
