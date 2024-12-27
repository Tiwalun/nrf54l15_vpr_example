#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0300],
    info: Info,
    _reserved1: [u8; 0x50],
    er: [Er; 4],
    ir: [Ir; 4],
    deviceaddrtype: Deviceaddrtype,
    deviceaddr: [Deviceaddr; 2],
    _reserved5: [u8; 0x54],
    trimcnf: [Trimcnf; 64],
    nfc: Nfc,
    _reserved7: [u8; 0x10],
    xosc32mtrim: Xosc32mtrim,
    xosc32ktrim: Xosc32ktrim,
}
impl RegisterBlock {
    #[doc = "0x300..0x330 - Device info"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x380..0x390 - Description collection: Common encryption root key, word n"]
    #[inline(always)]
    pub const fn er(&self, n: usize) -> &Er {
        &self.er[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x380..0x390 - Description collection: Common encryption root key, word n"]
    #[inline(always)]
    pub fn er_iter(&self) -> impl Iterator<Item = &Er> {
        self.er.iter()
    }
    #[doc = "0x390..0x3a0 - Description collection: Common identity root key, word n"]
    #[inline(always)]
    pub const fn ir(&self, n: usize) -> &Ir {
        &self.ir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x390..0x3a0 - Description collection: Common identity root key, word n"]
    #[inline(always)]
    pub fn ir_iter(&self) -> impl Iterator<Item = &Ir> {
        self.ir.iter()
    }
    #[doc = "0x3a0 - Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(&self) -> &Deviceaddrtype {
        &self.deviceaddrtype
    }
    #[doc = "0x3a4..0x3ac - Description collection: Device address n"]
    #[inline(always)]
    pub const fn deviceaddr(&self, n: usize) -> &Deviceaddr {
        &self.deviceaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3a4..0x3ac - Description collection: Device address n"]
    #[inline(always)]
    pub fn deviceaddr_iter(&self) -> impl Iterator<Item = &Deviceaddr> {
        self.deviceaddr.iter()
    }
    #[doc = "0x400..0x600 - Unspecified"]
    #[inline(always)]
    pub const fn trimcnf(&self, n: usize) -> &Trimcnf {
        &self.trimcnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x600 - Unspecified"]
    #[inline(always)]
    pub fn trimcnf_iter(&self) -> impl Iterator<Item = &Trimcnf> {
        self.trimcnf.iter()
    }
    #[doc = "0x600..0x610 - Unspecified"]
    #[inline(always)]
    pub const fn nfc(&self) -> &Nfc {
        &self.nfc
    }
    #[doc = "0x620 - XOSC32M capacitor selection trim values"]
    #[inline(always)]
    pub const fn xosc32mtrim(&self) -> &Xosc32mtrim {
        &self.xosc32mtrim
    }
    #[doc = "0x624 - XOSC32K capacitor selection trim values"]
    #[inline(always)]
    pub const fn xosc32ktrim(&self) -> &Xosc32ktrim {
        &self.xosc32ktrim
    }
}
#[doc = "Device info"]
pub use self::info::Info;
#[doc = r"Cluster"]
#[doc = "Device info"]
pub mod info;
#[doc = "ER (r) register accessor: Description collection: Common encryption root key, word n\n\nYou can [`read`](crate::Reg::read) this register and get [`er::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er`]
module"]
#[doc(alias = "ER")]
pub type Er = crate::Reg<er::ErSpec>;
#[doc = "Description collection: Common encryption root key, word n"]
pub mod er;
#[doc = "IR (r) register accessor: Description collection: Common identity root key, word n\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Description collection: Common identity root key, word n"]
pub mod ir;
#[doc = "DEVICEADDRTYPE (r) register accessor: Device address type\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddrtype::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddrtype`]
module"]
#[doc(alias = "DEVICEADDRTYPE")]
pub type Deviceaddrtype = crate::Reg<deviceaddrtype::DeviceaddrtypeSpec>;
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "DEVICEADDR (r) register accessor: Description collection: Device address n\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddr`]
module"]
#[doc(alias = "DEVICEADDR")]
pub type Deviceaddr = crate::Reg<deviceaddr::DeviceaddrSpec>;
#[doc = "Description collection: Device address n"]
pub mod deviceaddr;
#[doc = "Unspecified"]
pub use self::trimcnf::Trimcnf;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = "Unspecified"]
pub use self::nfc::Nfc;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = "XOSC32MTRIM (r) register accessor: XOSC32M capacitor selection trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32mtrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32mtrim`]
module"]
#[doc(alias = "XOSC32MTRIM")]
pub type Xosc32mtrim = crate::Reg<xosc32mtrim::Xosc32mtrimSpec>;
#[doc = "XOSC32M capacitor selection trim values"]
pub mod xosc32mtrim;
#[doc = "XOSC32KTRIM (r) register accessor: XOSC32K capacitor selection trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32ktrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32ktrim`]
module"]
#[doc(alias = "XOSC32KTRIM")]
pub type Xosc32ktrim = crate::Reg<xosc32ktrim::Xosc32ktrimSpec>;
#[doc = "XOSC32K capacitor selection trim values"]
pub mod xosc32ktrim;
