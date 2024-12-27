#[doc = "Register `END` reader"]
pub type R = crate::R<EndSpec>;
#[doc = "Register `END` writer"]
pub type W = crate::W<EndSpec>;
#[doc = "Generated after all MAXCNT bytes have been transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Generated after all MAXCNT bytes have been transferred"]
pub type EndR = crate::BitReader<End>;
impl EndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> End {
        match self.bits {
            false => End::NotGenerated,
            true => End::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == End::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == End::Generated
    }
}
#[doc = "Field `END` writer - Generated after all MAXCNT bytes have been transferred"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, End>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(End::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(End::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Generated after all MAXCNT bytes have been transferred"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generated after all MAXCNT bytes have been transferred"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<EndSpec> {
        EndW::new(self, 0)
    }
}
#[doc = "Generated after all MAXCNT bytes have been transferred\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndSpec;
impl crate::RegisterSpec for EndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`end::R`](R) reader structure"]
impl crate::Readable for EndSpec {}
#[doc = "`write(|w| ..)` method takes [`end::W`](W) writer structure"]
impl crate::Writable for EndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets END to value 0"]
impl crate::Resettable for EndSpec {
    const RESET_VALUE: u32 = 0;
}
