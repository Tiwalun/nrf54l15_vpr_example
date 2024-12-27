#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_provision: TasksProvision,
    tasks_push: TasksPush,
    tasks_revoke: TasksRevoke,
    tasks_readmetadata: TasksReadmetadata,
    tasks_pushblock: TasksPushblock,
    _reserved5: [u8; 0xec],
    events_provisioned: EventsProvisioned,
    events_pushed: EventsPushed,
    events_revoked: EventsRevoked,
    events_error: EventsError,
    events_metadataread: EventsMetadataread,
    events_pushblocked: EventsPushblocked,
    _reserved11: [u8; 0x02e8],
    status: Status,
    _reserved12: [u8; 0xfc],
    keyslot: Keyslot,
    src: Src,
    metadata: Metadata,
}
impl RegisterBlock {
    #[doc = "0x00 - Provision key slot"]
    #[inline(always)]
    pub const fn tasks_provision(&self) -> &TasksProvision {
        &self.tasks_provision
    }
    #[doc = "0x04 - Push key slot"]
    #[inline(always)]
    pub const fn tasks_push(&self) -> &TasksPush {
        &self.tasks_push
    }
    #[doc = "0x08 - Revoke key slot"]
    #[inline(always)]
    pub const fn tasks_revoke(&self) -> &TasksRevoke {
        &self.tasks_revoke
    }
    #[doc = "0x0c - Read key slot metedata into METADATA register"]
    #[inline(always)]
    pub const fn tasks_readmetadata(&self) -> &TasksReadmetadata {
        &self.tasks_readmetadata
    }
    #[doc = "0x10 - Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset"]
    #[inline(always)]
    pub const fn tasks_pushblock(&self) -> &TasksPushblock {
        &self.tasks_pushblock
    }
    #[doc = "0x100 - Key slot successfully provisioned"]
    #[inline(always)]
    pub const fn events_provisioned(&self) -> &EventsProvisioned {
        &self.events_provisioned
    }
    #[doc = "0x104 - Key slot successfully pushed"]
    #[inline(always)]
    pub const fn events_pushed(&self) -> &EventsPushed {
        &self.events_pushed
    }
    #[doc = "0x108 - Key slot has been revoked and can no longer be used"]
    #[inline(always)]
    pub const fn events_revoked(&self) -> &EventsRevoked {
        &self.events_revoked
    }
    #[doc = "0x10c - Error during PROVISION, PUSH, or REVOKE operations"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x110 - Key slot metedata has been read into METADATA register"]
    #[inline(always)]
    pub const fn events_metadataread(&self) -> &EventsMetadataread {
        &self.events_metadataread
    }
    #[doc = "0x114 - The PUSHBLOCK operation was succesful"]
    #[inline(always)]
    pub const fn events_pushblocked(&self) -> &EventsPushblocked {
        &self.events_pushblocked
    }
    #[doc = "0x400 - KMU status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x500 - Select key slot to operate on"]
    #[inline(always)]
    pub const fn keyslot(&self) -> &Keyslot {
        &self.keyslot
    }
    #[doc = "0x504 - Source address for provisioning"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x508 - Key slot metadata as read by TASKS_READMETADATA."]
    #[inline(always)]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
#[doc = "TASKS_PROVISION (w) register accessor: Provision key slot\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_provision::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_provision`]
module"]
#[doc(alias = "TASKS_PROVISION")]
pub type TasksProvision = crate::Reg<tasks_provision::TasksProvisionSpec>;
#[doc = "Provision key slot"]
pub mod tasks_provision;
#[doc = "TASKS_PUSH (w) register accessor: Push key slot\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_push::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_push`]
module"]
#[doc(alias = "TASKS_PUSH")]
pub type TasksPush = crate::Reg<tasks_push::TasksPushSpec>;
#[doc = "Push key slot"]
pub mod tasks_push;
#[doc = "TASKS_REVOKE (w) register accessor: Revoke key slot\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_revoke::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_revoke`]
module"]
#[doc(alias = "TASKS_REVOKE")]
pub type TasksRevoke = crate::Reg<tasks_revoke::TasksRevokeSpec>;
#[doc = "Revoke key slot"]
pub mod tasks_revoke;
#[doc = "TASKS_READMETADATA (w) register accessor: Read key slot metedata into METADATA register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readmetadata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_readmetadata`]
module"]
#[doc(alias = "TASKS_READMETADATA")]
pub type TasksReadmetadata = crate::Reg<tasks_readmetadata::TasksReadmetadataSpec>;
#[doc = "Read key slot metedata into METADATA register"]
pub mod tasks_readmetadata;
#[doc = "TASKS_PUSHBLOCK (w) register accessor: Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_pushblock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_pushblock`]
module"]
#[doc(alias = "TASKS_PUSHBLOCK")]
pub type TasksPushblock = crate::Reg<tasks_pushblock::TasksPushblockSpec>;
#[doc = "Block the PUSH operation of key slot, preventing the key slot being PUSH until next reset"]
pub mod tasks_pushblock;
#[doc = "EVENTS_PROVISIONED (rw) register accessor: Key slot successfully provisioned\n\nYou can [`read`](crate::Reg::read) this register and get [`events_provisioned::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_provisioned::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_provisioned`]
module"]
#[doc(alias = "EVENTS_PROVISIONED")]
pub type EventsProvisioned = crate::Reg<events_provisioned::EventsProvisionedSpec>;
#[doc = "Key slot successfully provisioned"]
pub mod events_provisioned;
#[doc = "EVENTS_PUSHED (rw) register accessor: Key slot successfully pushed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pushed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pushed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pushed`]
module"]
#[doc(alias = "EVENTS_PUSHED")]
pub type EventsPushed = crate::Reg<events_pushed::EventsPushedSpec>;
#[doc = "Key slot successfully pushed"]
pub mod events_pushed;
#[doc = "EVENTS_REVOKED (rw) register accessor: Key slot has been revoked and can no longer be used\n\nYou can [`read`](crate::Reg::read) this register and get [`events_revoked::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_revoked::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_revoked`]
module"]
#[doc(alias = "EVENTS_REVOKED")]
pub type EventsRevoked = crate::Reg<events_revoked::EventsRevokedSpec>;
#[doc = "Key slot has been revoked and can no longer be used"]
pub mod events_revoked;
#[doc = "EVENTS_ERROR (rw) register accessor: Error during PROVISION, PUSH, or REVOKE operations\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`]
module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "Error during PROVISION, PUSH, or REVOKE operations"]
pub mod events_error;
#[doc = "EVENTS_METADATAREAD (rw) register accessor: Key slot metedata has been read into METADATA register\n\nYou can [`read`](crate::Reg::read) this register and get [`events_metadataread::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_metadataread::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_metadataread`]
module"]
#[doc(alias = "EVENTS_METADATAREAD")]
pub type EventsMetadataread = crate::Reg<events_metadataread::EventsMetadatareadSpec>;
#[doc = "Key slot metedata has been read into METADATA register"]
pub mod events_metadataread;
#[doc = "EVENTS_PUSHBLOCKED (rw) register accessor: The PUSHBLOCK operation was succesful\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pushblocked::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pushblocked::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_pushblocked`]
module"]
#[doc(alias = "EVENTS_PUSHBLOCKED")]
pub type EventsPushblocked = crate::Reg<events_pushblocked::EventsPushblockedSpec>;
#[doc = "The PUSHBLOCK operation was succesful"]
pub mod events_pushblocked;
#[doc = "STATUS (r) register accessor: KMU status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "KMU status register"]
pub mod status;
#[doc = "KEYSLOT (rw) register accessor: Select key slot to operate on\n\nYou can [`read`](crate::Reg::read) this register and get [`keyslot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyslot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyslot`]
module"]
#[doc(alias = "KEYSLOT")]
pub type Keyslot = crate::Reg<keyslot::KeyslotSpec>;
#[doc = "Select key slot to operate on"]
pub mod keyslot;
#[doc = "SRC (rw) register accessor: Source address for provisioning\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "Source address for provisioning"]
pub mod src;
#[doc = "METADATA (rw) register accessor: Key slot metadata as read by TASKS_READMETADATA.\n\nYou can [`read`](crate::Reg::read) this register and get [`metadata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`metadata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@metadata`]
module"]
#[doc(alias = "METADATA")]
pub type Metadata = crate::Reg<metadata::MetadataSpec>;
#[doc = "Key slot metadata as read by TASKS_READMETADATA."]
pub mod metadata;
