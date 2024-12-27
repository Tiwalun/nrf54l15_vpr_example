#[doc = "Register `TRACEPORTSPEED` reader"]
pub type R = crate::R<TraceportspeedSpec>;
#[doc = "Register `TRACEPORTSPEED` writer"]
pub type W = crate::W<TraceportspeedSpec>;
#[doc = "Trace port speed is divided from CPU clock. The TRACECLK pin output will be divided again by two from the trace port clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Traceportspeed {
    #[doc = "0: Trace port speed equals CPU clock"]
    Div1 = 0,
    #[doc = "1: Trace port speed equals CPU clock divided by 2"]
    Div2 = 1,
    #[doc = "2: Trace port speed equals CPU clock divided by 4"]
    Div4 = 2,
    #[doc = "3: Trace port speed equals CPU clock divided by 32"]
    Div32 = 3,
}
impl From<Traceportspeed> for u8 {
    #[inline(always)]
    fn from(variant: Traceportspeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Traceportspeed {
    type Ux = u8;
}
impl crate::IsEnum for Traceportspeed {}
#[doc = "Field `TRACEPORTSPEED` reader - Trace port speed is divided from CPU clock. The TRACECLK pin output will be divided again by two from the trace port clock."]
pub type TraceportspeedR = crate::FieldReader<Traceportspeed>;
impl TraceportspeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Traceportspeed {
        match self.bits {
            0 => Traceportspeed::Div1,
            1 => Traceportspeed::Div2,
            2 => Traceportspeed::Div4,
            3 => Traceportspeed::Div32,
            _ => unreachable!(),
        }
    }
    #[doc = "Trace port speed equals CPU clock"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Traceportspeed::Div1
    }
    #[doc = "Trace port speed equals CPU clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Traceportspeed::Div2
    }
    #[doc = "Trace port speed equals CPU clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Traceportspeed::Div4
    }
    #[doc = "Trace port speed equals CPU clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Traceportspeed::Div32
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Trace port speed is divided from CPU clock. The TRACECLK pin output will be divided again by two from the trace port clock."]
pub type TraceportspeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Traceportspeed, crate::Safe>;
impl<'a, REG> TraceportspeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trace port speed equals CPU clock"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::Div1)
    }
    #[doc = "Trace port speed equals CPU clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::Div2)
    }
    #[doc = "Trace port speed equals CPU clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::Div4)
    }
    #[doc = "Trace port speed equals CPU clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::Div32)
    }
}
impl R {
    #[doc = "Bits 0:1 - Trace port speed is divided from CPU clock. The TRACECLK pin output will be divided again by two from the trace port clock."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TraceportspeedR {
        TraceportspeedR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trace port speed is divided from CPU clock. The TRACECLK pin output will be divided again by two from the trace port clock."]
    #[inline(always)]
    #[must_use]
    pub fn traceportspeed(&mut self) -> TraceportspeedW<TraceportspeedSpec> {
        TraceportspeedW::new(self, 0)
    }
}
#[doc = "Trace port speed\n\nYou can [`read`](crate::Reg::read) this register and get [`traceportspeed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceportspeed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceportspeedSpec;
impl crate::RegisterSpec for TraceportspeedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceportspeed::R`](R) reader structure"]
impl crate::Readable for TraceportspeedSpec {}
#[doc = "`write(|w| ..)` method takes [`traceportspeed::W`](W) writer structure"]
impl crate::Writable for TraceportspeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACEPORTSPEED to value 0"]
impl crate::Resettable for TraceportspeedSpec {
    const RESET_VALUE: u32 = 0;
}
