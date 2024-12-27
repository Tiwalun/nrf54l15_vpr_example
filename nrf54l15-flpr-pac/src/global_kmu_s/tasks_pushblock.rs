#[doc = "Register `TASKS_PUSHBLOCK` writer"]
pub type W = crate::W<TasksPushblockSpec>;
#[doc = "Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksPushblock {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksPushblock> for bool {
    #[inline(always)]
    fn from(variant: TasksPushblock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_PUSHBLOCK` writer - Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset"]
pub type TasksPushblockW<'a, REG> = crate::BitWriter<'a, REG, TasksPushblock>;
impl<'a, REG> TasksPushblockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksPushblock::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_pushblock(&mut self) -> TasksPushblockW<TasksPushblockSpec> {
        TasksPushblockW::new(self, 0)
    }
}
#[doc = "Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pushblock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksPushblockSpec;
impl crate::RegisterSpec for TasksPushblockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_pushblock::W`](W) writer structure"]
impl crate::Writable for TasksPushblockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_PUSHBLOCK to value 0"]
impl crate::Resettable for TasksPushblockSpec {
    const RESET_VALUE: u32 = 0;
}
