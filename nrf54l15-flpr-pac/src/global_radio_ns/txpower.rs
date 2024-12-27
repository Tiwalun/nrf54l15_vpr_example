#[doc = "Register `TXPOWER` reader"]
pub type R = crate::R<TxpowerSpec>;
#[doc = "Register `TXPOWER` writer"]
pub type W = crate::W<TxpowerSpec>;
#[doc = "RADIO output power\n\nValue on reset: 19"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Txpower {
    #[doc = "63: +8 dBm"]
    Pos8dBm = 63,
    #[doc = "57: +7 dBm"]
    Pos7dBm = 57,
    #[doc = "51: +6 dBm"]
    Pos6dBm = 51,
    #[doc = "45: +5 dBm"]
    Pos5dBm = 45,
    #[doc = "40: +4 dBm"]
    Pos4dBm = 40,
    #[doc = "35: +3 dBm"]
    Pos3dBm = 35,
    #[doc = "31: +2 dBm"]
    Pos2dBm = 31,
    #[doc = "27: +1 dBm"]
    Pos1dBm = 27,
    #[doc = "24: 0 dBm"]
    _0dBm = 24,
    #[doc = "21: -1 dBm"]
    Neg1dBm = 21,
    #[doc = "19: -2 dBm"]
    Neg2dBm = 19,
    #[doc = "17: -3 dBm"]
    Neg3dBm = 17,
    #[doc = "15: -4 dBm"]
    Neg4dBm = 15,
    #[doc = "13: -5 dBm"]
    Neg5dBm = 13,
    #[doc = "11: -6 dBm"]
    Neg6dBm = 11,
    #[doc = "10: -7 dBm"]
    Neg7dBm = 10,
    #[doc = "9: -8 dBm"]
    Neg8dBm = 9,
    #[doc = "8: -9 dBm"]
    Neg9dBm = 8,
    #[doc = "7: -10 dBm"]
    Neg10dBm = 7,
    #[doc = "6: -12 dBm"]
    Neg12dBm = 6,
    #[doc = "5: -14 dBm"]
    Neg14dBm = 5,
    #[doc = "4: -16 dBm"]
    Neg16dBm = 4,
    #[doc = "3: -18 dBm"]
    Neg18dBm = 3,
    #[doc = "2: -20 dBm"]
    Neg20dBm = 2,
    #[doc = "1: -28 dBm"]
    Neg28dBm = 1,
    #[doc = "304: -40 dBm"]
    Neg40dBm = 304,
    #[doc = "272: -46 dBm"]
    Neg46dBm = 272,
}
impl From<Txpower> for u16 {
    #[inline(always)]
    fn from(variant: Txpower) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txpower {
    type Ux = u16;
}
impl crate::IsEnum for Txpower {}
#[doc = "Field `TXPOWER` reader - RADIO output power"]
pub type TxpowerR = crate::FieldReader<Txpower>;
impl TxpowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txpower> {
        match self.bits {
            63 => Some(Txpower::Pos8dBm),
            57 => Some(Txpower::Pos7dBm),
            51 => Some(Txpower::Pos6dBm),
            45 => Some(Txpower::Pos5dBm),
            40 => Some(Txpower::Pos4dBm),
            35 => Some(Txpower::Pos3dBm),
            31 => Some(Txpower::Pos2dBm),
            27 => Some(Txpower::Pos1dBm),
            24 => Some(Txpower::_0dBm),
            21 => Some(Txpower::Neg1dBm),
            19 => Some(Txpower::Neg2dBm),
            17 => Some(Txpower::Neg3dBm),
            15 => Some(Txpower::Neg4dBm),
            13 => Some(Txpower::Neg5dBm),
            11 => Some(Txpower::Neg6dBm),
            10 => Some(Txpower::Neg7dBm),
            9 => Some(Txpower::Neg8dBm),
            8 => Some(Txpower::Neg9dBm),
            7 => Some(Txpower::Neg10dBm),
            6 => Some(Txpower::Neg12dBm),
            5 => Some(Txpower::Neg14dBm),
            4 => Some(Txpower::Neg16dBm),
            3 => Some(Txpower::Neg18dBm),
            2 => Some(Txpower::Neg20dBm),
            1 => Some(Txpower::Neg28dBm),
            304 => Some(Txpower::Neg40dBm),
            272 => Some(Txpower::Neg46dBm),
            _ => None,
        }
    }
    #[doc = "+8 dBm"]
    #[inline(always)]
    pub fn is_pos8d_bm(&self) -> bool {
        *self == Txpower::Pos8dBm
    }
    #[doc = "+7 dBm"]
    #[inline(always)]
    pub fn is_pos7d_bm(&self) -> bool {
        *self == Txpower::Pos7dBm
    }
    #[doc = "+6 dBm"]
    #[inline(always)]
    pub fn is_pos6d_bm(&self) -> bool {
        *self == Txpower::Pos6dBm
    }
    #[doc = "+5 dBm"]
    #[inline(always)]
    pub fn is_pos5d_bm(&self) -> bool {
        *self == Txpower::Pos5dBm
    }
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn is_pos4d_bm(&self) -> bool {
        *self == Txpower::Pos4dBm
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn is_pos3d_bm(&self) -> bool {
        *self == Txpower::Pos3dBm
    }
    #[doc = "+2 dBm"]
    #[inline(always)]
    pub fn is_pos2d_bm(&self) -> bool {
        *self == Txpower::Pos2dBm
    }
    #[doc = "+1 dBm"]
    #[inline(always)]
    pub fn is_pos1d_bm(&self) -> bool {
        *self == Txpower::Pos1dBm
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == Txpower::_0dBm
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn is_neg1d_bm(&self) -> bool {
        *self == Txpower::Neg1dBm
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn is_neg2d_bm(&self) -> bool {
        *self == Txpower::Neg2dBm
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn is_neg3d_bm(&self) -> bool {
        *self == Txpower::Neg3dBm
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == Txpower::Neg4dBm
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn is_neg5d_bm(&self) -> bool {
        *self == Txpower::Neg5dBm
    }
    #[doc = "-6 dBm"]
    #[inline(always)]
    pub fn is_neg6d_bm(&self) -> bool {
        *self == Txpower::Neg6dBm
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn is_neg7d_bm(&self) -> bool {
        *self == Txpower::Neg7dBm
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == Txpower::Neg8dBm
    }
    #[doc = "-9 dBm"]
    #[inline(always)]
    pub fn is_neg9d_bm(&self) -> bool {
        *self == Txpower::Neg9dBm
    }
    #[doc = "-10 dBm"]
    #[inline(always)]
    pub fn is_neg10d_bm(&self) -> bool {
        *self == Txpower::Neg10dBm
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == Txpower::Neg12dBm
    }
    #[doc = "-14 dBm"]
    #[inline(always)]
    pub fn is_neg14d_bm(&self) -> bool {
        *self == Txpower::Neg14dBm
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == Txpower::Neg16dBm
    }
    #[doc = "-18 dBm"]
    #[inline(always)]
    pub fn is_neg18d_bm(&self) -> bool {
        *self == Txpower::Neg18dBm
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == Txpower::Neg20dBm
    }
    #[doc = "-28 dBm"]
    #[inline(always)]
    pub fn is_neg28d_bm(&self) -> bool {
        *self == Txpower::Neg28dBm
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == Txpower::Neg40dBm
    }
    #[doc = "-46 dBm"]
    #[inline(always)]
    pub fn is_neg46d_bm(&self) -> bool {
        *self == Txpower::Neg46dBm
    }
}
#[doc = "Field `TXPOWER` writer - RADIO output power"]
pub type TxpowerW<'a, REG> = crate::FieldWriter<'a, REG, 11, Txpower>;
impl<'a, REG> TxpowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "+8 dBm"]
    #[inline(always)]
    pub fn pos8d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos8dBm)
    }
    #[doc = "+7 dBm"]
    #[inline(always)]
    pub fn pos7d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos7dBm)
    }
    #[doc = "+6 dBm"]
    #[inline(always)]
    pub fn pos6d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos6dBm)
    }
    #[doc = "+5 dBm"]
    #[inline(always)]
    pub fn pos5d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos5dBm)
    }
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn pos4d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos4dBm)
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn pos3d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos3dBm)
    }
    #[doc = "+2 dBm"]
    #[inline(always)]
    pub fn pos2d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos2dBm)
    }
    #[doc = "+1 dBm"]
    #[inline(always)]
    pub fn pos1d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Pos1dBm)
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::_0dBm)
    }
    #[doc = "-1 dBm"]
    #[inline(always)]
    pub fn neg1d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg1dBm)
    }
    #[doc = "-2 dBm"]
    #[inline(always)]
    pub fn neg2d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg2dBm)
    }
    #[doc = "-3 dBm"]
    #[inline(always)]
    pub fn neg3d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg3dBm)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg4dBm)
    }
    #[doc = "-5 dBm"]
    #[inline(always)]
    pub fn neg5d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg5dBm)
    }
    #[doc = "-6 dBm"]
    #[inline(always)]
    pub fn neg6d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg6dBm)
    }
    #[doc = "-7 dBm"]
    #[inline(always)]
    pub fn neg7d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg7dBm)
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg8dBm)
    }
    #[doc = "-9 dBm"]
    #[inline(always)]
    pub fn neg9d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg9dBm)
    }
    #[doc = "-10 dBm"]
    #[inline(always)]
    pub fn neg10d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg10dBm)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg12dBm)
    }
    #[doc = "-14 dBm"]
    #[inline(always)]
    pub fn neg14d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg14dBm)
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg16dBm)
    }
    #[doc = "-18 dBm"]
    #[inline(always)]
    pub fn neg18d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg18dBm)
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg20dBm)
    }
    #[doc = "-28 dBm"]
    #[inline(always)]
    pub fn neg28d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg28dBm)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg40dBm)
    }
    #[doc = "-46 dBm"]
    #[inline(always)]
    pub fn neg46d_bm(self) -> &'a mut crate::W<REG> {
        self.variant(Txpower::Neg46dBm)
    }
}
impl R {
    #[doc = "Bits 0:10 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&self) -> TxpowerR {
        TxpowerR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - RADIO output power"]
    #[inline(always)]
    pub fn txpower(&mut self) -> TxpowerW<TxpowerSpec> {
        TxpowerW::new(self, 0)
    }
}
#[doc = "Output power\n\nYou can [`read`](crate::Reg::read) this register and get [`txpower::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpower::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpowerSpec;
impl crate::RegisterSpec for TxpowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpower::R`](R) reader structure"]
impl crate::Readable for TxpowerSpec {}
#[doc = "`write(|w| ..)` method takes [`txpower::W`](W) writer structure"]
impl crate::Writable for TxpowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPOWER to value 0x13"]
impl crate::Resettable for TxpowerSpec {
    const RESET_VALUE: u32 = 0x13;
}
