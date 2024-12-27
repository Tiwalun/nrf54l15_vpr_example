#[doc = "Register `DATAWHITE` reader"]
pub type R = crate::R<DatawhiteSpec>;
#[doc = "Register `DATAWHITE` writer"]
pub type W = crate::W<DatawhiteSpec>;
#[doc = "Field `IV` reader - Whitening initial value"]
pub type IvR = crate::FieldReader<u16>;
#[doc = "Field `IV` writer - Whitening initial value"]
pub type IvW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `POLY` reader - Whitening polynomial"]
pub type PolyR = crate::FieldReader<u16>;
#[doc = "Field `POLY` writer - Whitening polynomial"]
pub type PolyW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:8 - Whitening initial value"]
    #[inline(always)]
    pub fn iv(&self) -> IvR {
        IvR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:25 - Whitening polynomial"]
    #[inline(always)]
    pub fn poly(&self) -> PolyR {
        PolyR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Whitening initial value"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IvW<DatawhiteSpec> {
        IvW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Whitening polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> PolyW<DatawhiteSpec> {
        PolyW::new(self, 16)
    }
}
#[doc = "Data whitening configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`datawhite::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datawhite::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatawhiteSpec;
impl crate::RegisterSpec for DatawhiteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datawhite::R`](R) reader structure"]
impl crate::Readable for DatawhiteSpec {}
#[doc = "`write(|w| ..)` method takes [`datawhite::W`](W) writer structure"]
impl crate::Writable for DatawhiteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAWHITE to value 0x0089_0040"]
impl crate::Resettable for DatawhiteSpec {
    const RESET_VALUE: u32 = 0x0089_0040;
}
