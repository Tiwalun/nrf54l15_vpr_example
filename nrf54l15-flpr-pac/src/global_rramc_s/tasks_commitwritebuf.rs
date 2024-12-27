#[doc = "Register `TASKS_COMMITWRITEBUF` writer"]
pub type W = crate::W<TasksCommitwritebufSpec>;
#[doc = "Commits the data stored in internal write-buffer to RRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksCommitwritebuf {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksCommitwritebuf> for bool {
    #[inline(always)]
    fn from(variant: TasksCommitwritebuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_COMMITWRITEBUF` writer - Commits the data stored in internal write-buffer to RRAM"]
pub type TasksCommitwritebufW<'a, REG> = crate::BitWriter<'a, REG, TasksCommitwritebuf>;
impl<'a, REG> TasksCommitwritebufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksCommitwritebuf::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Commits the data stored in internal write-buffer to RRAM"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_commitwritebuf(&mut self) -> TasksCommitwritebufW<TasksCommitwritebufSpec> {
        TasksCommitwritebufW::new(self, 0)
    }
}
#[doc = "Commits the data stored in internal write-buffer to RRAM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_commitwritebuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksCommitwritebufSpec;
impl crate::RegisterSpec for TasksCommitwritebufSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_commitwritebuf::W`](W) writer structure"]
impl crate::Writable for TasksCommitwritebufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_COMMITWRITEBUF to value 0"]
impl crate::Resettable for TasksCommitwritebufSpec {
    const RESET_VALUE: u32 = 0;
}
