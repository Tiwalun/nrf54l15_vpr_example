#[doc = "Register `TESTDATA` reader"]
pub type R = crate::R<TestdataSpec>;
#[doc = "Register `TESTDATA` writer"]
pub type W = crate::W<TestdataSpec>;
#[doc = "Field `TESTDATA` writer - Test data register."]
pub type TestdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Test data register."]
    #[inline(always)]
    #[must_use]
    pub fn testdata(&mut self) -> TestdataW<TestdataSpec> {
        TestdataW::new(self, 0)
    }
}
#[doc = "Test data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`testdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestdataSpec;
impl crate::RegisterSpec for TestdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testdata::R`](R) reader structure"]
impl crate::Readable for TestdataSpec {}
#[doc = "`write(|w| ..)` method takes [`testdata::W`](W) writer structure"]
impl crate::Writable for TestdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TESTDATA to value 0"]
impl crate::Resettable for TestdataSpec {
    const RESET_VALUE: u32 = 0;
}
