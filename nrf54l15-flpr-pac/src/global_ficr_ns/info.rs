#[repr(C)]
#[doc = "Device info"]
#[doc(alias = "INFO")]
pub struct Info {
    configid: Configid,
    deviceid: [Deviceid; 2],
    uuid: [Uuid; 4],
    part: Part,
    variant: Variant,
    package: Package,
    ram: Ram,
    rram: Rram,
}
impl Info {
    #[doc = "0x00 - Configuration identifier"]
    #[inline(always)]
    pub const fn configid(&self) -> &Configid {
        &self.configid
    }
    #[doc = "0x04..0x0c - Description collection: Device identifier"]
    #[inline(always)]
    pub const fn deviceid(&self, n: usize) -> &Deviceid {
        &self.deviceid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Description collection: Device identifier"]
    #[inline(always)]
    pub fn deviceid_iter(&self) -> impl Iterator<Item = &Deviceid> {
        self.deviceid.iter()
    }
    #[doc = "0x0c..0x1c - Description collection: 128-bit Universally Unique IDentifier (UUID)."]
    #[inline(always)]
    pub const fn uuid(&self, n: usize) -> &Uuid {
        &self.uuid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - Description collection: 128-bit Universally Unique IDentifier (UUID)."]
    #[inline(always)]
    pub fn uuid_iter(&self) -> impl Iterator<Item = &Uuid> {
        self.uuid.iter()
    }
    #[doc = "0x1c - Part code"]
    #[inline(always)]
    pub const fn part(&self) -> &Part {
        &self.part
    }
    #[doc = "0x20 - Part Variant, Hardware version and Production configuration"]
    #[inline(always)]
    pub const fn variant(&self) -> &Variant {
        &self.variant
    }
    #[doc = "0x24 - Package option"]
    #[inline(always)]
    pub const fn package(&self) -> &Package {
        &self.package
    }
    #[doc = "0x28 - RAM variant"]
    #[inline(always)]
    pub const fn ram(&self) -> &Ram {
        &self.ram
    }
    #[doc = "0x2c - RRAM variant"]
    #[inline(always)]
    pub const fn rram(&self) -> &Rram {
        &self.rram
    }
}
#[doc = "CONFIGID (r) register accessor: Configuration identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`configid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configid`]
module"]
#[doc(alias = "CONFIGID")]
pub type Configid = crate::Reg<configid::ConfigidSpec>;
#[doc = "Configuration identifier"]
pub mod configid;
#[doc = "DEVICEID (r) register accessor: Description collection: Device identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`]
module"]
#[doc(alias = "DEVICEID")]
pub type Deviceid = crate::Reg<deviceid::DeviceidSpec>;
#[doc = "Description collection: Device identifier"]
pub mod deviceid;
#[doc = "UUID (rw) register accessor: Description collection: 128-bit Universally Unique IDentifier (UUID).\n\nYou can [`read`](crate::Reg::read) this register and get [`uuid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uuid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uuid`]
module"]
#[doc(alias = "UUID")]
pub type Uuid = crate::Reg<uuid::UuidSpec>;
#[doc = "Description collection: 128-bit Universally Unique IDentifier (UUID)."]
pub mod uuid;
#[doc = "PART (r) register accessor: Part code\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part`]
module"]
#[doc(alias = "PART")]
pub type Part = crate::Reg<part::PartSpec>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT (r) register accessor: Part Variant, Hardware version and Production configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`variant::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@variant`]
module"]
#[doc(alias = "VARIANT")]
pub type Variant = crate::Reg<variant::VariantSpec>;
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "PACKAGE (r) register accessor: Package option\n\nYou can [`read`](crate::Reg::read) this register and get [`package::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@package`]
module"]
#[doc(alias = "PACKAGE")]
pub type Package = crate::Reg<package::PackageSpec>;
#[doc = "Package option"]
pub mod package;
#[doc = "RAM (r) register accessor: RAM variant\n\nYou can [`read`](crate::Reg::read) this register and get [`ram::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram`]
module"]
#[doc(alias = "RAM")]
pub type Ram = crate::Reg<ram::RamSpec>;
#[doc = "RAM variant"]
pub mod ram;
#[doc = "RRAM (r) register accessor: RRAM variant\n\nYou can [`read`](crate::Reg::read) this register and get [`rram::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rram`]
module"]
#[doc(alias = "RRAM")]
pub type Rram = crate::Reg<rram::RramSpec>;
#[doc = "RRAM variant"]
pub mod rram;
