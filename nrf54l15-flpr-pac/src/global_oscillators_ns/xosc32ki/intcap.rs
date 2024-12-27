#[doc = "Register `INTCAP` reader"]
pub type R = crate::R<IntcapSpec>;
#[doc = "Register `INTCAP` writer"]
pub type W = crate::W<IntcapSpec>;
#[doc = "Field `VAL` reader - Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Crystal load capacitor as seen by the crystal across its terminals, including pin capacitance but excluding PCB stray capacitance."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<IntcapSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Programmable capacitance of XL1 and XL2\n\nYou can [`read`](crate::Reg::read) this register and get [`intcap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intcap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcapSpec;
impl crate::RegisterSpec for IntcapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intcap::R`](R) reader structure"]
impl crate::Readable for IntcapSpec {}
#[doc = "`write(|w| ..)` method takes [`intcap::W`](W) writer structure"]
impl crate::Writable for IntcapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCAP to value 0x17"]
impl crate::Resettable for IntcapSpec {
    const RESET_VALUE: u32 = 0x17;
}
