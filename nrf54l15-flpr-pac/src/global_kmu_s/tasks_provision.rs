#[doc = "Register `TASKS_PROVISION` writer"]
pub type W = crate::W<TasksProvisionSpec>;
#[doc = "Provision key slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksProvision {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksProvision> for bool {
    #[inline(always)]
    fn from(variant: TasksProvision) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PROVISION` writer - Provision key slot"]
pub type TasksProvisionW<'a, REG> = crate::BitWriter<'a, REG, TasksProvision>;
impl<'a, REG> TasksProvisionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksProvision::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Provision key slot"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_provision(&mut self) -> TasksProvisionW<TasksProvisionSpec> {
        TasksProvisionW::new(self, 0)
    }
}
#[doc = "Provision key slot\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_provision::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksProvisionSpec;
impl crate::RegisterSpec for TasksProvisionSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_provision::W`](W) writer structure"]
impl crate::Writable for TasksProvisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PROVISION to value 0"]
impl crate::Resettable for TasksProvisionSpec {
    const RESET_VALUE: u32 = 0;
}
