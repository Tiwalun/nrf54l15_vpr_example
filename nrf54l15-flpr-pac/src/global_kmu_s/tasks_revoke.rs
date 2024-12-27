#[doc = "Register `TASKS_REVOKE` writer"]
pub type W = crate::W<TasksRevokeSpec>;
#[doc = "Revoke key slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksRevoke {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksRevoke> for bool {
    #[inline(always)]
    fn from(variant: TasksRevoke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_REVOKE` writer - Revoke key slot"]
pub type TasksRevokeW<'a, REG> = crate::BitWriter<'a, REG, TasksRevoke>;
impl<'a, REG> TasksRevokeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksRevoke::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Revoke key slot"]
    #[inline(always)]
    pub fn tasks_revoke(&mut self) -> TasksRevokeW<TasksRevokeSpec> {
        TasksRevokeW::new(self, 0)
    }
}
#[doc = "Revoke key slot\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_revoke::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksRevokeSpec;
impl crate::RegisterSpec for TasksRevokeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_revoke::W`](W) writer structure"]
impl crate::Writable for TasksRevokeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_REVOKE to value 0"]
impl crate::Resettable for TasksRevokeSpec {
    const RESET_VALUE: u32 = 0;
}
