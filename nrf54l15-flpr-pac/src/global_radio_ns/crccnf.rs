#[doc = "Register `CRCCNF` reader"]
pub type R = crate::R<CrccnfSpec>;
#[doc = "Register `CRCCNF` writer"]
pub type W = crate::W<CrccnfSpec>;
#[doc = "CRC length in number of bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Len {
    #[doc = "0: CRC length is zero and CRC calculation is disabled"]
    Disabled = 0,
    #[doc = "1: CRC length is one byte and CRC calculation is enabled"]
    One = 1,
    #[doc = "2: CRC length is two bytes and CRC calculation is enabled"]
    Two = 2,
    #[doc = "3: CRC length is three bytes and CRC calculation is enabled"]
    Three = 3,
}
impl From<Len> for u8 {
    #[inline(always)]
    fn from(variant: Len) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Len {
    type Ux = u8;
}
impl crate::IsEnum for Len {}
#[doc = "Field `LEN` reader - CRC length in number of bytes."]
pub type LenR = crate::FieldReader<Len>;
impl LenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Len {
        match self.bits {
            0 => Len::Disabled,
            1 => Len::One,
            2 => Len::Two,
            3 => Len::Three,
            _ => unreachable!(),
        }
    }
    #[doc = "CRC length is zero and CRC calculation is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Len::Disabled
    }
    #[doc = "CRC length is one byte and CRC calculation is enabled"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Len::One
    }
    #[doc = "CRC length is two bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Len::Two
    }
    #[doc = "CRC length is three bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Len::Three
    }
}
#[doc = "Field `LEN` writer - CRC length in number of bytes."]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Len, crate::Safe>;
impl<'a, REG> LenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC length is zero and CRC calculation is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Disabled)
    }
    #[doc = "CRC length is one byte and CRC calculation is enabled"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Len::One)
    }
    #[doc = "CRC length is two bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Two)
    }
    #[doc = "CRC length is three bytes and CRC calculation is enabled"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Len::Three)
    }
}
#[doc = "Control whether CRC calculation skips the address field. Other fields can also be skipped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Skipaddr {
    #[doc = "0: CRC calculation includes address field"]
    Include = 0,
    #[doc = "1: CRC calculation starting at first byte after address field."]
    Skip = 1,
    #[doc = "2: CRC calculation starting at first byte after length field (as per 802.15.4 standard)."]
    Ieee802154 = 2,
    #[doc = "3: CRC calculation starting at first byte after S0 field."]
    SkipS0 = 3,
    #[doc = "4: CRC calculation starting at first byte after S1 field."]
    SkipS1 = 4,
}
impl From<Skipaddr> for u8 {
    #[inline(always)]
    fn from(variant: Skipaddr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Skipaddr {
    type Ux = u8;
}
impl crate::IsEnum for Skipaddr {}
#[doc = "Field `SKIPADDR` reader - Control whether CRC calculation skips the address field. Other fields can also be skipped."]
pub type SkipaddrR = crate::FieldReader<Skipaddr>;
impl SkipaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Skipaddr> {
        match self.bits {
            0 => Some(Skipaddr::Include),
            1 => Some(Skipaddr::Skip),
            2 => Some(Skipaddr::Ieee802154),
            3 => Some(Skipaddr::SkipS0),
            4 => Some(Skipaddr::SkipS1),
            _ => None,
        }
    }
    #[doc = "CRC calculation includes address field"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == Skipaddr::Include
    }
    #[doc = "CRC calculation starting at first byte after address field."]
    #[inline(always)]
    pub fn is_skip(&self) -> bool {
        *self == Skipaddr::Skip
    }
    #[doc = "CRC calculation starting at first byte after length field (as per 802.15.4 standard)."]
    #[inline(always)]
    pub fn is_ieee802154(&self) -> bool {
        *self == Skipaddr::Ieee802154
    }
    #[doc = "CRC calculation starting at first byte after S0 field."]
    #[inline(always)]
    pub fn is_skip_s0(&self) -> bool {
        *self == Skipaddr::SkipS0
    }
    #[doc = "CRC calculation starting at first byte after S1 field."]
    #[inline(always)]
    pub fn is_skip_s1(&self) -> bool {
        *self == Skipaddr::SkipS1
    }
}
#[doc = "Field `SKIPADDR` writer - Control whether CRC calculation skips the address field. Other fields can also be skipped."]
pub type SkipaddrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Skipaddr>;
impl<'a, REG> SkipaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC calculation includes address field"]
    #[inline(always)]
    pub fn include(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::Include)
    }
    #[doc = "CRC calculation starting at first byte after address field."]
    #[inline(always)]
    pub fn skip(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::Skip)
    }
    #[doc = "CRC calculation starting at first byte after length field (as per 802.15.4 standard)."]
    #[inline(always)]
    pub fn ieee802154(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::Ieee802154)
    }
    #[doc = "CRC calculation starting at first byte after S0 field."]
    #[inline(always)]
    pub fn skip_s0(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::SkipS0)
    }
    #[doc = "CRC calculation starting at first byte after S1 field."]
    #[inline(always)]
    pub fn skip_s1(self) -> &'a mut crate::W<REG> {
        self.variant(Skipaddr::SkipS1)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC length in number of bytes."]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Control whether CRC calculation skips the address field. Other fields can also be skipped."]
    #[inline(always)]
    pub fn skipaddr(&self) -> SkipaddrR {
        SkipaddrR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC length in number of bytes."]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<CrccnfSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Control whether CRC calculation skips the address field. Other fields can also be skipped."]
    #[inline(always)]
    pub fn skipaddr(&mut self) -> SkipaddrW<CrccnfSpec> {
        SkipaddrW::new(self, 8)
    }
}
#[doc = "CRC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`crccnf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccnf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrccnfSpec;
impl crate::RegisterSpec for CrccnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crccnf::R`](R) reader structure"]
impl crate::Readable for CrccnfSpec {}
#[doc = "`write(|w| ..)` method takes [`crccnf::W`](W) writer structure"]
impl crate::Writable for CrccnfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCCNF to value 0"]
impl crate::Resettable for CrccnfSpec {
    const RESET_VALUE: u32 = 0;
}
