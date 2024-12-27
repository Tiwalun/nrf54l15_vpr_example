#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "BUFSTATUS")]
pub struct Bufstatus {
    _reserved0: [u8; 0x08],
    writebufempty: Writebufempty,
}
impl Bufstatus {
    #[doc = "0x08 - Internal write-buffer is empty"]
    #[inline(always)]
    pub const fn writebufempty(&self) -> &Writebufempty {
        &self.writebufempty
    }
}
#[doc = "WRITEBUFEMPTY (r) register accessor: Internal write-buffer is empty\n\nYou can [`read`](crate::Reg::read) this register and get [`writebufempty::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writebufempty`]
module"]
#[doc(alias = "WRITEBUFEMPTY")]
pub type Writebufempty = crate::Reg<writebufempty::WritebufemptySpec>;
#[doc = "Internal write-buffer is empty"]
pub mod writebufempty;
