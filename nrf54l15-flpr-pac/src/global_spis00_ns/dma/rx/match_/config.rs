#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Enable match filter 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable0 {
    #[doc = "0: Match filter disabled"]
    Disabled = 0,
    #[doc = "1: Match filter enabled"]
    Enabled = 1,
}
impl From<Enable0> for bool {
    #[inline(always)]
    fn from(variant: Enable0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_0` reader - Enable match filter 0"]
pub type Enable0R = crate::BitReader<Enable0>;
impl Enable0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable0 {
        match self.bits {
            false => Enable0::Disabled,
            true => Enable0::Enabled,
        }
    }
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable0::Disabled
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable0::Enabled
    }
}
#[doc = "Field `ENABLE_0` writer - Enable match filter 0"]
pub type Enable0W<'a, REG> = crate::BitWriter<'a, REG, Enable0>;
impl<'a, REG> Enable0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable0::Disabled)
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable0::Enabled)
    }
}
#[doc = "Enable match filter 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable1 {
    #[doc = "0: Match filter disabled"]
    Disabled = 0,
    #[doc = "1: Match filter enabled"]
    Enabled = 1,
}
impl From<Enable1> for bool {
    #[inline(always)]
    fn from(variant: Enable1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_1` reader - Enable match filter 1"]
pub type Enable1R = crate::BitReader<Enable1>;
impl Enable1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable1 {
        match self.bits {
            false => Enable1::Disabled,
            true => Enable1::Enabled,
        }
    }
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable1::Disabled
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable1::Enabled
    }
}
#[doc = "Field `ENABLE_1` writer - Enable match filter 1"]
pub type Enable1W<'a, REG> = crate::BitWriter<'a, REG, Enable1>;
impl<'a, REG> Enable1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable1::Disabled)
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable1::Enabled)
    }
}
#[doc = "Enable match filter 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable2 {
    #[doc = "0: Match filter disabled"]
    Disabled = 0,
    #[doc = "1: Match filter enabled"]
    Enabled = 1,
}
impl From<Enable2> for bool {
    #[inline(always)]
    fn from(variant: Enable2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_2` reader - Enable match filter 2"]
pub type Enable2R = crate::BitReader<Enable2>;
impl Enable2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable2 {
        match self.bits {
            false => Enable2::Disabled,
            true => Enable2::Enabled,
        }
    }
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable2::Disabled
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable2::Enabled
    }
}
#[doc = "Field `ENABLE_2` writer - Enable match filter 2"]
pub type Enable2W<'a, REG> = crate::BitWriter<'a, REG, Enable2>;
impl<'a, REG> Enable2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable2::Disabled)
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable2::Enabled)
    }
}
#[doc = "Enable match filter 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable3 {
    #[doc = "0: Match filter disabled"]
    Disabled = 0,
    #[doc = "1: Match filter enabled"]
    Enabled = 1,
}
impl From<Enable3> for bool {
    #[inline(always)]
    fn from(variant: Enable3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE_3` reader - Enable match filter 3"]
pub type Enable3R = crate::BitReader<Enable3>;
impl Enable3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable3 {
        match self.bits {
            false => Enable3::Disabled,
            true => Enable3::Enabled,
        }
    }
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable3::Disabled
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable3::Enabled
    }
}
#[doc = "Field `ENABLE_3` writer - Enable match filter 3"]
pub type Enable3W<'a, REG> = crate::BitWriter<'a, REG, Enable3>;
impl<'a, REG> Enable3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable3::Disabled)
    }
    #[doc = "Match filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable3::Enabled)
    }
}
#[doc = "Configure match filter 0 as one-shot or sticky\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshot0 {
    #[doc = "0: Match filter stays enabled until disabled by task"]
    Continuous = 0,
    #[doc = "1: Match filter stays enabled until next data word is received"]
    Oneshot = 1,
}
impl From<Oneshot0> for bool {
    #[inline(always)]
    fn from(variant: Oneshot0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT_0` reader - Configure match filter 0 as one-shot or sticky"]
pub type Oneshot0R = crate::BitReader<Oneshot0>;
impl Oneshot0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshot0 {
        match self.bits {
            false => Oneshot0::Continuous,
            true => Oneshot0::Oneshot,
        }
    }
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Oneshot0::Continuous
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Oneshot0::Oneshot
    }
}
#[doc = "Field `ONESHOT_0` writer - Configure match filter 0 as one-shot or sticky"]
pub type Oneshot0W<'a, REG> = crate::BitWriter<'a, REG, Oneshot0>;
impl<'a, REG> Oneshot0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot0::Continuous)
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot0::Oneshot)
    }
}
#[doc = "Configure match filter 1 as one-shot or sticky\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshot1 {
    #[doc = "0: Match filter stays enabled until disabled by task"]
    Continuous = 0,
    #[doc = "1: Match filter stays enabled until next data word is received"]
    Oneshot = 1,
}
impl From<Oneshot1> for bool {
    #[inline(always)]
    fn from(variant: Oneshot1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT_1` reader - Configure match filter 1 as one-shot or sticky"]
pub type Oneshot1R = crate::BitReader<Oneshot1>;
impl Oneshot1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshot1 {
        match self.bits {
            false => Oneshot1::Continuous,
            true => Oneshot1::Oneshot,
        }
    }
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Oneshot1::Continuous
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Oneshot1::Oneshot
    }
}
#[doc = "Field `ONESHOT_1` writer - Configure match filter 1 as one-shot or sticky"]
pub type Oneshot1W<'a, REG> = crate::BitWriter<'a, REG, Oneshot1>;
impl<'a, REG> Oneshot1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot1::Continuous)
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot1::Oneshot)
    }
}
#[doc = "Configure match filter 2 as one-shot or sticky\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshot2 {
    #[doc = "0: Match filter stays enabled until disabled by task"]
    Continuous = 0,
    #[doc = "1: Match filter stays enabled until next data word is received"]
    Oneshot = 1,
}
impl From<Oneshot2> for bool {
    #[inline(always)]
    fn from(variant: Oneshot2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT_2` reader - Configure match filter 2 as one-shot or sticky"]
pub type Oneshot2R = crate::BitReader<Oneshot2>;
impl Oneshot2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshot2 {
        match self.bits {
            false => Oneshot2::Continuous,
            true => Oneshot2::Oneshot,
        }
    }
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Oneshot2::Continuous
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Oneshot2::Oneshot
    }
}
#[doc = "Field `ONESHOT_2` writer - Configure match filter 2 as one-shot or sticky"]
pub type Oneshot2W<'a, REG> = crate::BitWriter<'a, REG, Oneshot2>;
impl<'a, REG> Oneshot2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot2::Continuous)
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot2::Oneshot)
    }
}
#[doc = "Configure match filter 3 as one-shot or sticky\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oneshot3 {
    #[doc = "0: Match filter stays enabled until disabled by task"]
    Continuous = 0,
    #[doc = "1: Match filter stays enabled until next data word is received"]
    Oneshot = 1,
}
impl From<Oneshot3> for bool {
    #[inline(always)]
    fn from(variant: Oneshot3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT_3` reader - Configure match filter 3 as one-shot or sticky"]
pub type Oneshot3R = crate::BitReader<Oneshot3>;
impl Oneshot3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oneshot3 {
        match self.bits {
            false => Oneshot3::Continuous,
            true => Oneshot3::Oneshot,
        }
    }
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Oneshot3::Continuous
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn is_oneshot(&self) -> bool {
        *self == Oneshot3::Oneshot
    }
}
#[doc = "Field `ONESHOT_3` writer - Configure match filter 3 as one-shot or sticky"]
pub type Oneshot3W<'a, REG> = crate::BitWriter<'a, REG, Oneshot3>;
impl<'a, REG> Oneshot3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match filter stays enabled until disabled by task"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot3::Continuous)
    }
    #[doc = "Match filter stays enabled until next data word is received"]
    #[inline(always)]
    pub fn oneshot(self) -> &'a mut crate::W<REG> {
        self.variant(Oneshot3::Oneshot)
    }
}
impl R {
    #[doc = "Bit 0 - Enable match filter 0"]
    #[inline(always)]
    pub fn enable_0(&self) -> Enable0R {
        Enable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable match filter 1"]
    #[inline(always)]
    pub fn enable_1(&self) -> Enable1R {
        Enable1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable match filter 2"]
    #[inline(always)]
    pub fn enable_2(&self) -> Enable2R {
        Enable2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable match filter 3"]
    #[inline(always)]
    pub fn enable_3(&self) -> Enable3R {
        Enable3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Configure match filter 0 as one-shot or sticky"]
    #[inline(always)]
    pub fn oneshot_0(&self) -> Oneshot0R {
        Oneshot0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configure match filter 1 as one-shot or sticky"]
    #[inline(always)]
    pub fn oneshot_1(&self) -> Oneshot1R {
        Oneshot1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configure match filter 2 as one-shot or sticky"]
    #[inline(always)]
    pub fn oneshot_2(&self) -> Oneshot2R {
        Oneshot2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configure match filter 3 as one-shot or sticky"]
    #[inline(always)]
    pub fn oneshot_3(&self) -> Oneshot3R {
        Oneshot3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable match filter 0"]
    #[inline(always)]
    #[must_use]
    pub fn enable_0(&mut self) -> Enable0W<ConfigSpec> {
        Enable0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable match filter 1"]
    #[inline(always)]
    #[must_use]
    pub fn enable_1(&mut self) -> Enable1W<ConfigSpec> {
        Enable1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable match filter 2"]
    #[inline(always)]
    #[must_use]
    pub fn enable_2(&mut self) -> Enable2W<ConfigSpec> {
        Enable2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable match filter 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable_3(&mut self) -> Enable3W<ConfigSpec> {
        Enable3W::new(self, 3)
    }
    #[doc = "Bit 16 - Configure match filter 0 as one-shot or sticky"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot_0(&mut self) -> Oneshot0W<ConfigSpec> {
        Oneshot0W::new(self, 16)
    }
    #[doc = "Bit 17 - Configure match filter 1 as one-shot or sticky"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot_1(&mut self) -> Oneshot1W<ConfigSpec> {
        Oneshot1W::new(self, 17)
    }
    #[doc = "Bit 18 - Configure match filter 2 as one-shot or sticky"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot_2(&mut self) -> Oneshot2W<ConfigSpec> {
        Oneshot2W::new(self, 18)
    }
    #[doc = "Bit 19 - Configure match filter 3 as one-shot or sticky"]
    #[inline(always)]
    #[must_use]
    pub fn oneshot_3(&mut self) -> Oneshot3W<ConfigSpec> {
        Oneshot3W::new(self, 19)
    }
}
#[doc = "Configure individual match events\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
