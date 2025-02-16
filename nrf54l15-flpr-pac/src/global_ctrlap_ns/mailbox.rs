#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "MAILBOX")]
pub struct Mailbox {
    rxdata: Rxdata,
    rxstatus: Rxstatus,
    _reserved2: [u8; 0x78],
    txdata: Txdata,
    txstatus: Txstatus,
}
impl Mailbox {
    #[doc = "0x00 - Data sent from the debugger to the CPU."]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x04 - Status to indicate if data sent from the debugger to the CPU has been read."]
    #[inline(always)]
    pub const fn rxstatus(&self) -> &Rxstatus {
        &self.rxstatus
    }
    #[doc = "0x80 - Data sent from the CPU to the debugger."]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x84 - Status to indicate if data sent from the CPU to the debugger has been read."]
    #[inline(always)]
    pub const fn txstatus(&self) -> &Txstatus {
        &self.txstatus
    }
}
#[doc = "RXDATA (r) register accessor: Data sent from the debugger to the CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Data sent from the debugger to the CPU."]
pub mod rxdata;
#[doc = "RXSTATUS (r) register accessor: Status to indicate if data sent from the debugger to the CPU has been read.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstatus`]
module"]
#[doc(alias = "RXSTATUS")]
pub type Rxstatus = crate::Reg<rxstatus::RxstatusSpec>;
#[doc = "Status to indicate if data sent from the debugger to the CPU has been read."]
pub mod rxstatus;
#[doc = "TXDATA (rw) register accessor: Data sent from the CPU to the debugger.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Data sent from the CPU to the debugger."]
pub mod txdata;
#[doc = "TXSTATUS (r) register accessor: Status to indicate if data sent from the CPU to the debugger has been read.\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txstatus`]
module"]
#[doc(alias = "TXSTATUS")]
pub type Txstatus = crate::Reg<txstatus::TxstatusSpec>;
#[doc = "Status to indicate if data sent from the CPU to the debugger has been read."]
pub mod txstatus;
