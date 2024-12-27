#[doc = "Register `TASKS_PWMSTART` writer"]
pub type W = crate::W<TasksPwmstartSpec>;
#[doc = "Start the PWM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPwmstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPwmstart> for bool {
    #[inline(always)]
    fn from(variant: TasksPwmstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PWMSTART` writer - Start the PWM"]
pub type TasksPwmstartW<'a, REG> = crate::BitWriter<'a, REG, TasksPwmstart>;
impl<'a, REG> TasksPwmstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPwmstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start the PWM"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_pwmstart(&mut self) -> TasksPwmstartW<TasksPwmstartSpec> {
        TasksPwmstartW::new(self, 0)
    }
}
#[doc = "Start the PWM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pwmstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPwmstartSpec;
impl crate::RegisterSpec for TasksPwmstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pwmstart::W`](W) writer structure"]
impl crate::Writable for TasksPwmstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PWMSTART to value 0"]
impl crate::Resettable for TasksPwmstartSpec {
    const RESET_VALUE: u32 = 0;
}
