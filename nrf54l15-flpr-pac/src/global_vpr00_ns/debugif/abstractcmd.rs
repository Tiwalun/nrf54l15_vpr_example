#[doc = "Register `ABSTRACTCMD` writer"]
pub type W = crate::W<AbstractcmdSpec>;
#[doc = "Field `CONTROL` writer - This Field is interpreted in a command specific manner, described for each abstract command."]
pub type ControlW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "The type determines the overall functionality of this abstract command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmdtype {
    #[doc = "0: Register Access Command"]
    Regaccess = 0,
    #[doc = "1: Quick Access Command"]
    Quickaccess = 1,
    #[doc = "2: Memory Access Command"]
    Memaccess = 2,
}
impl From<Cmdtype> for u8 {
    #[inline(always)]
    fn from(variant: Cmdtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmdtype {
    type Ux = u8;
}
impl crate::IsEnum for Cmdtype {}
#[doc = "Field `CMDTYPE` writer - The type determines the overall functionality of this abstract command."]
pub type CmdtypeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cmdtype>;
impl<'a, REG> CmdtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Register Access Command"]
    #[inline(always)]
    pub fn regaccess(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Regaccess)
    }
    #[doc = "Quick Access Command"]
    #[inline(always)]
    pub fn quickaccess(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Quickaccess)
    }
    #[doc = "Memory Access Command"]
    #[inline(always)]
    pub fn memaccess(self) -> &'a mut crate::W<REG> {
        self.variant(Cmdtype::Memaccess)
    }
}
impl W {
    #[doc = "Bits 0:23 - This Field is interpreted in a command specific manner, described for each abstract command."]
    #[inline(always)]
    pub fn control(&mut self) -> ControlW<AbstractcmdSpec> {
        ControlW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The type determines the overall functionality of this abstract command."]
    #[inline(always)]
    pub fn cmdtype(&mut self) -> CmdtypeW<AbstractcmdSpec> {
        CmdtypeW::new(self, 24)
    }
}
#[doc = "Abstract command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abstractcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbstractcmdSpec;
impl crate::RegisterSpec for AbstractcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`abstractcmd::W`](W) writer structure"]
impl crate::Writable for AbstractcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABSTRACTCMD to value 0"]
impl crate::Resettable for AbstractcmdSpec {
    const RESET_VALUE: u32 = 0;
}
