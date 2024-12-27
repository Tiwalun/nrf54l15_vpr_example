#[doc = "Register `CHEN` reader"]
pub type R = crate::R<ChenSpec>;
#[doc = "Register `CHEN` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Enable or disable active shield channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Disable channel."]
    Disabled = 0,
    #[doc = "1: Enable channel."]
    Enabled = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH_0` reader - Enable or disable active shield channel 0."]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Disabled,
            true => Ch0::Enabled,
        }
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0::Disabled
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0::Enabled
    }
}
#[doc = "Field `CH_0` writer - Enable or disable active shield channel 0."]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Disabled)
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Enabled)
    }
}
#[doc = "Enable or disable active shield channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Disable channel."]
    Disabled = 0,
    #[doc = "1: Enable channel."]
    Enabled = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH_1` reader - Enable or disable active shield channel 1."]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Disabled,
            true => Ch1::Enabled,
        }
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1::Disabled
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1::Enabled
    }
}
#[doc = "Field `CH_1` writer - Enable or disable active shield channel 1."]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Disabled)
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Enabled)
    }
}
#[doc = "Enable or disable active shield channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Disable channel."]
    Disabled = 0,
    #[doc = "1: Enable channel."]
    Enabled = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH_2` reader - Enable or disable active shield channel 2."]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Disabled,
            true => Ch2::Enabled,
        }
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2::Disabled
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2::Enabled
    }
}
#[doc = "Field `CH_2` writer - Enable or disable active shield channel 2."]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Disabled)
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Enabled)
    }
}
#[doc = "Enable or disable active shield channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Disable channel."]
    Disabled = 0,
    #[doc = "1: Enable channel."]
    Enabled = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH_3` reader - Enable or disable active shield channel 3."]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Disabled,
            true => Ch3::Enabled,
        }
    }
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3::Disabled
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3::Enabled
    }
}
#[doc = "Field `CH_3` writer - Enable or disable active shield channel 3."]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Disabled)
    }
    #[doc = "Enable channel."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable active shield channel 0."]
    #[inline(always)]
    pub fn ch_0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable active shield channel 1."]
    #[inline(always)]
    pub fn ch_1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable active shield channel 2."]
    #[inline(always)]
    pub fn ch_2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable active shield channel 3."]
    #[inline(always)]
    pub fn ch_3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable active shield channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn ch_0(&mut self) -> Ch0W<ChenSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable active shield channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn ch_1(&mut self) -> Ch1W<ChenSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable active shield channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn ch_2(&mut self) -> Ch2W<ChenSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable active shield channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn ch_3(&mut self) -> Ch3W<ChenSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "Active shield detector channel enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for ChenSpec {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for ChenSpec {
    const RESET_VALUE: u32 = 0;
}
