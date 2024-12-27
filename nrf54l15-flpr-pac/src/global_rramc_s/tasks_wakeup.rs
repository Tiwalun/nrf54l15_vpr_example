#[doc = "Register `TASKS_WAKEUP` writer"]
pub type W = crate::W<TasksWakeupSpec>;
#[doc = "Wakeup the RRAM from low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksWakeup {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksWakeup> for bool {
    #[inline(always)]
    fn from(variant: TasksWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_WAKEUP` writer - Wakeup the RRAM from low power mode"]
pub type TasksWakeupW<'a, REG> = crate::BitWriter<'a, REG, TasksWakeup>;
impl<'a, REG> TasksWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksWakeup::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup the RRAM from low power mode"]
    #[inline(always)]
    pub fn tasks_wakeup(&mut self) -> TasksWakeupW<TasksWakeupSpec> {
        TasksWakeupW::new(self, 0)
    }
}
#[doc = "Wakeup the RRAM from low power mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_wakeup::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksWakeupSpec;
impl crate::RegisterSpec for TasksWakeupSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_wakeup::W`](W) writer structure"]
impl crate::Writable for TasksWakeupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_WAKEUP to value 0"]
impl crate::Resettable for TasksWakeupSpec {
    const RESET_VALUE: u32 = 0;
}
