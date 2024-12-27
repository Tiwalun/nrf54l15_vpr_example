#[doc = "Register `TASKS_XOSTART` writer"]
pub type W = crate::W<TasksXostartSpec>;
#[doc = "Start crystal oscillator (HFXO)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksXostart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksXostart> for bool {
    #[inline(always)]
    fn from(variant: TasksXostart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_XOSTART` writer - Start crystal oscillator (HFXO)"]
pub type TasksXostartW<'a, REG> = crate::BitWriter<'a, REG, TasksXostart>;
impl<'a, REG> TasksXostartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksXostart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start crystal oscillator (HFXO)"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_xostart(&mut self) -> TasksXostartW<TasksXostartSpec> {
        TasksXostartW::new(self, 0)
    }
}
#[doc = "Start crystal oscillator (HFXO)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_xostart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksXostartSpec;
impl crate::RegisterSpec for TasksXostartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_xostart::W`](W) writer structure"]
impl crate::Writable for TasksXostartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_XOSTART to value 0"]
impl crate::Resettable for TasksXostartSpec {
    const RESET_VALUE: u32 = 0;
}
