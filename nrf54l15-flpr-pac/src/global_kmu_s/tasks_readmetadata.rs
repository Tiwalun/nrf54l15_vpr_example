#[doc = "Register `TASKS_READMETADATA` writer"]
pub type W = crate::W<TasksReadmetadataSpec>;
#[doc = "Read key slot metadata into METADATA register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksReadmetadata {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksReadmetadata> for bool {
    #[inline(always)]
    fn from(variant: TasksReadmetadata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_READMETADATA` writer - Read key slot metadata into METADATA register"]
pub type TasksReadmetadataW<'a, REG> = crate::BitWriter<'a, REG, TasksReadmetadata>;
impl<'a, REG> TasksReadmetadataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksReadmetadata::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Read key slot metadata into METADATA register"]
    #[inline(always)]
    pub fn tasks_readmetadata(&mut self) -> TasksReadmetadataW<TasksReadmetadataSpec> {
        TasksReadmetadataW::new(self, 0)
    }
}
#[doc = "Read key slot metadata into METADATA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readmetadata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksReadmetadataSpec;
impl crate::RegisterSpec for TasksReadmetadataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_readmetadata::W`](W) writer structure"]
impl crate::Writable for TasksReadmetadataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_READMETADATA to value 0"]
impl crate::Resettable for TasksReadmetadataSpec {
    const RESET_VALUE: u32 = 0;
}
