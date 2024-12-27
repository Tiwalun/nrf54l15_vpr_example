#[doc = "Register `MATCH[%s]` reader"]
pub type R = crate::R<MatchSpec>;
#[doc = "Register `MATCH[%s]` writer"]
pub type W = crate::W<MatchSpec>;
#[doc = "Pattern match is detected on the DMA data bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Match {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<Match> for bool {
    #[inline(always)]
    fn from(variant: Match) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MATCH` reader - Pattern match is detected on the DMA data bus."]
pub type MatchR = crate::BitReader<Match>;
impl MatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Match {
        match self.bits {
            false => Match::NotGenerated,
            true => Match::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == Match::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == Match::Generated
    }
}
#[doc = "Field `MATCH` writer - Pattern match is detected on the DMA data bus."]
pub type MatchW<'a, REG> = crate::BitWriter<'a, REG, Match>;
impl<'a, REG> MatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Match::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(Match::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Pattern match is detected on the DMA data bus."]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pattern match is detected on the DMA data bus."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MatchW<MatchSpec> {
        MatchW::new(self, 0)
    }
}
#[doc = "Description collection: Pattern match is detected on the DMA data bus.\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatchSpec;
impl crate::RegisterSpec for MatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`match_::R`](R) reader structure"]
impl crate::Readable for MatchSpec {}
#[doc = "`write(|w| ..)` method takes [`match_::W`](W) writer structure"]
impl crate::Writable for MatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MATCH[%s]
to value 0"]
impl crate::Resettable for MatchSpec {
    const RESET_VALUE: u32 = 0;
}
