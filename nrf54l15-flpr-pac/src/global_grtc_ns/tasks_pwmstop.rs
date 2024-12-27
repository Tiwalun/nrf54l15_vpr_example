#[doc = "Register `TASKS_PWMSTOP` writer"]
pub type W = crate::W<TasksPwmstopSpec>;
#[doc = "Stop the PWM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPwmstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPwmstop> for bool {
    #[inline(always)]
    fn from(variant: TasksPwmstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PWMSTOP` writer - Stop the PWM"]
pub type TasksPwmstopW<'a, REG> = crate::BitWriter<'a, REG, TasksPwmstop>;
impl<'a, REG> TasksPwmstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPwmstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the PWM"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_pwmstop(&mut self) -> TasksPwmstopW<TasksPwmstopSpec> {
        TasksPwmstopW::new(self, 0)
    }
}
#[doc = "Stop the PWM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pwmstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPwmstopSpec;
impl crate::RegisterSpec for TasksPwmstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pwmstop::W`](W) writer structure"]
impl crate::Writable for TasksPwmstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PWMSTOP to value 0"]
impl crate::Resettable for TasksPwmstopSpec {
    const RESET_VALUE: u32 = 0;
}
