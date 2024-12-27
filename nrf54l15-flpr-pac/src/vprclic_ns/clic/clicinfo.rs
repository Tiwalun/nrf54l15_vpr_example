#[doc = "Register `CLICINFO` reader"]
pub type R = crate::R<ClicinfoSpec>;
#[doc = "Field `NUMINTERRUPTS` reader - Maximum number of interrupts supported."]
pub type NuminterruptsR = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - Version"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `NUMTRIGGER` reader - Number of maximum interrupt triggers supported"]
pub type NumtriggerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - Maximum number of interrupts supported."]
    #[inline(always)]
    pub fn numinterrupts(&self) -> NuminterruptsR {
        NuminterruptsR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - Version"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 25:30 - Number of maximum interrupt triggers supported"]
    #[inline(always)]
    pub fn numtrigger(&self) -> NumtriggerR {
        NumtriggerR::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
#[doc = "CLIC information.\n\nYou can [`read`](crate::Reg::read) this register and get [`clicinfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClicinfoSpec;
impl crate::RegisterSpec for ClicinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clicinfo::R`](R) reader structure"]
impl crate::Readable for ClicinfoSpec {}
#[doc = "`reset()` method sets CLICINFO to value 0x0040_1fff"]
impl crate::Resettable for ClicinfoSpec {
    const RESET_VALUE: u32 = 0x0040_1fff;
}
