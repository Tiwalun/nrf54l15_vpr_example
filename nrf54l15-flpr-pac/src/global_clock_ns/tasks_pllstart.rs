#[doc = "Register `TASKS_PLLSTART` writer"]
pub type W = crate::W<TasksPllstartSpec>;
#[doc = "Start PLL and keep it running, regardless of the automatic clock requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPllstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPllstart> for bool {
    #[inline(always)]
    fn from(variant: TasksPllstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PLLSTART` writer - Start PLL and keep it running, regardless of the automatic clock requests"]
pub type TasksPllstartW<'a, REG> = crate::BitWriter<'a, REG, TasksPllstart>;
impl<'a, REG> TasksPllstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPllstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start PLL and keep it running, regardless of the automatic clock requests"]
    #[inline(always)]
    pub fn tasks_pllstart(&mut self) -> TasksPllstartW<TasksPllstartSpec> {
        TasksPllstartW::new(self, 0)
    }
}
#[doc = "Start PLL and keep it running, regardless of the automatic clock requests\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pllstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPllstartSpec;
impl crate::RegisterSpec for TasksPllstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pllstart::W`](W) writer structure"]
impl crate::Writable for TasksPllstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PLLSTART to value 0"]
impl crate::Resettable for TasksPllstartSpec {
    const RESET_VALUE: u32 = 0;
}
