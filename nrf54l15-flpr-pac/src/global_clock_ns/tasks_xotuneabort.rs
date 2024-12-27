#[doc = "Register `TASKS_XOTUNEABORT` writer"]
pub type W = crate::W<TasksXotuneabortSpec>;
#[doc = "Abort tuning for HFXO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksXotuneabort {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksXotuneabort> for bool {
    #[inline(always)]
    fn from(variant: TasksXotuneabort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_XOTUNEABORT` writer - Abort tuning for HFXO"]
pub type TasksXotuneabortW<'a, REG> = crate::BitWriter<'a, REG, TasksXotuneabort>;
impl<'a, REG> TasksXotuneabortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksXotuneabort::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Abort tuning for HFXO"]
    #[inline(always)]
    pub fn tasks_xotuneabort(&mut self) -> TasksXotuneabortW<TasksXotuneabortSpec> {
        TasksXotuneabortW::new(self, 0)
    }
}
#[doc = "Abort tuning for HFXO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xotuneabort::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksXotuneabortSpec;
impl crate::RegisterSpec for TasksXotuneabortSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_xotuneabort::W`](W) writer structure"]
impl crate::Writable for TasksXotuneabortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_XOTUNEABORT to value 0"]
impl crate::Resettable for TasksXotuneabortSpec {
    const RESET_VALUE: u32 = 0;
}
