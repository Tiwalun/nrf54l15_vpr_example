#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Active shield detector detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Activeshield {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Activeshield> for bool {
    #[inline(always)]
    fn from(variant: Activeshield) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVESHIELD` reader - Active shield detector detected an error."]
pub type ActiveshieldR = crate::BitReader<Activeshield>;
impl ActiveshieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Activeshield {
        match self.bits {
            false => Activeshield::NotDetected,
            true => Activeshield::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Activeshield::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Activeshield::Detected
    }
}
#[doc = "Field `ACTIVESHIELD` writer - Active shield detector detected an error."]
pub type ActiveshieldW<'a, REG> = crate::BitWriter1C<'a, REG, Activeshield>;
impl<'a, REG> ActiveshieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Activeshield::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Activeshield::Detected)
    }
}
#[doc = "Error detected for the protected signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTECT` reader - Error detected for the protected signals."]
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::NotDetected,
            true => Protect::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Protect::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Protect::Detected
    }
}
#[doc = "Field `PROTECT` writer - Error detected for the protected signals."]
pub type ProtectW<'a, REG> = crate::BitWriter1C<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Detected)
    }
}
#[doc = "CRACEN detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cracentamp {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Cracentamp> for bool {
    #[inline(always)]
    fn from(variant: Cracentamp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRACENTAMP` reader - CRACEN detected an error."]
pub type CracentampR = crate::BitReader<Cracentamp>;
impl CracentampR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cracentamp {
        match self.bits {
            false => Cracentamp::NotDetected,
            true => Cracentamp::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Cracentamp::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Cracentamp::Detected
    }
}
#[doc = "Field `CRACENTAMP` writer - CRACEN detected an error."]
pub type CracentampW<'a, REG> = crate::BitWriter1C<'a, REG, Cracentamp>;
impl<'a, REG> CracentampW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Cracentamp::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Cracentamp::Detected)
    }
}
#[doc = "Slow domain glitch detector 0 detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glitchslowdomain0 {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Glitchslowdomain0> for bool {
    #[inline(always)]
    fn from(variant: Glitchslowdomain0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLITCHSLOWDOMAIN_0` reader - Slow domain glitch detector 0 detected an error."]
pub type Glitchslowdomain0R = crate::BitReader<Glitchslowdomain0>;
impl Glitchslowdomain0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glitchslowdomain0 {
        match self.bits {
            false => Glitchslowdomain0::NotDetected,
            true => Glitchslowdomain0::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Glitchslowdomain0::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Glitchslowdomain0::Detected
    }
}
#[doc = "Field `GLITCHSLOWDOMAIN_0` writer - Slow domain glitch detector 0 detected an error."]
pub type Glitchslowdomain0W<'a, REG> = crate::BitWriter1C<'a, REG, Glitchslowdomain0>;
impl<'a, REG> Glitchslowdomain0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchslowdomain0::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchslowdomain0::Detected)
    }
}
#[doc = "Fast domain glitch detector 0 detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glitchfastdomain0 {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Glitchfastdomain0> for bool {
    #[inline(always)]
    fn from(variant: Glitchfastdomain0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_0` reader - Fast domain glitch detector 0 detected an error."]
pub type Glitchfastdomain0R = crate::BitReader<Glitchfastdomain0>;
impl Glitchfastdomain0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glitchfastdomain0 {
        match self.bits {
            false => Glitchfastdomain0::NotDetected,
            true => Glitchfastdomain0::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Glitchfastdomain0::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Glitchfastdomain0::Detected
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_0` writer - Fast domain glitch detector 0 detected an error."]
pub type Glitchfastdomain0W<'a, REG> = crate::BitWriter1C<'a, REG, Glitchfastdomain0>;
impl<'a, REG> Glitchfastdomain0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain0::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain0::Detected)
    }
}
#[doc = "Fast domain glitch detector 1 detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glitchfastdomain1 {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Glitchfastdomain1> for bool {
    #[inline(always)]
    fn from(variant: Glitchfastdomain1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_1` reader - Fast domain glitch detector 1 detected an error."]
pub type Glitchfastdomain1R = crate::BitReader<Glitchfastdomain1>;
impl Glitchfastdomain1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glitchfastdomain1 {
        match self.bits {
            false => Glitchfastdomain1::NotDetected,
            true => Glitchfastdomain1::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Glitchfastdomain1::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Glitchfastdomain1::Detected
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_1` writer - Fast domain glitch detector 1 detected an error."]
pub type Glitchfastdomain1W<'a, REG> = crate::BitWriter1C<'a, REG, Glitchfastdomain1>;
impl<'a, REG> Glitchfastdomain1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain1::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain1::Detected)
    }
}
#[doc = "Fast domain glitch detector 2 detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glitchfastdomain2 {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Glitchfastdomain2> for bool {
    #[inline(always)]
    fn from(variant: Glitchfastdomain2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_2` reader - Fast domain glitch detector 2 detected an error."]
pub type Glitchfastdomain2R = crate::BitReader<Glitchfastdomain2>;
impl Glitchfastdomain2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glitchfastdomain2 {
        match self.bits {
            false => Glitchfastdomain2::NotDetected,
            true => Glitchfastdomain2::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Glitchfastdomain2::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Glitchfastdomain2::Detected
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_2` writer - Fast domain glitch detector 2 detected an error."]
pub type Glitchfastdomain2W<'a, REG> = crate::BitWriter1C<'a, REG, Glitchfastdomain2>;
impl<'a, REG> Glitchfastdomain2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain2::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain2::Detected)
    }
}
#[doc = "Fast domain glitch detector 3 detected an error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glitchfastdomain3 {
    #[doc = "0: Not detected."]
    NotDetected = 0,
    #[doc = "1: Detected."]
    Detected = 1,
}
impl From<Glitchfastdomain3> for bool {
    #[inline(always)]
    fn from(variant: Glitchfastdomain3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_3` reader - Fast domain glitch detector 3 detected an error."]
pub type Glitchfastdomain3R = crate::BitReader<Glitchfastdomain3>;
impl Glitchfastdomain3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glitchfastdomain3 {
        match self.bits {
            false => Glitchfastdomain3::NotDetected,
            true => Glitchfastdomain3::Detected,
        }
    }
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Glitchfastdomain3::NotDetected
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Glitchfastdomain3::Detected
    }
}
#[doc = "Field `GLITCHFASTDOMAIN_3` writer - Fast domain glitch detector 3 detected an error."]
pub type Glitchfastdomain3W<'a, REG> = crate::BitWriter1C<'a, REG, Glitchfastdomain3>;
impl<'a, REG> Glitchfastdomain3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain3::NotDetected)
    }
    #[doc = "Detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Glitchfastdomain3::Detected)
    }
}
impl R {
    #[doc = "Bit 0 - Active shield detector detected an error."]
    #[inline(always)]
    pub fn activeshield(&self) -> ActiveshieldR {
        ActiveshieldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Error detected for the protected signals."]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CRACEN detected an error."]
    #[inline(always)]
    pub fn cracentamp(&self) -> CracentampR {
        CracentampR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Slow domain glitch detector 0 detected an error."]
    #[inline(always)]
    pub fn glitchslowdomain_0(&self) -> Glitchslowdomain0R {
        Glitchslowdomain0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast domain glitch detector 0 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_0(&self) -> Glitchfastdomain0R {
        Glitchfastdomain0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast domain glitch detector 1 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_1(&self) -> Glitchfastdomain1R {
        Glitchfastdomain1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast domain glitch detector 2 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_2(&self) -> Glitchfastdomain2R {
        Glitchfastdomain2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast domain glitch detector 3 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_3(&self) -> Glitchfastdomain3R {
        Glitchfastdomain3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active shield detector detected an error."]
    #[inline(always)]
    pub fn activeshield(&mut self) -> ActiveshieldW<StatusSpec> {
        ActiveshieldW::new(self, 0)
    }
    #[doc = "Bit 4 - Error detected for the protected signals."]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<StatusSpec> {
        ProtectW::new(self, 4)
    }
    #[doc = "Bit 5 - CRACEN detected an error."]
    #[inline(always)]
    pub fn cracentamp(&mut self) -> CracentampW<StatusSpec> {
        CracentampW::new(self, 5)
    }
    #[doc = "Bit 8 - Slow domain glitch detector 0 detected an error."]
    #[inline(always)]
    pub fn glitchslowdomain_0(&mut self) -> Glitchslowdomain0W<StatusSpec> {
        Glitchslowdomain0W::new(self, 8)
    }
    #[doc = "Bit 12 - Fast domain glitch detector 0 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_0(&mut self) -> Glitchfastdomain0W<StatusSpec> {
        Glitchfastdomain0W::new(self, 12)
    }
    #[doc = "Bit 13 - Fast domain glitch detector 1 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_1(&mut self) -> Glitchfastdomain1W<StatusSpec> {
        Glitchfastdomain1W::new(self, 13)
    }
    #[doc = "Bit 14 - Fast domain glitch detector 2 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_2(&mut self) -> Glitchfastdomain2W<StatusSpec> {
        Glitchfastdomain2W::new(self, 14)
    }
    #[doc = "Bit 15 - Fast domain glitch detector 3 detected an error."]
    #[inline(always)]
    pub fn glitchfastdomain_3(&mut self) -> Glitchfastdomain3W<StatusSpec> {
        Glitchfastdomain3W::new(self, 15)
    }
}
#[doc = "The tamper controller status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf131;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
