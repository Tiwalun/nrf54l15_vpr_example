#[doc = "Register `LIST` reader"]
pub type R = crate::R<ListSpec>;
#[doc = "Register `LIST` writer"]
pub type W = crate::W<ListSpec>;
#[doc = "List type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "0: Disable EasyDMA list"]
    Disabled = 0,
    #[doc = "1: Use array list"]
    ArrayList = 1,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `TYPE` reader - List type"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Type> {
        match self.bits {
            0 => Some(Type::Disabled),
            1 => Some(Type::ArrayList),
            _ => None,
        }
    }
    #[doc = "Disable EasyDMA list"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Type::Disabled
    }
    #[doc = "Use array list"]
    #[inline(always)]
    pub fn is_array_list(&self) -> bool {
        *self == Type::ArrayList
    }
}
#[doc = "Field `TYPE` writer - List type"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Type>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable EasyDMA list"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Type::Disabled)
    }
    #[doc = "Use array list"]
    #[inline(always)]
    pub fn array_list(self) -> &'a mut crate::W<REG> {
        self.variant(Type::ArrayList)
    }
}
impl R {
    #[doc = "Bits 0:2 - List type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - List type"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<ListSpec> {
        TypeW::new(self, 0)
    }
}
#[doc = "EasyDMA list type\n\nYou can [`read`](crate::Reg::read) this register and get [`list::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`list::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ListSpec;
impl crate::RegisterSpec for ListSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`list::R`](R) reader structure"]
impl crate::Readable for ListSpec {}
#[doc = "`write(|w| ..)` method takes [`list::W`](W) writer structure"]
impl crate::Writable for ListSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIST to value 0"]
impl crate::Resettable for ListSpec {
    const RESET_VALUE: u32 = 0;
}
