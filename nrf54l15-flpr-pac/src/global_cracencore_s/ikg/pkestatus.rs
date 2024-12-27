#[doc = "Register `PKESTATUS` reader"]
pub type R = crate::R<PkestatusSpec>;
#[doc = "Register `PKESTATUS` writer"]
pub type W = crate::W<PkestatusSpec>;
#[doc = "Field `ERROR` reader - Error because either Private Keys are not stored or the operation is not defined."]
pub type ErrorR = crate::BitReader;
#[doc = "Field `STARTERROR` reader - Error because a new operation is started while the previous one is still busy."]
pub type StarterrorR = crate::BitReader;
#[doc = "Field `IKGPKBUSY` reader - Busy, set when the operation starts and cleared when the operation is finished."]
pub type IkgpkbusyR = crate::BitReader;
#[doc = "Field `IRQSTATUS` reader - IRQ, set when the operation is finished and cleared when the CPU writes the bit 1 of PKE_Control Register or a new operation is started."]
pub type IrqstatusR = crate::BitReader;
#[doc = "Field `ERASEBUSY` reader - The PKE Data RAM is being erased."]
pub type ErasebusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Error because either Private Keys are not stored or the operation is not defined."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error because a new operation is started while the previous one is still busy."]
    #[inline(always)]
    pub fn starterror(&self) -> StarterrorR {
        StarterrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy, set when the operation starts and cleared when the operation is finished."]
    #[inline(always)]
    pub fn ikgpkbusy(&self) -> IkgpkbusyR {
        IkgpkbusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IRQ, set when the operation is finished and cleared when the CPU writes the bit 1 of PKE_Control Register or a new operation is started."]
    #[inline(always)]
    pub fn irqstatus(&self) -> IrqstatusR {
        IrqstatusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The PKE Data RAM is being erased."]
    #[inline(always)]
    pub fn erasebusy(&self) -> ErasebusyR {
        ErasebusyR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {}
#[doc = "PKE Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pkestatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkestatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkestatusSpec;
impl crate::RegisterSpec for PkestatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkestatus::R`](R) reader structure"]
impl crate::Readable for PkestatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pkestatus::W`](W) writer structure"]
impl crate::Writable for PkestatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKESTATUS to value 0"]
impl crate::Resettable for PkestatusSpec {
    const RESET_VALUE: u32 = 0;
}
