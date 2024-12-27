#[doc = "Register `WRITEBUFEMPTY` reader"]
pub type R = crate::R<WritebufemptySpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Empty {
    #[doc = "0: The internal write-buffer has data that needs committing"]
    NotEmpty = 0,
    #[doc = "1: The internal write-buffer is empty and has no content that needs to be committed"]
    Empty = 1,
}
impl From<Empty> for bool {
    #[inline(always)]
    fn from(variant: Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - "]
pub type EmptyR = crate::BitReader<Empty>;
impl EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Empty {
        match self.bits {
            false => Empty::NotEmpty,
            true => Empty::Empty,
        }
    }
    #[doc = "The internal write-buffer has data that needs committing"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Empty::NotEmpty
    }
    #[doc = "The internal write-buffer is empty and has no content that needs to be committed"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Empty::Empty
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Internal write-buffer is empty\n\nYou can [`read`](crate::Reg::read) this register and get [`writebufempty::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritebufemptySpec;
impl crate::RegisterSpec for WritebufemptySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writebufempty::R`](R) reader structure"]
impl crate::Readable for WritebufemptySpec {}
#[doc = "`reset()` method sets WRITEBUFEMPTY to value 0"]
impl crate::Resettable for WritebufemptySpec {
    const RESET_VALUE: u32 = 0;
}
