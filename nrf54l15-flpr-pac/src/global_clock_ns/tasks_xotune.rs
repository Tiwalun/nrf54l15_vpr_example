#[doc = "Register `TASKS_XOTUNE` writer"]
pub type W = crate::W<TasksXotuneSpec>;
#[doc = "Request tuning for HFXO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksXotune {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksXotune> for bool {
    #[inline(always)]
    fn from(variant: TasksXotune) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_XOTUNE` writer - Request tuning for HFXO"]
pub type TasksXotuneW<'a, REG> = crate::BitWriter<'a, REG, TasksXotune>;
impl<'a, REG> TasksXotuneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksXotune::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Request tuning for HFXO"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_xotune(&mut self) -> TasksXotuneW<TasksXotuneSpec> {
        TasksXotuneW::new(self, 0)
    }
}
#[doc = "Request tuning for HFXO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xotune::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksXotuneSpec;
impl crate::RegisterSpec for TasksXotuneSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_xotune::W`](W) writer structure"]
impl crate::Writable for TasksXotuneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_XOTUNE to value 0"]
impl crate::Resettable for TasksXotuneSpec {
    const RESET_VALUE: u32 = 0;
}
