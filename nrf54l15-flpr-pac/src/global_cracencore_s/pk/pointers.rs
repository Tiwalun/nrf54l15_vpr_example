#[doc = "Register `POINTERS` reader"]
pub type R = crate::R<PointersSpec>;
#[doc = "Register `POINTERS` writer"]
pub type W = crate::W<PointersSpec>;
#[doc = "Field `OPPTRA` reader - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
pub type OpptraR = crate::FieldReader;
#[doc = "Field `OPPTRA` writer - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
pub type OpptraW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPPTRB` reader - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
pub type OpptrbR = crate::FieldReader;
#[doc = "Field `OPPTRB` writer - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
pub type OpptrbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPPTRC` reader - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
pub type OpptrcR = crate::FieldReader;
#[doc = "Field `OPPTRC` writer - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
pub type OpptrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPPTRN` reader - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
pub type OpptrnR = crate::FieldReader;
#[doc = "Field `OPPTRN` writer - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
pub type OpptrnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn opptra(&self) -> OpptraR {
        OpptraR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn opptrb(&self) -> OpptrbR {
        OpptrbR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
    #[inline(always)]
    pub fn opptrc(&self) -> OpptrcR {
        OpptrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    pub fn opptrn(&self) -> OpptrnR {
        OpptrnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - When executing primitive arithmetic operations, this pointer defines where operand A is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    #[must_use]
    pub fn opptra(&mut self) -> OpptraW<PointersSpec> {
        OpptraW::new(self, 0)
    }
    #[doc = "Bits 8:11 - When executing primitive arithmetic operations, this pointer defines where operand B is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    #[must_use]
    pub fn opptrb(&mut self) -> OpptrbW<PointersSpec> {
        OpptrbW::new(self, 8)
    }
    #[doc = "Bits 16:19 - When executing primitive arithmetic operations, this pointer defines the location (0x0 to 0xF) where the result will be stored in memory."]
    #[inline(always)]
    #[must_use]
    pub fn opptrc(&mut self) -> OpptrcW<PointersSpec> {
        OpptrcW::new(self, 16)
    }
    #[doc = "Bits 24:27 - When executing primitive arithmetic operations, this pointer defines the location where the modulus is located in memory (location 0x0 to 0xF)."]
    #[inline(always)]
    #[must_use]
    pub fn opptrn(&mut self) -> OpptrnW<PointersSpec> {
        OpptrnW::new(self, 24)
    }
}
#[doc = "Pointers register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pointers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pointers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PointersSpec;
impl crate::RegisterSpec for PointersSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pointers::R`](R) reader structure"]
impl crate::Readable for PointersSpec {}
#[doc = "`write(|w| ..)` method takes [`pointers::W`](W) writer structure"]
impl crate::Writable for PointersSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POINTERS to value 0"]
impl crate::Resettable for PointersSpec {
    const RESET_VALUE: u32 = 0;
}
