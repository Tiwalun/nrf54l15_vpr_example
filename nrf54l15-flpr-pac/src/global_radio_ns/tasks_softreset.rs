#[doc = "Register `TASKS_SOFTRESET` writer"]
pub type W = crate::W<TasksSoftresetSpec>;
#[doc = "Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSoftreset {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSoftreset> for bool {
    #[inline(always)]
    fn from(variant: TasksSoftreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SOFTRESET` writer - Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state."]
pub type TasksSoftresetW<'a, REG> = crate::BitWriter<'a, REG, TasksSoftreset>;
impl<'a, REG> TasksSoftresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSoftreset::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state."]
    #[inline(always)]
    pub fn tasks_softreset(&mut self) -> TasksSoftresetW<TasksSoftresetSpec> {
        TasksSoftresetW::new(self, 0)
    }
}
#[doc = "Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_softreset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSoftresetSpec;
impl crate::RegisterSpec for TasksSoftresetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_softreset::W`](W) writer structure"]
impl crate::Writable for TasksSoftresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_SOFTRESET to value 0"]
impl crate::Resettable for TasksSoftresetSpec {
    const RESET_VALUE: u32 = 0;
}
