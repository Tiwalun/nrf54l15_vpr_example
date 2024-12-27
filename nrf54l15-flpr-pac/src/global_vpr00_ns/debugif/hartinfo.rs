#[doc = "Register `HARTINFO` reader"]
pub type R = crate::R<HartinfoSpec>;
#[doc = "Register `HARTINFO` writer"]
pub type W = crate::W<HartinfoSpec>;
#[doc = "Field `DATAADDR` reader - Data Address"]
pub type DataaddrR = crate::FieldReader<u16>;
#[doc = "Field `DATASIZE` reader - Data Size"]
pub type DatasizeR = crate::FieldReader;
#[doc = "Data Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataaccess {
    #[doc = "0: The data registers are shadowed in the hart by CSRs. Each CSR is DXLEN bits in size, and corresponds to a single argument."]
    No = 0,
    #[doc = "1: The data registers are shadowed in the hart's memory map. Each register takes up 4 bytes in the memory map."]
    Yes = 1,
}
impl From<Dataaccess> for bool {
    #[inline(always)]
    fn from(variant: Dataaccess) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAACCESS` reader - Data Access"]
pub type DataaccessR = crate::BitReader<Dataaccess>;
impl DataaccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataaccess {
        match self.bits {
            false => Dataaccess::No,
            true => Dataaccess::Yes,
        }
    }
    #[doc = "The data registers are shadowed in the hart by CSRs. Each CSR is DXLEN bits in size, and corresponds to a single argument."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Dataaccess::No
    }
    #[doc = "The data registers are shadowed in the hart's memory map. Each register takes up 4 bytes in the memory map."]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Dataaccess::Yes
    }
}
#[doc = "Field `NSCRATCH` reader - Number of dscratch registers"]
pub type NscratchR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Data Address"]
    #[inline(always)]
    pub fn dataaddr(&self) -> DataaddrR {
        DataaddrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Data Size"]
    #[inline(always)]
    pub fn datasize(&self) -> DatasizeR {
        DatasizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Data Access"]
    #[inline(always)]
    pub fn dataaccess(&self) -> DataaccessR {
        DataaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of dscratch registers"]
    #[inline(always)]
    pub fn nscratch(&self) -> NscratchR {
        NscratchR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Hart Information\n\nYou can [`read`](crate::Reg::read) this register and get [`hartinfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hartinfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HartinfoSpec;
impl crate::RegisterSpec for HartinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hartinfo::R`](R) reader structure"]
impl crate::Readable for HartinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`hartinfo::W`](W) writer structure"]
impl crate::Writable for HartinfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HARTINFO to value 0"]
impl crate::Resettable for HartinfoSpec {
    const RESET_VALUE: u32 = 0;
}
