#[doc = "Register `POFSTAT` reader"]
pub type R = crate::R<PofstatSpec>;
#[doc = "Power-fail comparator status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparator {
    #[doc = "0: Voltage detected above VPOF threshold"]
    Above = 0,
    #[doc = "1: Voltage detected below VPOF threshold"]
    Below = 1,
}
impl From<Comparator> for bool {
    #[inline(always)]
    fn from(variant: Comparator) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARATOR` reader - Power-fail comparator status"]
pub type ComparatorR = crate::BitReader<Comparator>;
impl ComparatorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Comparator {
        match self.bits {
            false => Comparator::Above,
            true => Comparator::Below,
        }
    }
    #[doc = "Voltage detected above VPOF threshold"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Comparator::Above
    }
    #[doc = "Voltage detected below VPOF threshold"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Comparator::Below
    }
}
impl R {
    #[doc = "Bit 0 - Power-fail comparator status"]
    #[inline(always)]
    pub fn comparator(&self) -> ComparatorR {
        ComparatorR::new((self.bits & 1) != 0)
    }
}
#[doc = "Power-fail comparator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pofstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PofstatSpec;
impl crate::RegisterSpec for PofstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pofstat::R`](R) reader structure"]
impl crate::Readable for PofstatSpec {}
#[doc = "`reset()` method sets POFSTAT to value 0"]
impl crate::Resettable for PofstatSpec {
    const RESET_VALUE: u32 = 0;
}
