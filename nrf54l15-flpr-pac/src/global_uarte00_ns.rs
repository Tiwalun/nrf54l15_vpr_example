#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    tasks_flushrx: TasksFlushrx,
    _reserved1: [u8; 0x08],
    tasks_dma: TasksDma,
    _reserved2: [u8; 0x44],
    subscribe_flushrx: SubscribeFlushrx,
    _reserved3: [u8; 0x08],
    subscribe_dma: SubscribeDma,
    _reserved4: [u8; 0x28],
    events_cts: EventsCts,
    events_ncts: EventsNcts,
    _reserved6: [u8; 0x04],
    events_txdrdy: EventsTxdrdy,
    events_rxdrdy: EventsRxdrdy,
    events_error: EventsError,
    _reserved9: [u8; 0x0c],
    events_rxto: EventsRxto,
    _reserved10: [u8; 0x08],
    events_txstopped: EventsTxstopped,
    _reserved11: [u8; 0x18],
    events_dma: EventsDma,
    events_frametimeout: EventsFrametimeout,
    _reserved13: [u8; 0x08],
    publish_cts: PublishCts,
    publish_ncts: PublishNcts,
    _reserved15: [u8; 0x04],
    publish_txdrdy: PublishTxdrdy,
    publish_rxdrdy: PublishRxdrdy,
    publish_error: PublishError,
    _reserved18: [u8; 0x0c],
    publish_rxto: PublishRxto,
    _reserved19: [u8; 0x08],
    publish_txstopped: PublishTxstopped,
    _reserved20: [u8; 0x18],
    publish_dma: PublishDma,
    publish_frametimeout: PublishFrametimeout,
    _reserved22: [u8; 0x08],
    shorts: Shorts,
    _reserved23: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved26: [u8; 0x0174],
    errorsrc: Errorsrc,
    _reserved27: [u8; 0x7c],
    enable: Enable,
    _reserved28: [u8; 0x20],
    baudrate: Baudrate,
    _reserved29: [u8; 0x44],
    config: Config,
    _reserved30: [u8; 0x04],
    address: Address,
    frametimeout: Frametimeout,
    _reserved32: [u8; 0x88],
    psel: Psel,
    _reserved33: [u8; 0xec],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x1c - Flush RX FIFO into RX buffer"]
    #[inline(always)]
    pub const fn tasks_flushrx(&self) -> &TasksFlushrx {
        &self.tasks_flushrx
    }
    #[doc = "0x28..0x58 - Peripheral tasks."]
    #[inline(always)]
    pub const fn tasks_dma(&self) -> &TasksDma {
        &self.tasks_dma
    }
    #[doc = "0x9c - Subscribe configuration for task FLUSHRX"]
    #[inline(always)]
    pub const fn subscribe_flushrx(&self) -> &SubscribeFlushrx {
        &self.subscribe_flushrx
    }
    #[doc = "0xa8..0xd8 - Subscribe configuration for tasks"]
    #[inline(always)]
    pub const fn subscribe_dma(&self) -> &SubscribeDma {
        &self.subscribe_dma
    }
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    #[inline(always)]
    pub const fn events_cts(&self) -> &EventsCts {
        &self.events_cts
    }
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub const fn events_ncts(&self) -> &EventsNcts {
        &self.events_ncts
    }
    #[doc = "0x10c - Data sent from TXD"]
    #[inline(always)]
    pub const fn events_txdrdy(&self) -> &EventsTxdrdy {
        &self.events_txdrdy
    }
    #[doc = "0x110 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    #[inline(always)]
    pub const fn events_rxdrdy(&self) -> &EventsRxdrdy {
        &self.events_rxdrdy
    }
    #[doc = "0x114 - Error detected"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x124 - Receiver timeout"]
    #[inline(always)]
    pub const fn events_rxto(&self) -> &EventsRxto {
        &self.events_rxto
    }
    #[doc = "0x130 - Transmitter stopped"]
    #[inline(always)]
    pub const fn events_txstopped(&self) -> &EventsTxstopped {
        &self.events_txstopped
    }
    #[doc = "0x14c..0x174 - Peripheral events."]
    #[inline(always)]
    pub const fn events_dma(&self) -> &EventsDma {
        &self.events_dma
    }
    #[doc = "0x174 - Timed out due to bus being idle while receiving data."]
    #[inline(always)]
    pub const fn events_frametimeout(&self) -> &EventsFrametimeout {
        &self.events_frametimeout
    }
    #[doc = "0x180 - Publish configuration for event CTS"]
    #[inline(always)]
    pub const fn publish_cts(&self) -> &PublishCts {
        &self.publish_cts
    }
    #[doc = "0x184 - Publish configuration for event NCTS"]
    #[inline(always)]
    pub const fn publish_ncts(&self) -> &PublishNcts {
        &self.publish_ncts
    }
    #[doc = "0x18c - Publish configuration for event TXDRDY"]
    #[inline(always)]
    pub const fn publish_txdrdy(&self) -> &PublishTxdrdy {
        &self.publish_txdrdy
    }
    #[doc = "0x190 - Publish configuration for event RXDRDY"]
    #[inline(always)]
    pub const fn publish_rxdrdy(&self) -> &PublishRxdrdy {
        &self.publish_rxdrdy
    }
    #[doc = "0x194 - Publish configuration for event ERROR"]
    #[inline(always)]
    pub const fn publish_error(&self) -> &PublishError {
        &self.publish_error
    }
    #[doc = "0x1a4 - Publish configuration for event RXTO"]
    #[inline(always)]
    pub const fn publish_rxto(&self) -> &PublishRxto {
        &self.publish_rxto
    }
    #[doc = "0x1b0 - Publish configuration for event TXSTOPPED"]
    #[inline(always)]
    pub const fn publish_txstopped(&self) -> &PublishTxstopped {
        &self.publish_txstopped
    }
    #[doc = "0x1cc..0x1f4 - Publish configuration for events"]
    #[inline(always)]
    pub const fn publish_dma(&self) -> &PublishDma {
        &self.publish_dma
    }
    #[doc = "0x1f4 - Publish configuration for event FRAMETIMEOUT"]
    #[inline(always)]
    pub const fn publish_frametimeout(&self) -> &PublishFrametimeout {
        &self.publish_frametimeout
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x480 - Error source"]
    #[inline(always)]
    pub const fn errorsrc(&self) -> &Errorsrc {
        &self.errorsrc
    }
    #[doc = "0x500 - Enable UART"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x524 - Baud rate. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn baudrate(&self) -> &Baudrate {
        &self.baudrate
    }
    #[doc = "0x56c - Configuration of parity, hardware flow control, framesize, and packet timeout."]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x574 - Set the address of the UARTE for RX when used in 9 bit data frame mode."]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x578 - Set the number of UARTE bits to count before triggering packet timeout."]
    #[inline(always)]
    pub const fn frametimeout(&self) -> &Frametimeout {
        &self.frametimeout
    }
    #[doc = "0x604..0x614 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x700..0x75c - Unspecified"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "TASKS_FLUSHRX (w) register accessor: Flush RX FIFO into RX buffer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_flushrx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_flushrx`]
module"]
#[doc(alias = "TASKS_FLUSHRX")]
pub type TasksFlushrx = crate::Reg<tasks_flushrx::TasksFlushrxSpec>;
#[doc = "Flush RX FIFO into RX buffer"]
pub mod tasks_flushrx;
#[doc = "Peripheral tasks."]
pub use self::tasks_dma::TasksDma;
#[doc = r"Cluster"]
#[doc = "Peripheral tasks."]
pub mod tasks_dma;
#[doc = "SUBSCRIBE_FLUSHRX (rw) register accessor: Subscribe configuration for task FLUSHRX\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_flushrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_flushrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_flushrx`]
module"]
#[doc(alias = "SUBSCRIBE_FLUSHRX")]
pub type SubscribeFlushrx = crate::Reg<subscribe_flushrx::SubscribeFlushrxSpec>;
#[doc = "Subscribe configuration for task FLUSHRX"]
pub mod subscribe_flushrx;
#[doc = "Subscribe configuration for tasks"]
pub use self::subscribe_dma::SubscribeDma;
#[doc = r"Cluster"]
#[doc = "Subscribe configuration for tasks"]
pub mod subscribe_dma;
#[doc = "EVENTS_CTS (rw) register accessor: CTS is activated (set low). Clear To Send.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_cts`]
module"]
#[doc(alias = "EVENTS_CTS")]
pub type EventsCts = crate::Reg<events_cts::EventsCtsSpec>;
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "EVENTS_NCTS (rw) register accessor: CTS is deactivated (set high). Not Clear To Send.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ncts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ncts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ncts`]
module"]
#[doc(alias = "EVENTS_NCTS")]
pub type EventsNcts = crate::Reg<events_ncts::EventsNctsSpec>;
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "EVENTS_TXDRDY (rw) register accessor: Data sent from TXD\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txdrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txdrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txdrdy`]
module"]
#[doc(alias = "EVENTS_TXDRDY")]
pub type EventsTxdrdy = crate::Reg<events_txdrdy::EventsTxdrdySpec>;
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "EVENTS_RXDRDY (rw) register accessor: Data received in RXD (but potentially not yet transferred to Data RAM)\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxdrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxdrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxdrdy`]
module"]
#[doc(alias = "EVENTS_RXDRDY")]
pub type EventsRxdrdy = crate::Reg<events_rxdrdy::EventsRxdrdySpec>;
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub mod events_rxdrdy;
#[doc = "EVENTS_ERROR (rw) register accessor: Error detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`]
module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "Error detected"]
pub mod events_error;
#[doc = "EVENTS_RXTO (rw) register accessor: Receiver timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxto`]
module"]
#[doc(alias = "EVENTS_RXTO")]
pub type EventsRxto = crate::Reg<events_rxto::EventsRxtoSpec>;
#[doc = "Receiver timeout"]
pub mod events_rxto;
#[doc = "EVENTS_TXSTOPPED (rw) register accessor: Transmitter stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txstopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txstopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txstopped`]
module"]
#[doc(alias = "EVENTS_TXSTOPPED")]
pub type EventsTxstopped = crate::Reg<events_txstopped::EventsTxstoppedSpec>;
#[doc = "Transmitter stopped"]
pub mod events_txstopped;
#[doc = "Peripheral events."]
pub use self::events_dma::EventsDma;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_dma;
#[doc = "EVENTS_FRAMETIMEOUT (rw) register accessor: Timed out due to bus being idle while receiving data.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_frametimeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_frametimeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_frametimeout`]
module"]
#[doc(alias = "EVENTS_FRAMETIMEOUT")]
pub type EventsFrametimeout = crate::Reg<events_frametimeout::EventsFrametimeoutSpec>;
#[doc = "Timed out due to bus being idle while receiving data."]
pub mod events_frametimeout;
#[doc = "PUBLISH_CTS (rw) register accessor: Publish configuration for event CTS\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_cts`]
module"]
#[doc(alias = "PUBLISH_CTS")]
pub type PublishCts = crate::Reg<publish_cts::PublishCtsSpec>;
#[doc = "Publish configuration for event CTS"]
pub mod publish_cts;
#[doc = "PUBLISH_NCTS (rw) register accessor: Publish configuration for event NCTS\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ncts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ncts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ncts`]
module"]
#[doc(alias = "PUBLISH_NCTS")]
pub type PublishNcts = crate::Reg<publish_ncts::PublishNctsSpec>;
#[doc = "Publish configuration for event NCTS"]
pub mod publish_ncts;
#[doc = "PUBLISH_TXDRDY (rw) register accessor: Publish configuration for event TXDRDY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_txdrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_txdrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txdrdy`]
module"]
#[doc(alias = "PUBLISH_TXDRDY")]
pub type PublishTxdrdy = crate::Reg<publish_txdrdy::PublishTxdrdySpec>;
#[doc = "Publish configuration for event TXDRDY"]
pub mod publish_txdrdy;
#[doc = "PUBLISH_RXDRDY (rw) register accessor: Publish configuration for event RXDRDY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rxdrdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rxdrdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxdrdy`]
module"]
#[doc(alias = "PUBLISH_RXDRDY")]
pub type PublishRxdrdy = crate::Reg<publish_rxdrdy::PublishRxdrdySpec>;
#[doc = "Publish configuration for event RXDRDY"]
pub mod publish_rxdrdy;
#[doc = "PUBLISH_ERROR (rw) register accessor: Publish configuration for event ERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_error::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_error::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_error`]
module"]
#[doc(alias = "PUBLISH_ERROR")]
pub type PublishError = crate::Reg<publish_error::PublishErrorSpec>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "PUBLISH_RXTO (rw) register accessor: Publish configuration for event RXTO\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rxto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rxto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxto`]
module"]
#[doc(alias = "PUBLISH_RXTO")]
pub type PublishRxto = crate::Reg<publish_rxto::PublishRxtoSpec>;
#[doc = "Publish configuration for event RXTO"]
pub mod publish_rxto;
#[doc = "PUBLISH_TXSTOPPED (rw) register accessor: Publish configuration for event TXSTOPPED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_txstopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_txstopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txstopped`]
module"]
#[doc(alias = "PUBLISH_TXSTOPPED")]
pub type PublishTxstopped = crate::Reg<publish_txstopped::PublishTxstoppedSpec>;
#[doc = "Publish configuration for event TXSTOPPED"]
pub mod publish_txstopped;
#[doc = "Publish configuration for events"]
pub use self::publish_dma::PublishDma;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod publish_dma;
#[doc = "PUBLISH_FRAMETIMEOUT (rw) register accessor: Publish configuration for event FRAMETIMEOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_frametimeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_frametimeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_frametimeout`]
module"]
#[doc(alias = "PUBLISH_FRAMETIMEOUT")]
pub type PublishFrametimeout = crate::Reg<publish_frametimeout::PublishFrametimeoutSpec>;
#[doc = "Publish configuration for event FRAMETIMEOUT"]
pub mod publish_frametimeout;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ERRORSRC (rw) register accessor: Error source\n\nYou can [`read`](crate::Reg::read) this register and get [`errorsrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errorsrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorsrc`]
module"]
#[doc(alias = "ERRORSRC")]
pub type Errorsrc = crate::Reg<errorsrc::ErrorsrcSpec>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: Enable UART\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable UART"]
pub mod enable;
#[doc = "BAUDRATE (rw) register accessor: Baud rate. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::Reg::read) this register and get [`baudrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudrate`]
module"]
#[doc(alias = "BAUDRATE")]
pub type Baudrate = crate::Reg<baudrate::BaudrateSpec>;
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub mod baudrate;
#[doc = "CONFIG (rw) register accessor: Configuration of parity, hardware flow control, framesize, and packet timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration of parity, hardware flow control, framesize, and packet timeout."]
pub mod config;
#[doc = "ADDRESS (rw) register accessor: Set the address of the UARTE for RX when used in 9 bit data frame mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Set the address of the UARTE for RX when used in 9 bit data frame mode."]
pub mod address;
#[doc = "FRAMETIMEOUT (rw) register accessor: Set the number of UARTE bits to count before triggering packet timeout.\n\nYou can [`read`](crate::Reg::read) this register and get [`frametimeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frametimeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frametimeout`]
module"]
#[doc(alias = "FRAMETIMEOUT")]
pub type Frametimeout = crate::Reg<frametimeout::FrametimeoutSpec>;
#[doc = "Set the number of UARTE bits to count before triggering packet timeout."]
pub mod frametimeout;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Unspecified"]
pub use self::dma::Dma;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod dma;
