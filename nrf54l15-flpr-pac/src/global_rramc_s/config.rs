#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wen {
    #[doc = "0: Write is disabled"]
    Disabled = 0,
    #[doc = "1: Write is enabled"]
    Enabled = 1,
}
impl From<Wen> for bool {
    #[inline(always)]
    fn from(variant: Wen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WEN` reader - Write enable"]
pub type WenR = crate::BitReader<Wen>;
impl WenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wen {
        match self.bits {
            false => Wen::Disabled,
            true => Wen::Enabled,
        }
    }
    #[doc = "Write is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wen::Disabled
    }
    #[doc = "Write is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wen::Enabled
    }
}
#[doc = "Field `WEN` writer - Write enable"]
pub type WenW<'a, REG> = crate::BitWriter<'a, REG, Wen>;
impl<'a, REG> WenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Disabled)
    }
    #[doc = "Write is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wen::Enabled)
    }
}
#[doc = "write-buffer size in number of 128-bit words\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Writebufsize {
    #[doc = "0: Disable buffering"]
    Unbuffered = 0,
}
impl From<Writebufsize> for u8 {
    #[inline(always)]
    fn from(variant: Writebufsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Writebufsize {
    type Ux = u8;
}
impl crate::IsEnum for Writebufsize {}
#[doc = "Field `WRITEBUFSIZE` reader - write-buffer size in number of 128-bit words"]
pub type WritebufsizeR = crate::FieldReader<Writebufsize>;
impl WritebufsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Writebufsize> {
        match self.bits {
            0 => Some(Writebufsize::Unbuffered),
            _ => None,
        }
    }
    #[doc = "Disable buffering"]
    #[inline(always)]
    pub fn is_unbuffered(&self) -> bool {
        *self == Writebufsize::Unbuffered
    }
}
#[doc = "Field `WRITEBUFSIZE` writer - write-buffer size in number of 128-bit words"]
pub type WritebufsizeW<'a, REG> = crate::FieldWriter<'a, REG, 6, Writebufsize>;
impl<'a, REG> WritebufsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable buffering"]
    #[inline(always)]
    pub fn unbuffered(self) -> &'a mut crate::W<REG> {
        self.variant(Writebufsize::Unbuffered)
    }
}
impl R {
    #[doc = "Bit 0 - Write enable"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - write-buffer size in number of 128-bit words"]
    #[inline(always)]
    pub fn writebufsize(&self) -> WritebufsizeR {
        WritebufsizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable"]
    #[inline(always)]
    pub fn wen(&mut self) -> WenW<ConfigSpec> {
        WenW::new(self, 0)
    }
    #[doc = "Bits 8:13 - write-buffer size in number of 128-bit words"]
    #[inline(always)]
    pub fn writebufsize(&mut self) -> WritebufsizeW<ConfigSpec> {
        WritebufsizeW::new(self, 8)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
