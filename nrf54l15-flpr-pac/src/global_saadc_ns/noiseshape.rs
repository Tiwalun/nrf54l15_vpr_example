#[doc = "Register `NOISESHAPE` reader"]
pub type R = crate::R<NoiseshapeSpec>;
#[doc = "Register `NOISESHAPE` writer"]
pub type W = crate::W<NoiseshapeSpec>;
#[doc = "Enable noise shaping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Noiseshape {
    #[doc = "0: Disable noiseshaping. Oversampling based on accumulate and average."]
    Disable = 0,
    #[doc = "1: Noiseshaping and decimating. Larger passband. Provides a 50kS/s cut off frequency, 8x the oversampling ratio. See design description for more information"]
    Audio = 1,
    #[doc = "2: Noiseshaping and decimating. Smaller passband. Recommended resolution setting is 14 bits. Provides a 5kS/s cut off frequency, 32x the oversampling ratio. See design description for more information"]
    Accuracy = 2,
}
impl From<Noiseshape> for u8 {
    #[inline(always)]
    fn from(variant: Noiseshape) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Noiseshape {
    type Ux = u8;
}
impl crate::IsEnum for Noiseshape {}
#[doc = "Field `NOISESHAPE` reader - Enable noise shaping"]
pub type NoiseshapeR = crate::FieldReader<Noiseshape>;
impl NoiseshapeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Noiseshape> {
        match self.bits {
            0 => Some(Noiseshape::Disable),
            1 => Some(Noiseshape::Audio),
            2 => Some(Noiseshape::Accuracy),
            _ => None,
        }
    }
    #[doc = "Disable noiseshaping. Oversampling based on accumulate and average."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Noiseshape::Disable
    }
    #[doc = "Noiseshaping and decimating. Larger passband. Provides a 50kS/s cut off frequency, 8x the oversampling ratio. See design description for more information"]
    #[inline(always)]
    pub fn is_audio(&self) -> bool {
        *self == Noiseshape::Audio
    }
    #[doc = "Noiseshaping and decimating. Smaller passband. Recommended resolution setting is 14 bits. Provides a 5kS/s cut off frequency, 32x the oversampling ratio. See design description for more information"]
    #[inline(always)]
    pub fn is_accuracy(&self) -> bool {
        *self == Noiseshape::Accuracy
    }
}
#[doc = "Field `NOISESHAPE` writer - Enable noise shaping"]
pub type NoiseshapeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Noiseshape>;
impl<'a, REG> NoiseshapeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable noiseshaping. Oversampling based on accumulate and average."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Noiseshape::Disable)
    }
    #[doc = "Noiseshaping and decimating. Larger passband. Provides a 50kS/s cut off frequency, 8x the oversampling ratio. See design description for more information"]
    #[inline(always)]
    pub fn audio(self) -> &'a mut crate::W<REG> {
        self.variant(Noiseshape::Audio)
    }
    #[doc = "Noiseshaping and decimating. Smaller passband. Recommended resolution setting is 14 bits. Provides a 5kS/s cut off frequency, 32x the oversampling ratio. See design description for more information"]
    #[inline(always)]
    pub fn accuracy(self) -> &'a mut crate::W<REG> {
        self.variant(Noiseshape::Accuracy)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable noise shaping"]
    #[inline(always)]
    pub fn noiseshape(&self) -> NoiseshapeR {
        NoiseshapeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable noise shaping"]
    #[inline(always)]
    pub fn noiseshape(&mut self) -> NoiseshapeW<NoiseshapeSpec> {
        NoiseshapeW::new(self, 0)
    }
}
#[doc = "Enable noise shaping\n\nYou can [`read`](crate::Reg::read) this register and get [`noiseshape::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noiseshape::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NoiseshapeSpec;
impl crate::RegisterSpec for NoiseshapeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`noiseshape::R`](R) reader structure"]
impl crate::Readable for NoiseshapeSpec {}
#[doc = "`write(|w| ..)` method takes [`noiseshape::W`](W) writer structure"]
impl crate::Writable for NoiseshapeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NOISESHAPE to value 0"]
impl crate::Resettable for NoiseshapeSpec {
    const RESET_VALUE: u32 = 0;
}
