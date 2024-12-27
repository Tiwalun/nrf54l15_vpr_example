#[doc = "Register `TASKS_PLLSTOP` writer"]
pub type W = crate::W<TasksPllstopSpec>;
#[doc = "Stop PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPllstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPllstop> for bool {
    #[inline(always)]
    fn from(variant: TasksPllstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PLLSTOP` writer - Stop PLL"]
pub type TasksPllstopW<'a, REG> = crate::BitWriter<'a, REG, TasksPllstop>;
impl<'a, REG> TasksPllstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPllstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PLL"]
    #[inline(always)]
    pub fn tasks_pllstop(&mut self) -> TasksPllstopW<TasksPllstopSpec> {
        TasksPllstopW::new(self, 0)
    }
}
#[doc = "Stop PLL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pllstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPllstopSpec;
impl crate::RegisterSpec for TasksPllstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pllstop::W`](W) writer structure"]
impl crate::Writable for TasksPllstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PLLSTOP to value 0"]
impl crate::Resettable for TasksPllstopSpec {
    const RESET_VALUE: u32 = 0;
}
