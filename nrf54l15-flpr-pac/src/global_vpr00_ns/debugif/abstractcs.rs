#[doc = "Register `ABSTRACTCS` reader"]
pub type R = crate::R<AbstractcsSpec>;
#[doc = "Register `ABSTRACTCS` writer"]
pub type W = crate::W<AbstractcsSpec>;
#[doc = "Field `DATACOUNT` reader - Number of data registers that are implemented as part of the abstract command interface. Valid sizes are 1..12."]
pub type DatacountR = crate::FieldReader;
#[doc = "Command error when the abstract command fails.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmderr {
    #[doc = "0: No error."]
    NoError = 0,
    #[doc = "1: An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0"]
    Busy = 1,
    #[doc = "2: The requested command is notsupported, regardless of whether the hart is running or not."]
    NotSupported = 2,
    #[doc = "3: An exception occurred while executing the command (e.g. while executing theProgram Buffer)."]
    Exception = 3,
    #[doc = "4: The abstract command couldn't execute because the hart wasn't in the required state (running/halted). or unavailable."]
    HaltResume = 4,
    #[doc = "5: The abstract command failed due to abus error (e.g. alignment, access size, or timeout)."]
    Bus = 5,
    #[doc = "7: The command failed for another reason."]
    Other = 7,
}
impl From<Cmderr> for u8 {
    #[inline(always)]
    fn from(variant: Cmderr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmderr {
    type Ux = u8;
}
impl crate::IsEnum for Cmderr {}
#[doc = "Field `CMDERR` reader - Command error when the abstract command fails."]
pub type CmderrR = crate::FieldReader<Cmderr>;
impl CmderrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmderr> {
        match self.bits {
            0 => Some(Cmderr::NoError),
            1 => Some(Cmderr::Busy),
            2 => Some(Cmderr::NotSupported),
            3 => Some(Cmderr::Exception),
            4 => Some(Cmderr::HaltResume),
            5 => Some(Cmderr::Bus),
            7 => Some(Cmderr::Other),
            _ => None,
        }
    }
    #[doc = "No error."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Cmderr::NoError
    }
    #[doc = "An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Cmderr::Busy
    }
    #[doc = "The requested command is notsupported, regardless of whether the hart is running or not."]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == Cmderr::NotSupported
    }
    #[doc = "An exception occurred while executing the command (e.g. while executing theProgram Buffer)."]
    #[inline(always)]
    pub fn is_exception(&self) -> bool {
        *self == Cmderr::Exception
    }
    #[doc = "The abstract command couldn't execute because the hart wasn't in the required state (running/halted). or unavailable."]
    #[inline(always)]
    pub fn is_halt_resume(&self) -> bool {
        *self == Cmderr::HaltResume
    }
    #[doc = "The abstract command failed due to abus error (e.g. alignment, access size, or timeout)."]
    #[inline(always)]
    pub fn is_bus(&self) -> bool {
        *self == Cmderr::Bus
    }
    #[doc = "The command failed for another reason."]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == Cmderr::Other
    }
}
#[doc = "Field `CMDERR` writer - Command error when the abstract command fails."]
pub type CmderrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmderr>;
impl<'a, REG> CmderrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::NoError)
    }
    #[doc = "An abstract command was executing while command, abstractcs, or abstractauto was written, or when one of the data or progbuf registers was read or written. This status is only written if cmderr contains 0"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::Busy)
    }
    #[doc = "The requested command is notsupported, regardless of whether the hart is running or not."]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::NotSupported)
    }
    #[doc = "An exception occurred while executing the command (e.g. while executing theProgram Buffer)."]
    #[inline(always)]
    pub fn exception(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::Exception)
    }
    #[doc = "The abstract command couldn't execute because the hart wasn't in the required state (running/halted). or unavailable."]
    #[inline(always)]
    pub fn halt_resume(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::HaltResume)
    }
    #[doc = "The abstract command failed due to abus error (e.g. alignment, access size, or timeout)."]
    #[inline(always)]
    pub fn bus(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::Bus)
    }
    #[doc = "The command failed for another reason."]
    #[inline(always)]
    pub fn other(self) -> &'a mut crate::W<REG> {
        self.variant(Cmderr::Other)
    }
}
#[doc = "Abstract command execution status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Not busy."]
    NotBusy = 0,
    #[doc = "1: An abstract command is currently being executed. This bit is set as soon as command is written, and is not cleared until that command has completed."]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Abstract command execution status."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::NotBusy,
            true => Busy::Busy,
        }
    }
    #[doc = "Not busy."]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == Busy::NotBusy
    }
    #[doc = "An abstract command is currently being executed. This bit is set as soon as command is written, and is not cleared until that command has completed."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
#[doc = "Field `PROGBUFSIZE` reader - Size of the Program Buffer, in 32-bit words. Valid sizes are 0 - 1."]
pub type ProgbufsizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of data registers that are implemented as part of the abstract command interface. Valid sizes are 1..12."]
    #[inline(always)]
    pub fn datacount(&self) -> DatacountR {
        DatacountR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Command error when the abstract command fails."]
    #[inline(always)]
    pub fn cmderr(&self) -> CmderrR {
        CmderrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Abstract command execution status."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Size of the Program Buffer, in 32-bit words. Valid sizes are 0 - 1."]
    #[inline(always)]
    pub fn progbufsize(&self) -> ProgbufsizeR {
        ProgbufsizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Command error when the abstract command fails."]
    #[inline(always)]
    pub fn cmderr(&mut self) -> CmderrW<AbstractcsSpec> {
        CmderrW::new(self, 8)
    }
}
#[doc = "Abstract Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`abstractcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbstractcsSpec;
impl crate::RegisterSpec for AbstractcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abstractcs::R`](R) reader structure"]
impl crate::Readable for AbstractcsSpec {}
#[doc = "`write(|w| ..)` method takes [`abstractcs::W`](W) writer structure"]
impl crate::Writable for AbstractcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABSTRACTCS to value 0x0100_0002"]
impl crate::Resettable for AbstractcsSpec {
    const RESET_VALUE: u32 = 0x0100_0002;
}
