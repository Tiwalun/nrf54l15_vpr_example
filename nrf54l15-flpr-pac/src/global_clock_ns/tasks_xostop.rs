#[doc = "Register `TASKS_XOSTOP` writer"]
pub type W = crate::W<TasksXostopSpec>;
#[doc = "Stop crystal oscillator (HFXO)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksXostop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksXostop> for bool {
    #[inline(always)]
    fn from(variant: TasksXostop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_XOSTOP` writer - Stop crystal oscillator (HFXO)"]
pub type TasksXostopW<'a, REG> = crate::BitWriter<'a, REG, TasksXostop>;
impl<'a, REG> TasksXostopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksXostop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop crystal oscillator (HFXO)"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_xostop(&mut self) -> TasksXostopW<TasksXostopSpec> {
        TasksXostopW::new(self, 0)
    }
}
#[doc = "Stop crystal oscillator (HFXO)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xostop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksXostopSpec;
impl crate::RegisterSpec for TasksXostopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_xostop::W`](W) writer structure"]
impl crate::Writable for TasksXostopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_XOSTOP to value 0"]
impl crate::Resettable for TasksXostopSpec {
    const RESET_VALUE: u32 = 0;
}
