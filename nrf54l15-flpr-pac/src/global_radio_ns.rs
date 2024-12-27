#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_txen: TasksTxen,
    tasks_rxen: TasksRxen,
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_disable: TasksDisable,
    tasks_rssistart: TasksRssistart,
    tasks_bcstart: TasksBcstart,
    tasks_bcstop: TasksBcstop,
    tasks_edstart: TasksEdstart,
    tasks_edstop: TasksEdstop,
    tasks_ccastart: TasksCcastart,
    tasks_ccastop: TasksCcastop,
    _reserved12: [u8; 0x74],
    tasks_softreset: TasksSoftreset,
    _reserved13: [u8; 0x58],
    subscribe_txen: SubscribeTxen,
    subscribe_rxen: SubscribeRxen,
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    subscribe_disable: SubscribeDisable,
    subscribe_rssistart: SubscribeRssistart,
    subscribe_bcstart: SubscribeBcstart,
    subscribe_bcstop: SubscribeBcstop,
    subscribe_edstart: SubscribeEdstart,
    subscribe_edstop: SubscribeEdstop,
    subscribe_ccastart: SubscribeCcastart,
    subscribe_ccastop: SubscribeCcastop,
    _reserved25: [u8; 0x74],
    subscribe_softreset: SubscribeSoftreset,
    _reserved26: [u8; 0x58],
    events_ready: EventsReady,
    events_txready: EventsTxready,
    events_rxready: EventsRxready,
    events_address: EventsAddress,
    events_framestart: EventsFramestart,
    events_payload: EventsPayload,
    events_end: EventsEnd,
    events_phyend: EventsPhyend,
    events_disabled: EventsDisabled,
    events_devmatch: EventsDevmatch,
    events_devmiss: EventsDevmiss,
    events_crcok: EventsCrcok,
    events_crcerror: EventsCrcerror,
    _reserved39: [u8; 0x04],
    events_bcmatch: EventsBcmatch,
    events_edend: EventsEdend,
    events_edstopped: EventsEdstopped,
    events_ccaidle: EventsCcaidle,
    events_ccabusy: EventsCcabusy,
    events_ccastopped: EventsCcastopped,
    events_rateboost: EventsRateboost,
    events_mhrmatch: EventsMhrmatch,
    events_sync: EventsSync,
    events_ctepresent: EventsCtepresent,
    _reserved49: [u8; 0xa0],
    publish_ready: PublishReady,
    publish_txready: PublishTxready,
    publish_rxready: PublishRxready,
    publish_address: PublishAddress,
    publish_framestart: PublishFramestart,
    publish_payload: PublishPayload,
    publish_end: PublishEnd,
    publish_phyend: PublishPhyend,
    publish_disabled: PublishDisabled,
    publish_devmatch: PublishDevmatch,
    publish_devmiss: PublishDevmiss,
    publish_crcok: PublishCrcok,
    publish_crcerror: PublishCrcerror,
    _reserved62: [u8; 0x04],
    publish_bcmatch: PublishBcmatch,
    publish_edend: PublishEdend,
    publish_edstopped: PublishEdstopped,
    publish_ccaidle: PublishCcaidle,
    publish_ccabusy: PublishCcabusy,
    publish_ccastopped: PublishCcastopped,
    publish_rateboost: PublishRateboost,
    publish_mhrmatch: PublishMhrmatch,
    publish_sync: PublishSync,
    publish_ctepresent: PublishCtepresent,
    _reserved72: [u8; 0xa0],
    shorts: Shorts,
    _reserved73: [u8; 0x84],
    intenset00: Intenset00,
    _reserved74: [u8; 0x04],
    intenclr00: Intenclr00,
    _reserved75: [u8; 0x14],
    intenset10: Intenset10,
    _reserved76: [u8; 0x04],
    intenclr10: Intenclr10,
    _reserved77: [u8; 0x4c],
    mode: Mode,
    _reserved78: [u8; 0x1c],
    state: State,
    _reserved79: [u8; 0x0c],
    edctrl: Edctrl,
    edsample: Edsample,
    ccactrl: Ccactrl,
    _reserved82: [u8; 0x04],
    datawhite: Datawhite,
    _reserved83: [u8; 0x01c0],
    timing: Timing,
    frequency: Frequency,
    _reserved85: [u8; 0x04],
    txpower: Txpower,
    tifs: Tifs,
    rssisample: Rssisample,
    _reserved88: [u8; 0x01ec],
    feconfig: Feconfig,
    _reserved89: [u8; 0x03f4],
    dfemode: Dfemode,
    dfestatus: Dfestatus,
    _reserved91: [u8; 0x08],
    dfectrl1: Dfectrl1,
    dfectrl2: Dfectrl2,
    _reserved93: [u8; 0x10],
    switchpattern: Switchpattern,
    clearpattern: Clearpattern,
    psel: Psel,
    dfepacket: Dfepacket,
    _reserved97: [u8; 0xac],
    crcstatus: Crcstatus,
    rxmatch: Rxmatch,
    rxcrc: Rxcrc,
    dai: Dai,
    pdustat: Pdustat,
    pcnf0: Pcnf0,
    _reserved103: [u8; 0x04],
    pcnf1: Pcnf1,
    base0: Base0,
    base1: Base1,
    prefix0: Prefix0,
    prefix1: Prefix1,
    txaddress: Txaddress,
    rxaddresses: Rxaddresses,
    crccnf: Crccnf,
    crcpoly: Crcpoly,
    crcinit: Crcinit,
    dab: [Dab; 8],
    dap: [Dap; 8],
    dacnf: Dacnf,
    bcc: Bcc,
    _reserved117: [u8; 0x0c],
    ctestatus: Ctestatus,
    _reserved118: [u8; 0x0c],
    mhrmatchconf: Mhrmatchconf,
    mhrmatchmask: Mhrmatchmask,
    sfd: Sfd,
    cteinlineconf: Cteinlineconf,
    _reserved122: [u8; 0x0c],
    packetptr: Packetptr,
    _reserved123: [u8; 0x012c],
    cstones: Cstones,
    rtt: Rtt,
}
impl RegisterBlock {
    #[doc = "0x00 - Enable RADIO in TX mode"]
    #[inline(always)]
    pub const fn tasks_txen(&self) -> &TasksTxen {
        &self.tasks_txen
    }
    #[doc = "0x04 - Enable RADIO in RX mode"]
    #[inline(always)]
    pub const fn tasks_rxen(&self) -> &TasksRxen {
        &self.tasks_rxen
    }
    #[doc = "0x08 - Start RADIO"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x0c - Stop RADIO"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x10 - Disable RADIO"]
    #[inline(always)]
    pub const fn tasks_disable(&self) -> &TasksDisable {
        &self.tasks_disable
    }
    #[doc = "0x14 - Start the RSSI and take one single sample of the receive signal strength"]
    #[inline(always)]
    pub const fn tasks_rssistart(&self) -> &TasksRssistart {
        &self.tasks_rssistart
    }
    #[doc = "0x18 - Start the bit counter"]
    #[inline(always)]
    pub const fn tasks_bcstart(&self) -> &TasksBcstart {
        &self.tasks_bcstart
    }
    #[doc = "0x1c - Stop the bit counter"]
    #[inline(always)]
    pub const fn tasks_bcstop(&self) -> &TasksBcstop {
        &self.tasks_bcstop
    }
    #[doc = "0x20 - Start the energy detect measurement used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub const fn tasks_edstart(&self) -> &TasksEdstart {
        &self.tasks_edstart
    }
    #[doc = "0x24 - Stop the energy detect measurement"]
    #[inline(always)]
    pub const fn tasks_edstop(&self) -> &TasksEdstop {
        &self.tasks_edstop
    }
    #[doc = "0x28 - Start the clear channel assessment used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub const fn tasks_ccastart(&self) -> &TasksCcastart {
        &self.tasks_ccastart
    }
    #[doc = "0x2c - Stop the clear channel assessment"]
    #[inline(always)]
    pub const fn tasks_ccastop(&self) -> &TasksCcastop {
        &self.tasks_ccastop
    }
    #[doc = "0xa4 - Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state."]
    #[inline(always)]
    pub const fn tasks_softreset(&self) -> &TasksSoftreset {
        &self.tasks_softreset
    }
    #[doc = "0x100 - Subscribe configuration for task TXEN"]
    #[inline(always)]
    pub const fn subscribe_txen(&self) -> &SubscribeTxen {
        &self.subscribe_txen
    }
    #[doc = "0x104 - Subscribe configuration for task RXEN"]
    #[inline(always)]
    pub const fn subscribe_rxen(&self) -> &SubscribeRxen {
        &self.subscribe_rxen
    }
    #[doc = "0x108 - Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(&self) -> &SubscribeStart {
        &self.subscribe_start
    }
    #[doc = "0x10c - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(&self) -> &SubscribeStop {
        &self.subscribe_stop
    }
    #[doc = "0x110 - Subscribe configuration for task DISABLE"]
    #[inline(always)]
    pub const fn subscribe_disable(&self) -> &SubscribeDisable {
        &self.subscribe_disable
    }
    #[doc = "0x114 - Subscribe configuration for task RSSISTART"]
    #[inline(always)]
    pub const fn subscribe_rssistart(&self) -> &SubscribeRssistart {
        &self.subscribe_rssistart
    }
    #[doc = "0x118 - Subscribe configuration for task BCSTART"]
    #[inline(always)]
    pub const fn subscribe_bcstart(&self) -> &SubscribeBcstart {
        &self.subscribe_bcstart
    }
    #[doc = "0x11c - Subscribe configuration for task BCSTOP"]
    #[inline(always)]
    pub const fn subscribe_bcstop(&self) -> &SubscribeBcstop {
        &self.subscribe_bcstop
    }
    #[doc = "0x120 - Subscribe configuration for task EDSTART"]
    #[inline(always)]
    pub const fn subscribe_edstart(&self) -> &SubscribeEdstart {
        &self.subscribe_edstart
    }
    #[doc = "0x124 - Subscribe configuration for task EDSTOP"]
    #[inline(always)]
    pub const fn subscribe_edstop(&self) -> &SubscribeEdstop {
        &self.subscribe_edstop
    }
    #[doc = "0x128 - Subscribe configuration for task CCASTART"]
    #[inline(always)]
    pub const fn subscribe_ccastart(&self) -> &SubscribeCcastart {
        &self.subscribe_ccastart
    }
    #[doc = "0x12c - Subscribe configuration for task CCASTOP"]
    #[inline(always)]
    pub const fn subscribe_ccastop(&self) -> &SubscribeCcastop {
        &self.subscribe_ccastop
    }
    #[doc = "0x1a4 - Subscribe configuration for task SOFTRESET"]
    #[inline(always)]
    pub const fn subscribe_softreset(&self) -> &SubscribeSoftreset {
        &self.subscribe_softreset
    }
    #[doc = "0x200 - RADIO has ramped up and is ready to be started"]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x204 - RADIO has ramped up and is ready to be started TX path"]
    #[inline(always)]
    pub const fn events_txready(&self) -> &EventsTxready {
        &self.events_txready
    }
    #[doc = "0x208 - RADIO has ramped up and is ready to be started RX path"]
    #[inline(always)]
    pub const fn events_rxready(&self) -> &EventsRxready {
        &self.events_rxready
    }
    #[doc = "0x20c - Address sent or received"]
    #[inline(always)]
    pub const fn events_address(&self) -> &EventsAddress {
        &self.events_address
    }
    #[doc = "0x210 - IEEE 802.15.4 length field received"]
    #[inline(always)]
    pub const fn events_framestart(&self) -> &EventsFramestart {
        &self.events_framestart
    }
    #[doc = "0x214 - Packet payload sent or received"]
    #[inline(always)]
    pub const fn events_payload(&self) -> &EventsPayload {
        &self.events_payload
    }
    #[doc = "0x218 - Memory access for packet data has been completed"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x21c - The last bit is sent on air or last bit is received"]
    #[inline(always)]
    pub const fn events_phyend(&self) -> &EventsPhyend {
        &self.events_phyend
    }
    #[doc = "0x220 - RADIO has been disabled"]
    #[inline(always)]
    pub const fn events_disabled(&self) -> &EventsDisabled {
        &self.events_disabled
    }
    #[doc = "0x224 - A device address match occurred on the last received packet"]
    #[inline(always)]
    pub const fn events_devmatch(&self) -> &EventsDevmatch {
        &self.events_devmatch
    }
    #[doc = "0x228 - No device address match occurred on the last received packet"]
    #[inline(always)]
    pub const fn events_devmiss(&self) -> &EventsDevmiss {
        &self.events_devmiss
    }
    #[doc = "0x22c - Packet received with CRC ok"]
    #[inline(always)]
    pub const fn events_crcok(&self) -> &EventsCrcok {
        &self.events_crcok
    }
    #[doc = "0x230 - Packet received with CRC error"]
    #[inline(always)]
    pub const fn events_crcerror(&self) -> &EventsCrcerror {
        &self.events_crcerror
    }
    #[doc = "0x238 - Bit counter reached bit count value"]
    #[inline(always)]
    pub const fn events_bcmatch(&self) -> &EventsBcmatch {
        &self.events_bcmatch
    }
    #[doc = "0x23c - Sampling of energy detection complete (a new ED sample is ready for readout from the RADIO.EDSAMPLE register)"]
    #[inline(always)]
    pub const fn events_edend(&self) -> &EventsEdend {
        &self.events_edend
    }
    #[doc = "0x240 - The sampling of energy detection has stopped"]
    #[inline(always)]
    pub const fn events_edstopped(&self) -> &EventsEdstopped {
        &self.events_edstopped
    }
    #[doc = "0x244 - Wireless medium in idle - clear to send"]
    #[inline(always)]
    pub const fn events_ccaidle(&self) -> &EventsCcaidle {
        &self.events_ccaidle
    }
    #[doc = "0x248 - Wireless medium busy - do not send"]
    #[inline(always)]
    pub const fn events_ccabusy(&self) -> &EventsCcabusy {
        &self.events_ccabusy
    }
    #[doc = "0x24c - The CCA has stopped"]
    #[inline(always)]
    pub const fn events_ccastopped(&self) -> &EventsCcastopped {
        &self.events_ccastopped
    }
    #[doc = "0x250 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit"]
    #[inline(always)]
    pub const fn events_rateboost(&self) -> &EventsRateboost {
        &self.events_rateboost
    }
    #[doc = "0x254 - MAC header match found"]
    #[inline(always)]
    pub const fn events_mhrmatch(&self) -> &EventsMhrmatch {
        &self.events_mhrmatch
    }
    #[doc = "0x258 - Initial sync detected"]
    #[inline(always)]
    pub const fn events_sync(&self) -> &EventsSync {
        &self.events_sync
    }
    #[doc = "0x25c - CTEInfo byte is received"]
    #[inline(always)]
    pub const fn events_ctepresent(&self) -> &EventsCtepresent {
        &self.events_ctepresent
    }
    #[doc = "0x300 - Publish configuration for event READY"]
    #[inline(always)]
    pub const fn publish_ready(&self) -> &PublishReady {
        &self.publish_ready
    }
    #[doc = "0x304 - Publish configuration for event TXREADY"]
    #[inline(always)]
    pub const fn publish_txready(&self) -> &PublishTxready {
        &self.publish_txready
    }
    #[doc = "0x308 - Publish configuration for event RXREADY"]
    #[inline(always)]
    pub const fn publish_rxready(&self) -> &PublishRxready {
        &self.publish_rxready
    }
    #[doc = "0x30c - Publish configuration for event ADDRESS"]
    #[inline(always)]
    pub const fn publish_address(&self) -> &PublishAddress {
        &self.publish_address
    }
    #[doc = "0x310 - Publish configuration for event FRAMESTART"]
    #[inline(always)]
    pub const fn publish_framestart(&self) -> &PublishFramestart {
        &self.publish_framestart
    }
    #[doc = "0x314 - Publish configuration for event PAYLOAD"]
    #[inline(always)]
    pub const fn publish_payload(&self) -> &PublishPayload {
        &self.publish_payload
    }
    #[doc = "0x318 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(&self) -> &PublishEnd {
        &self.publish_end
    }
    #[doc = "0x31c - Publish configuration for event PHYEND"]
    #[inline(always)]
    pub const fn publish_phyend(&self) -> &PublishPhyend {
        &self.publish_phyend
    }
    #[doc = "0x320 - Publish configuration for event DISABLED"]
    #[inline(always)]
    pub const fn publish_disabled(&self) -> &PublishDisabled {
        &self.publish_disabled
    }
    #[doc = "0x324 - Publish configuration for event DEVMATCH"]
    #[inline(always)]
    pub const fn publish_devmatch(&self) -> &PublishDevmatch {
        &self.publish_devmatch
    }
    #[doc = "0x328 - Publish configuration for event DEVMISS"]
    #[inline(always)]
    pub const fn publish_devmiss(&self) -> &PublishDevmiss {
        &self.publish_devmiss
    }
    #[doc = "0x32c - Publish configuration for event CRCOK"]
    #[inline(always)]
    pub const fn publish_crcok(&self) -> &PublishCrcok {
        &self.publish_crcok
    }
    #[doc = "0x330 - Publish configuration for event CRCERROR"]
    #[inline(always)]
    pub const fn publish_crcerror(&self) -> &PublishCrcerror {
        &self.publish_crcerror
    }
    #[doc = "0x338 - Publish configuration for event BCMATCH"]
    #[inline(always)]
    pub const fn publish_bcmatch(&self) -> &PublishBcmatch {
        &self.publish_bcmatch
    }
    #[doc = "0x33c - Publish configuration for event EDEND"]
    #[inline(always)]
    pub const fn publish_edend(&self) -> &PublishEdend {
        &self.publish_edend
    }
    #[doc = "0x340 - Publish configuration for event EDSTOPPED"]
    #[inline(always)]
    pub const fn publish_edstopped(&self) -> &PublishEdstopped {
        &self.publish_edstopped
    }
    #[doc = "0x344 - Publish configuration for event CCAIDLE"]
    #[inline(always)]
    pub const fn publish_ccaidle(&self) -> &PublishCcaidle {
        &self.publish_ccaidle
    }
    #[doc = "0x348 - Publish configuration for event CCABUSY"]
    #[inline(always)]
    pub const fn publish_ccabusy(&self) -> &PublishCcabusy {
        &self.publish_ccabusy
    }
    #[doc = "0x34c - Publish configuration for event CCASTOPPED"]
    #[inline(always)]
    pub const fn publish_ccastopped(&self) -> &PublishCcastopped {
        &self.publish_ccastopped
    }
    #[doc = "0x350 - Publish configuration for event RATEBOOST"]
    #[inline(always)]
    pub const fn publish_rateboost(&self) -> &PublishRateboost {
        &self.publish_rateboost
    }
    #[doc = "0x354 - Publish configuration for event MHRMATCH"]
    #[inline(always)]
    pub const fn publish_mhrmatch(&self) -> &PublishMhrmatch {
        &self.publish_mhrmatch
    }
    #[doc = "0x358 - Publish configuration for event SYNC"]
    #[inline(always)]
    pub const fn publish_sync(&self) -> &PublishSync {
        &self.publish_sync
    }
    #[doc = "0x35c - Publish configuration for event CTEPRESENT"]
    #[inline(always)]
    pub const fn publish_ctepresent(&self) -> &PublishCtepresent {
        &self.publish_ctepresent
    }
    #[doc = "0x400 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x488 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset00(&self) -> &Intenset00 {
        &self.intenset00
    }
    #[doc = "0x490 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr00(&self) -> &Intenclr00 {
        &self.intenclr00
    }
    #[doc = "0x4a8 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset10(&self) -> &Intenset10 {
        &self.intenset10
    }
    #[doc = "0x4b0 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr10(&self) -> &Intenclr10 {
        &self.intenclr10
    }
    #[doc = "0x500 - Data rate and modulation"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x520 - Current radio state"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x530 - IEEE 802.15.4 energy detect control"]
    #[inline(always)]
    pub const fn edctrl(&self) -> &Edctrl {
        &self.edctrl
    }
    #[doc = "0x534 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub const fn edsample(&self) -> &Edsample {
        &self.edsample
    }
    #[doc = "0x538 - IEEE 802.15.4 clear channel assessment control"]
    #[inline(always)]
    pub const fn ccactrl(&self) -> &Ccactrl {
        &self.ccactrl
    }
    #[doc = "0x540 - Data whitening configuration"]
    #[inline(always)]
    pub const fn datawhite(&self) -> &Datawhite {
        &self.datawhite
    }
    #[doc = "0x704 - Timing"]
    #[inline(always)]
    pub const fn timing(&self) -> &Timing {
        &self.timing
    }
    #[doc = "0x708 - Frequency"]
    #[inline(always)]
    pub const fn frequency(&self) -> &Frequency {
        &self.frequency
    }
    #[doc = "0x710 - Output power"]
    #[inline(always)]
    pub const fn txpower(&self) -> &Txpower {
        &self.txpower
    }
    #[doc = "0x714 - Interframe spacing in us"]
    #[inline(always)]
    pub const fn tifs(&self) -> &Tifs {
        &self.tifs
    }
    #[doc = "0x718 - RSSI sample"]
    #[inline(always)]
    pub const fn rssisample(&self) -> &Rssisample {
        &self.rssisample
    }
    #[doc = "0x908 - Config register"]
    #[inline(always)]
    pub const fn feconfig(&self) -> &Feconfig {
        &self.feconfig
    }
    #[doc = "0xd00 - Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
    #[inline(always)]
    pub const fn dfemode(&self) -> &Dfemode {
        &self.dfemode
    }
    #[doc = "0xd04 - DFE status information"]
    #[inline(always)]
    pub const fn dfestatus(&self) -> &Dfestatus {
        &self.dfestatus
    }
    #[doc = "0xd10 - Various configuration for Direction finding"]
    #[inline(always)]
    pub const fn dfectrl1(&self) -> &Dfectrl1 {
        &self.dfectrl1
    }
    #[doc = "0xd14 - Start offset for Direction finding"]
    #[inline(always)]
    pub const fn dfectrl2(&self) -> &Dfectrl2 {
        &self.dfectrl2
    }
    #[doc = "0xd28 - GPIO patterns to be used for each antenna"]
    #[inline(always)]
    pub const fn switchpattern(&self) -> &Switchpattern {
        &self.switchpattern
    }
    #[doc = "0xd2c - Clear the GPIO pattern array for antenna control"]
    #[inline(always)]
    pub const fn clearpattern(&self) -> &Clearpattern {
        &self.clearpattern
    }
    #[doc = "0xd30..0xd50 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0xd50..0xd60 - DFE packet EasyDMA channel"]
    #[inline(always)]
    pub const fn dfepacket(&self) -> &Dfepacket {
        &self.dfepacket
    }
    #[doc = "0xe0c - CRC status"]
    #[inline(always)]
    pub const fn crcstatus(&self) -> &Crcstatus {
        &self.crcstatus
    }
    #[doc = "0xe10 - Received address"]
    #[inline(always)]
    pub const fn rxmatch(&self) -> &Rxmatch {
        &self.rxmatch
    }
    #[doc = "0xe14 - CRC field of previously received packet"]
    #[inline(always)]
    pub const fn rxcrc(&self) -> &Rxcrc {
        &self.rxcrc
    }
    #[doc = "0xe18 - Device address match index"]
    #[inline(always)]
    pub const fn dai(&self) -> &Dai {
        &self.dai
    }
    #[doc = "0xe1c - Payload status"]
    #[inline(always)]
    pub const fn pdustat(&self) -> &Pdustat {
        &self.pdustat
    }
    #[doc = "0xe20 - Packet configuration register 0"]
    #[inline(always)]
    pub const fn pcnf0(&self) -> &Pcnf0 {
        &self.pcnf0
    }
    #[doc = "0xe28 - Packet configuration register 1"]
    #[inline(always)]
    pub const fn pcnf1(&self) -> &Pcnf1 {
        &self.pcnf1
    }
    #[doc = "0xe2c - Base address 0"]
    #[inline(always)]
    pub const fn base0(&self) -> &Base0 {
        &self.base0
    }
    #[doc = "0xe30 - Base address 1"]
    #[inline(always)]
    pub const fn base1(&self) -> &Base1 {
        &self.base1
    }
    #[doc = "0xe34 - Prefixes bytes for logical addresses 0-3"]
    #[inline(always)]
    pub const fn prefix0(&self) -> &Prefix0 {
        &self.prefix0
    }
    #[doc = "0xe38 - Prefixes bytes for logical addresses 4-7"]
    #[inline(always)]
    pub const fn prefix1(&self) -> &Prefix1 {
        &self.prefix1
    }
    #[doc = "0xe3c - Transmit address select"]
    #[inline(always)]
    pub const fn txaddress(&self) -> &Txaddress {
        &self.txaddress
    }
    #[doc = "0xe40 - Receive address select"]
    #[inline(always)]
    pub const fn rxaddresses(&self) -> &Rxaddresses {
        &self.rxaddresses
    }
    #[doc = "0xe44 - CRC configuration"]
    #[inline(always)]
    pub const fn crccnf(&self) -> &Crccnf {
        &self.crccnf
    }
    #[doc = "0xe48 - CRC polynomial"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0xe4c - CRC initial value"]
    #[inline(always)]
    pub const fn crcinit(&self) -> &Crcinit {
        &self.crcinit
    }
    #[doc = "0xe50..0xe70 - Description collection: Device address base segment n"]
    #[inline(always)]
    pub const fn dab(&self, n: usize) -> &Dab {
        &self.dab[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe50..0xe70 - Description collection: Device address base segment n"]
    #[inline(always)]
    pub fn dab_iter(&self) -> impl Iterator<Item = &Dab> {
        self.dab.iter()
    }
    #[doc = "0xe70..0xe90 - Description collection: Device address prefix n"]
    #[inline(always)]
    pub const fn dap(&self, n: usize) -> &Dap {
        &self.dap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe70..0xe90 - Description collection: Device address prefix n"]
    #[inline(always)]
    pub fn dap_iter(&self) -> impl Iterator<Item = &Dap> {
        self.dap.iter()
    }
    #[doc = "0xe90 - Device address match configuration"]
    #[inline(always)]
    pub const fn dacnf(&self) -> &Dacnf {
        &self.dacnf
    }
    #[doc = "0xe94 - Bit counter compare"]
    #[inline(always)]
    pub const fn bcc(&self) -> &Bcc {
        &self.bcc
    }
    #[doc = "0xea4 - CTEInfo parsed from received packet"]
    #[inline(always)]
    pub const fn ctestatus(&self) -> &Ctestatus {
        &self.ctestatus
    }
    #[doc = "0xeb4 - Search pattern configuration"]
    #[inline(always)]
    pub const fn mhrmatchconf(&self) -> &Mhrmatchconf {
        &self.mhrmatchconf
    }
    #[doc = "0xeb8 - Pattern mask"]
    #[inline(always)]
    pub const fn mhrmatchmask(&self) -> &Mhrmatchmask {
        &self.mhrmatchmask
    }
    #[doc = "0xebc - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub const fn sfd(&self) -> &Sfd {
        &self.sfd
    }
    #[doc = "0xec0 - Configuration for CTE inline mode"]
    #[inline(always)]
    pub const fn cteinlineconf(&self) -> &Cteinlineconf {
        &self.cteinlineconf
    }
    #[doc = "0xed0 - Packet pointer"]
    #[inline(always)]
    pub const fn packetptr(&self) -> &Packetptr {
        &self.packetptr
    }
    #[doc = "0x1000..0x1050 - Unspecified"]
    #[inline(always)]
    pub const fn cstones(&self) -> &Cstones {
        &self.cstones
    }
    #[doc = "0x1050..0x1064 - Unspecified"]
    #[inline(always)]
    pub const fn rtt(&self) -> &Rtt {
        &self.rtt
    }
}
#[doc = "TASKS_TXEN (w) register accessor: Enable RADIO in TX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_txen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_txen`]
module"]
#[doc(alias = "TASKS_TXEN")]
pub type TasksTxen = crate::Reg<tasks_txen::TasksTxenSpec>;
#[doc = "Enable RADIO in TX mode"]
pub mod tasks_txen;
#[doc = "TASKS_RXEN (w) register accessor: Enable RADIO in RX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rxen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rxen`]
module"]
#[doc(alias = "TASKS_RXEN")]
pub type TasksRxen = crate::Reg<tasks_rxen::TasksRxenSpec>;
#[doc = "Enable RADIO in RX mode"]
pub mod tasks_rxen;
#[doc = "TASKS_START (w) register accessor: Start RADIO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start RADIO"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop RADIO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop RADIO"]
pub mod tasks_stop;
#[doc = "TASKS_DISABLE (w) register accessor: Disable RADIO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_disable`]
module"]
#[doc(alias = "TASKS_DISABLE")]
pub type TasksDisable = crate::Reg<tasks_disable::TasksDisableSpec>;
#[doc = "Disable RADIO"]
pub mod tasks_disable;
#[doc = "TASKS_RSSISTART (w) register accessor: Start the RSSI and take one single sample of the receive signal strength\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rssistart`]
module"]
#[doc(alias = "TASKS_RSSISTART")]
pub type TasksRssistart = crate::Reg<tasks_rssistart::TasksRssistartSpec>;
#[doc = "Start the RSSI and take one single sample of the receive signal strength"]
pub mod tasks_rssistart;
#[doc = "TASKS_BCSTART (w) register accessor: Start the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_bcstart`]
module"]
#[doc(alias = "TASKS_BCSTART")]
pub type TasksBcstart = crate::Reg<tasks_bcstart::TasksBcstartSpec>;
#[doc = "Start the bit counter"]
pub mod tasks_bcstart;
#[doc = "TASKS_BCSTOP (w) register accessor: Stop the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_bcstop`]
module"]
#[doc(alias = "TASKS_BCSTOP")]
pub type TasksBcstop = crate::Reg<tasks_bcstop::TasksBcstopSpec>;
#[doc = "Stop the bit counter"]
pub mod tasks_bcstop;
#[doc = "TASKS_EDSTART (w) register accessor: Start the energy detect measurement used in IEEE 802.15.4 mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_edstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_edstart`]
module"]
#[doc(alias = "TASKS_EDSTART")]
pub type TasksEdstart = crate::Reg<tasks_edstart::TasksEdstartSpec>;
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
pub mod tasks_edstart;
#[doc = "TASKS_EDSTOP (w) register accessor: Stop the energy detect measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_edstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_edstop`]
module"]
#[doc(alias = "TASKS_EDSTOP")]
pub type TasksEdstop = crate::Reg<tasks_edstop::TasksEdstopSpec>;
#[doc = "Stop the energy detect measurement"]
pub mod tasks_edstop;
#[doc = "TASKS_CCASTART (w) register accessor: Start the clear channel assessment used in IEEE 802.15.4 mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ccastart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ccastart`]
module"]
#[doc(alias = "TASKS_CCASTART")]
pub type TasksCcastart = crate::Reg<tasks_ccastart::TasksCcastartSpec>;
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
pub mod tasks_ccastart;
#[doc = "TASKS_CCASTOP (w) register accessor: Stop the clear channel assessment\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ccastop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ccastop`]
module"]
#[doc(alias = "TASKS_CCASTOP")]
pub type TasksCcastop = crate::Reg<tasks_ccastop::TasksCcastopSpec>;
#[doc = "Stop the clear channel assessment"]
pub mod tasks_ccastop;
#[doc = "TASKS_SOFTRESET (w) register accessor: Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_softreset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_softreset`]
module"]
#[doc(alias = "TASKS_SOFTRESET")]
pub type TasksSoftreset = crate::Reg<tasks_softreset::TasksSoftresetSpec>;
#[doc = "Reset all public registers, but with these exceptions: DMA registers and EVENT/INTEN/SUBSCRIBE/PUBLISH registers. Only to be used in DISABLED state."]
pub mod tasks_softreset;
#[doc = "SUBSCRIBE_TXEN (rw) register accessor: Subscribe configuration for task TXEN\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_txen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_txen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_txen`]
module"]
#[doc(alias = "SUBSCRIBE_TXEN")]
pub type SubscribeTxen = crate::Reg<subscribe_txen::SubscribeTxenSpec>;
#[doc = "Subscribe configuration for task TXEN"]
pub mod subscribe_txen;
#[doc = "SUBSCRIBE_RXEN (rw) register accessor: Subscribe configuration for task RXEN\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_rxen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_rxen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_rxen`]
module"]
#[doc(alias = "SUBSCRIBE_RXEN")]
pub type SubscribeRxen = crate::Reg<subscribe_rxen::SubscribeRxenSpec>;
#[doc = "Subscribe configuration for task RXEN"]
pub mod subscribe_rxen;
#[doc = "SUBSCRIBE_START (rw) register accessor: Subscribe configuration for task START\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_start`]
module"]
#[doc(alias = "SUBSCRIBE_START")]
pub type SubscribeStart = crate::Reg<subscribe_start::SubscribeStartSpec>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: Subscribe configuration for task STOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_stop`]
module"]
#[doc(alias = "SUBSCRIBE_STOP")]
pub type SubscribeStop = crate::Reg<subscribe_stop::SubscribeStopSpec>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_DISABLE (rw) register accessor: Subscribe configuration for task DISABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_disable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_disable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_disable`]
module"]
#[doc(alias = "SUBSCRIBE_DISABLE")]
pub type SubscribeDisable = crate::Reg<subscribe_disable::SubscribeDisableSpec>;
#[doc = "Subscribe configuration for task DISABLE"]
pub mod subscribe_disable;
#[doc = "SUBSCRIBE_RSSISTART (rw) register accessor: Subscribe configuration for task RSSISTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_rssistart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_rssistart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_rssistart`]
module"]
#[doc(alias = "SUBSCRIBE_RSSISTART")]
pub type SubscribeRssistart = crate::Reg<subscribe_rssistart::SubscribeRssistartSpec>;
#[doc = "Subscribe configuration for task RSSISTART"]
pub mod subscribe_rssistart;
#[doc = "SUBSCRIBE_BCSTART (rw) register accessor: Subscribe configuration for task BCSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_bcstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_bcstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_bcstart`]
module"]
#[doc(alias = "SUBSCRIBE_BCSTART")]
pub type SubscribeBcstart = crate::Reg<subscribe_bcstart::SubscribeBcstartSpec>;
#[doc = "Subscribe configuration for task BCSTART"]
pub mod subscribe_bcstart;
#[doc = "SUBSCRIBE_BCSTOP (rw) register accessor: Subscribe configuration for task BCSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_bcstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_bcstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_bcstop`]
module"]
#[doc(alias = "SUBSCRIBE_BCSTOP")]
pub type SubscribeBcstop = crate::Reg<subscribe_bcstop::SubscribeBcstopSpec>;
#[doc = "Subscribe configuration for task BCSTOP"]
pub mod subscribe_bcstop;
#[doc = "SUBSCRIBE_EDSTART (rw) register accessor: Subscribe configuration for task EDSTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_edstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_edstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_edstart`]
module"]
#[doc(alias = "SUBSCRIBE_EDSTART")]
pub type SubscribeEdstart = crate::Reg<subscribe_edstart::SubscribeEdstartSpec>;
#[doc = "Subscribe configuration for task EDSTART"]
pub mod subscribe_edstart;
#[doc = "SUBSCRIBE_EDSTOP (rw) register accessor: Subscribe configuration for task EDSTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_edstop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_edstop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_edstop`]
module"]
#[doc(alias = "SUBSCRIBE_EDSTOP")]
pub type SubscribeEdstop = crate::Reg<subscribe_edstop::SubscribeEdstopSpec>;
#[doc = "Subscribe configuration for task EDSTOP"]
pub mod subscribe_edstop;
#[doc = "SUBSCRIBE_CCASTART (rw) register accessor: Subscribe configuration for task CCASTART\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_ccastart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_ccastart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_ccastart`]
module"]
#[doc(alias = "SUBSCRIBE_CCASTART")]
pub type SubscribeCcastart = crate::Reg<subscribe_ccastart::SubscribeCcastartSpec>;
#[doc = "Subscribe configuration for task CCASTART"]
pub mod subscribe_ccastart;
#[doc = "SUBSCRIBE_CCASTOP (rw) register accessor: Subscribe configuration for task CCASTOP\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_ccastop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_ccastop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_ccastop`]
module"]
#[doc(alias = "SUBSCRIBE_CCASTOP")]
pub type SubscribeCcastop = crate::Reg<subscribe_ccastop::SubscribeCcastopSpec>;
#[doc = "Subscribe configuration for task CCASTOP"]
pub mod subscribe_ccastop;
#[doc = "SUBSCRIBE_SOFTRESET (rw) register accessor: Subscribe configuration for task SOFTRESET\n\nYou can [`read`](crate::Reg::read) this register and get [`subscribe_softreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subscribe_softreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_softreset`]
module"]
#[doc(alias = "SUBSCRIBE_SOFTRESET")]
pub type SubscribeSoftreset = crate::Reg<subscribe_softreset::SubscribeSoftresetSpec>;
#[doc = "Subscribe configuration for task SOFTRESET"]
pub mod subscribe_softreset;
#[doc = "EVENTS_READY (rw) register accessor: RADIO has ramped up and is ready to be started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`]
module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "RADIO has ramped up and is ready to be started"]
pub mod events_ready;
#[doc = "EVENTS_TXREADY (rw) register accessor: RADIO has ramped up and is ready to be started TX path\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txready`]
module"]
#[doc(alias = "EVENTS_TXREADY")]
pub type EventsTxready = crate::Reg<events_txready::EventsTxreadySpec>;
#[doc = "RADIO has ramped up and is ready to be started TX path"]
pub mod events_txready;
#[doc = "EVENTS_RXREADY (rw) register accessor: RADIO has ramped up and is ready to be started RX path\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxready`]
module"]
#[doc(alias = "EVENTS_RXREADY")]
pub type EventsRxready = crate::Reg<events_rxready::EventsRxreadySpec>;
#[doc = "RADIO has ramped up and is ready to be started RX path"]
pub mod events_rxready;
#[doc = "EVENTS_ADDRESS (rw) register accessor: Address sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_address`]
module"]
#[doc(alias = "EVENTS_ADDRESS")]
pub type EventsAddress = crate::Reg<events_address::EventsAddressSpec>;
#[doc = "Address sent or received"]
pub mod events_address;
#[doc = "EVENTS_FRAMESTART (rw) register accessor: IEEE 802.15.4 length field received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_framestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_framestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_framestart`]
module"]
#[doc(alias = "EVENTS_FRAMESTART")]
pub type EventsFramestart = crate::Reg<events_framestart::EventsFramestartSpec>;
#[doc = "IEEE 802.15.4 length field received"]
pub mod events_framestart;
#[doc = "EVENTS_PAYLOAD (rw) register accessor: Packet payload sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_payload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_payload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_payload`]
module"]
#[doc(alias = "EVENTS_PAYLOAD")]
pub type EventsPayload = crate::Reg<events_payload::EventsPayloadSpec>;
#[doc = "Packet payload sent or received"]
pub mod events_payload;
#[doc = "EVENTS_END (rw) register accessor: Memory access for packet data has been completed\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Memory access for packet data has been completed"]
pub mod events_end;
#[doc = "EVENTS_PHYEND (rw) register accessor: The last bit is sent on air or last bit is received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_phyend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_phyend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_phyend`]
module"]
#[doc(alias = "EVENTS_PHYEND")]
pub type EventsPhyend = crate::Reg<events_phyend::EventsPhyendSpec>;
#[doc = "The last bit is sent on air or last bit is received"]
pub mod events_phyend;
#[doc = "EVENTS_DISABLED (rw) register accessor: RADIO has been disabled\n\nYou can [`read`](crate::Reg::read) this register and get [`events_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_disabled`]
module"]
#[doc(alias = "EVENTS_DISABLED")]
pub type EventsDisabled = crate::Reg<events_disabled::EventsDisabledSpec>;
#[doc = "RADIO has been disabled"]
pub mod events_disabled;
#[doc = "EVENTS_DEVMATCH (rw) register accessor: A device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_devmatch`]
module"]
#[doc(alias = "EVENTS_DEVMATCH")]
pub type EventsDevmatch = crate::Reg<events_devmatch::EventsDevmatchSpec>;
#[doc = "A device address match occurred on the last received packet"]
pub mod events_devmatch;
#[doc = "EVENTS_DEVMISS (rw) register accessor: No device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmiss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmiss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_devmiss`]
module"]
#[doc(alias = "EVENTS_DEVMISS")]
pub type EventsDevmiss = crate::Reg<events_devmiss::EventsDevmissSpec>;
#[doc = "No device address match occurred on the last received packet"]
pub mod events_devmiss;
#[doc = "EVENTS_CRCOK (rw) register accessor: Packet received with CRC ok\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcok::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcok::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_crcok`]
module"]
#[doc(alias = "EVENTS_CRCOK")]
pub type EventsCrcok = crate::Reg<events_crcok::EventsCrcokSpec>;
#[doc = "Packet received with CRC ok"]
pub mod events_crcok;
#[doc = "EVENTS_CRCERROR (rw) register accessor: Packet received with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_crcerror`]
module"]
#[doc(alias = "EVENTS_CRCERROR")]
pub type EventsCrcerror = crate::Reg<events_crcerror::EventsCrcerrorSpec>;
#[doc = "Packet received with CRC error"]
pub mod events_crcerror;
#[doc = "EVENTS_BCMATCH (rw) register accessor: Bit counter reached bit count value\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bcmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bcmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_bcmatch`]
module"]
#[doc(alias = "EVENTS_BCMATCH")]
pub type EventsBcmatch = crate::Reg<events_bcmatch::EventsBcmatchSpec>;
#[doc = "Bit counter reached bit count value"]
pub mod events_bcmatch;
#[doc = "EVENTS_EDEND (rw) register accessor: Sampling of energy detection complete (a new ED sample is ready for readout from the RADIO.EDSAMPLE register)\n\nYou can [`read`](crate::Reg::read) this register and get [`events_edend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_edend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_edend`]
module"]
#[doc(alias = "EVENTS_EDEND")]
pub type EventsEdend = crate::Reg<events_edend::EventsEdendSpec>;
#[doc = "Sampling of energy detection complete (a new ED sample is ready for readout from the RADIO.EDSAMPLE register)"]
pub mod events_edend;
#[doc = "EVENTS_EDSTOPPED (rw) register accessor: The sampling of energy detection has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_edstopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_edstopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_edstopped`]
module"]
#[doc(alias = "EVENTS_EDSTOPPED")]
pub type EventsEdstopped = crate::Reg<events_edstopped::EventsEdstoppedSpec>;
#[doc = "The sampling of energy detection has stopped"]
pub mod events_edstopped;
#[doc = "EVENTS_CCAIDLE (rw) register accessor: Wireless medium in idle - clear to send\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccaidle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccaidle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ccaidle`]
module"]
#[doc(alias = "EVENTS_CCAIDLE")]
pub type EventsCcaidle = crate::Reg<events_ccaidle::EventsCcaidleSpec>;
#[doc = "Wireless medium in idle - clear to send"]
pub mod events_ccaidle;
#[doc = "EVENTS_CCABUSY (rw) register accessor: Wireless medium busy - do not send\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ccabusy`]
module"]
#[doc(alias = "EVENTS_CCABUSY")]
pub type EventsCcabusy = crate::Reg<events_ccabusy::EventsCcabusySpec>;
#[doc = "Wireless medium busy - do not send"]
pub mod events_ccabusy;
#[doc = "EVENTS_CCASTOPPED (rw) register accessor: The CCA has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccastopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccastopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ccastopped`]
module"]
#[doc(alias = "EVENTS_CCASTOPPED")]
pub type EventsCcastopped = crate::Reg<events_ccastopped::EventsCcastoppedSpec>;
#[doc = "The CCA has stopped"]
pub mod events_ccastopped;
#[doc = "EVENTS_RATEBOOST (rw) register accessor: Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rateboost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rateboost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rateboost`]
module"]
#[doc(alias = "EVENTS_RATEBOOST")]
pub type EventsRateboost = crate::Reg<events_rateboost::EventsRateboostSpec>;
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit"]
pub mod events_rateboost;
#[doc = "EVENTS_MHRMATCH (rw) register accessor: MAC header match found\n\nYou can [`read`](crate::Reg::read) this register and get [`events_mhrmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_mhrmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_mhrmatch`]
module"]
#[doc(alias = "EVENTS_MHRMATCH")]
pub type EventsMhrmatch = crate::Reg<events_mhrmatch::EventsMhrmatchSpec>;
#[doc = "MAC header match found"]
pub mod events_mhrmatch;
#[doc = "EVENTS_SYNC (rw) register accessor: Initial sync detected\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sync`]
module"]
#[doc(alias = "EVENTS_SYNC")]
pub type EventsSync = crate::Reg<events_sync::EventsSyncSpec>;
#[doc = "Initial sync detected"]
pub mod events_sync;
#[doc = "EVENTS_CTEPRESENT (rw) register accessor: CTEInfo byte is received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctepresent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctepresent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ctepresent`]
module"]
#[doc(alias = "EVENTS_CTEPRESENT")]
pub type EventsCtepresent = crate::Reg<events_ctepresent::EventsCtepresentSpec>;
#[doc = "CTEInfo byte is received"]
pub mod events_ctepresent;
#[doc = "PUBLISH_READY (rw) register accessor: Publish configuration for event READY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ready`]
module"]
#[doc(alias = "PUBLISH_READY")]
pub type PublishReady = crate::Reg<publish_ready::PublishReadySpec>;
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "PUBLISH_TXREADY (rw) register accessor: Publish configuration for event TXREADY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_txready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_txready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txready`]
module"]
#[doc(alias = "PUBLISH_TXREADY")]
pub type PublishTxready = crate::Reg<publish_txready::PublishTxreadySpec>;
#[doc = "Publish configuration for event TXREADY"]
pub mod publish_txready;
#[doc = "PUBLISH_RXREADY (rw) register accessor: Publish configuration for event RXREADY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rxready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rxready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxready`]
module"]
#[doc(alias = "PUBLISH_RXREADY")]
pub type PublishRxready = crate::Reg<publish_rxready::PublishRxreadySpec>;
#[doc = "Publish configuration for event RXREADY"]
pub mod publish_rxready;
#[doc = "PUBLISH_ADDRESS (rw) register accessor: Publish configuration for event ADDRESS\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_address`]
module"]
#[doc(alias = "PUBLISH_ADDRESS")]
pub type PublishAddress = crate::Reg<publish_address::PublishAddressSpec>;
#[doc = "Publish configuration for event ADDRESS"]
pub mod publish_address;
#[doc = "PUBLISH_FRAMESTART (rw) register accessor: Publish configuration for event FRAMESTART\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_framestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_framestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_framestart`]
module"]
#[doc(alias = "PUBLISH_FRAMESTART")]
pub type PublishFramestart = crate::Reg<publish_framestart::PublishFramestartSpec>;
#[doc = "Publish configuration for event FRAMESTART"]
pub mod publish_framestart;
#[doc = "PUBLISH_PAYLOAD (rw) register accessor: Publish configuration for event PAYLOAD\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_payload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_payload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_payload`]
module"]
#[doc(alias = "PUBLISH_PAYLOAD")]
pub type PublishPayload = crate::Reg<publish_payload::PublishPayloadSpec>;
#[doc = "Publish configuration for event PAYLOAD"]
pub mod publish_payload;
#[doc = "PUBLISH_END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_end`]
module"]
#[doc(alias = "PUBLISH_END")]
pub type PublishEnd = crate::Reg<publish_end::PublishEndSpec>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_PHYEND (rw) register accessor: Publish configuration for event PHYEND\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_phyend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_phyend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_phyend`]
module"]
#[doc(alias = "PUBLISH_PHYEND")]
pub type PublishPhyend = crate::Reg<publish_phyend::PublishPhyendSpec>;
#[doc = "Publish configuration for event PHYEND"]
pub mod publish_phyend;
#[doc = "PUBLISH_DISABLED (rw) register accessor: Publish configuration for event DISABLED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_disabled`]
module"]
#[doc(alias = "PUBLISH_DISABLED")]
pub type PublishDisabled = crate::Reg<publish_disabled::PublishDisabledSpec>;
#[doc = "Publish configuration for event DISABLED"]
pub mod publish_disabled;
#[doc = "PUBLISH_DEVMATCH (rw) register accessor: Publish configuration for event DEVMATCH\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_devmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_devmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_devmatch`]
module"]
#[doc(alias = "PUBLISH_DEVMATCH")]
pub type PublishDevmatch = crate::Reg<publish_devmatch::PublishDevmatchSpec>;
#[doc = "Publish configuration for event DEVMATCH"]
pub mod publish_devmatch;
#[doc = "PUBLISH_DEVMISS (rw) register accessor: Publish configuration for event DEVMISS\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_devmiss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_devmiss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_devmiss`]
module"]
#[doc(alias = "PUBLISH_DEVMISS")]
pub type PublishDevmiss = crate::Reg<publish_devmiss::PublishDevmissSpec>;
#[doc = "Publish configuration for event DEVMISS"]
pub mod publish_devmiss;
#[doc = "PUBLISH_CRCOK (rw) register accessor: Publish configuration for event CRCOK\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_crcok::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_crcok::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_crcok`]
module"]
#[doc(alias = "PUBLISH_CRCOK")]
pub type PublishCrcok = crate::Reg<publish_crcok::PublishCrcokSpec>;
#[doc = "Publish configuration for event CRCOK"]
pub mod publish_crcok;
#[doc = "PUBLISH_CRCERROR (rw) register accessor: Publish configuration for event CRCERROR\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_crcerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_crcerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_crcerror`]
module"]
#[doc(alias = "PUBLISH_CRCERROR")]
pub type PublishCrcerror = crate::Reg<publish_crcerror::PublishCrcerrorSpec>;
#[doc = "Publish configuration for event CRCERROR"]
pub mod publish_crcerror;
#[doc = "PUBLISH_BCMATCH (rw) register accessor: Publish configuration for event BCMATCH\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_bcmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_bcmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_bcmatch`]
module"]
#[doc(alias = "PUBLISH_BCMATCH")]
pub type PublishBcmatch = crate::Reg<publish_bcmatch::PublishBcmatchSpec>;
#[doc = "Publish configuration for event BCMATCH"]
pub mod publish_bcmatch;
#[doc = "PUBLISH_EDEND (rw) register accessor: Publish configuration for event EDEND\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_edend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_edend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_edend`]
module"]
#[doc(alias = "PUBLISH_EDEND")]
pub type PublishEdend = crate::Reg<publish_edend::PublishEdendSpec>;
#[doc = "Publish configuration for event EDEND"]
pub mod publish_edend;
#[doc = "PUBLISH_EDSTOPPED (rw) register accessor: Publish configuration for event EDSTOPPED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_edstopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_edstopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_edstopped`]
module"]
#[doc(alias = "PUBLISH_EDSTOPPED")]
pub type PublishEdstopped = crate::Reg<publish_edstopped::PublishEdstoppedSpec>;
#[doc = "Publish configuration for event EDSTOPPED"]
pub mod publish_edstopped;
#[doc = "PUBLISH_CCAIDLE (rw) register accessor: Publish configuration for event CCAIDLE\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ccaidle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ccaidle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ccaidle`]
module"]
#[doc(alias = "PUBLISH_CCAIDLE")]
pub type PublishCcaidle = crate::Reg<publish_ccaidle::PublishCcaidleSpec>;
#[doc = "Publish configuration for event CCAIDLE"]
pub mod publish_ccaidle;
#[doc = "PUBLISH_CCABUSY (rw) register accessor: Publish configuration for event CCABUSY\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ccabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ccabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ccabusy`]
module"]
#[doc(alias = "PUBLISH_CCABUSY")]
pub type PublishCcabusy = crate::Reg<publish_ccabusy::PublishCcabusySpec>;
#[doc = "Publish configuration for event CCABUSY"]
pub mod publish_ccabusy;
#[doc = "PUBLISH_CCASTOPPED (rw) register accessor: Publish configuration for event CCASTOPPED\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ccastopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ccastopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ccastopped`]
module"]
#[doc(alias = "PUBLISH_CCASTOPPED")]
pub type PublishCcastopped = crate::Reg<publish_ccastopped::PublishCcastoppedSpec>;
#[doc = "Publish configuration for event CCASTOPPED"]
pub mod publish_ccastopped;
#[doc = "PUBLISH_RATEBOOST (rw) register accessor: Publish configuration for event RATEBOOST\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_rateboost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_rateboost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rateboost`]
module"]
#[doc(alias = "PUBLISH_RATEBOOST")]
pub type PublishRateboost = crate::Reg<publish_rateboost::PublishRateboostSpec>;
#[doc = "Publish configuration for event RATEBOOST"]
pub mod publish_rateboost;
#[doc = "PUBLISH_MHRMATCH (rw) register accessor: Publish configuration for event MHRMATCH\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_mhrmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_mhrmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_mhrmatch`]
module"]
#[doc(alias = "PUBLISH_MHRMATCH")]
pub type PublishMhrmatch = crate::Reg<publish_mhrmatch::PublishMhrmatchSpec>;
#[doc = "Publish configuration for event MHRMATCH"]
pub mod publish_mhrmatch;
#[doc = "PUBLISH_SYNC (rw) register accessor: Publish configuration for event SYNC\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_sync`]
module"]
#[doc(alias = "PUBLISH_SYNC")]
pub type PublishSync = crate::Reg<publish_sync::PublishSyncSpec>;
#[doc = "Publish configuration for event SYNC"]
pub mod publish_sync;
#[doc = "PUBLISH_CTEPRESENT (rw) register accessor: Publish configuration for event CTEPRESENT\n\nYou can [`read`](crate::Reg::read) this register and get [`publish_ctepresent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`publish_ctepresent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_ctepresent`]
module"]
#[doc(alias = "PUBLISH_CTEPRESENT")]
pub type PublishCtepresent = crate::Reg<publish_ctepresent::PublishCtepresentSpec>;
#[doc = "Publish configuration for event CTEPRESENT"]
pub mod publish_ctepresent;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET00 (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset00`]
module"]
#[doc(alias = "INTENSET00")]
pub type Intenset00 = crate::Reg<intenset00::Intenset00Spec>;
#[doc = "Enable interrupt"]
pub mod intenset00;
#[doc = "INTENCLR00 (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr00`]
module"]
#[doc(alias = "INTENCLR00")]
pub type Intenclr00 = crate::Reg<intenclr00::Intenclr00Spec>;
#[doc = "Disable interrupt"]
pub mod intenclr00;
#[doc = "INTENSET10 (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset10`]
module"]
#[doc(alias = "INTENSET10")]
pub type Intenset10 = crate::Reg<intenset10::Intenset10Spec>;
#[doc = "Enable interrupt"]
pub mod intenset10;
#[doc = "INTENCLR10 (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr10`]
module"]
#[doc(alias = "INTENCLR10")]
pub type Intenclr10 = crate::Reg<intenclr10::Intenclr10Spec>;
#[doc = "Disable interrupt"]
pub mod intenclr10;
#[doc = "MODE (rw) register accessor: Data rate and modulation\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Data rate and modulation"]
pub mod mode;
#[doc = "STATE (r) register accessor: Current radio state\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Current radio state"]
pub mod state;
#[doc = "EDCTRL (rw) register accessor: IEEE 802.15.4 energy detect control\n\nYou can [`read`](crate::Reg::read) this register and get [`edctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edctrl`]
module"]
#[doc(alias = "EDCTRL")]
pub type Edctrl = crate::Reg<edctrl::EdctrlSpec>;
#[doc = "IEEE 802.15.4 energy detect control"]
pub mod edctrl;
#[doc = "EDSAMPLE (r) register accessor: IEEE 802.15.4 energy detect level\n\nYou can [`read`](crate::Reg::read) this register and get [`edsample::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edsample`]
module"]
#[doc(alias = "EDSAMPLE")]
pub type Edsample = crate::Reg<edsample::EdsampleSpec>;
#[doc = "IEEE 802.15.4 energy detect level"]
pub mod edsample;
#[doc = "CCACTRL (rw) register accessor: IEEE 802.15.4 clear channel assessment control\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccactrl`]
module"]
#[doc(alias = "CCACTRL")]
pub type Ccactrl = crate::Reg<ccactrl::CcactrlSpec>;
#[doc = "IEEE 802.15.4 clear channel assessment control"]
pub mod ccactrl;
#[doc = "DATAWHITE (rw) register accessor: Data whitening configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`datawhite::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datawhite::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datawhite`]
module"]
#[doc(alias = "DATAWHITE")]
pub type Datawhite = crate::Reg<datawhite::DatawhiteSpec>;
#[doc = "Data whitening configuration"]
pub mod datawhite;
#[doc = "TIMING (rw) register accessor: Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing`]
module"]
#[doc(alias = "TIMING")]
pub type Timing = crate::Reg<timing::TimingSpec>;
#[doc = "Timing"]
pub mod timing;
#[doc = "FREQUENCY (rw) register accessor: Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`]
module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "Frequency"]
pub mod frequency;
#[doc = "TXPOWER (rw) register accessor: Output power\n\nYou can [`read`](crate::Reg::read) this register and get [`txpower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpower`]
module"]
#[doc(alias = "TXPOWER")]
pub type Txpower = crate::Reg<txpower::TxpowerSpec>;
#[doc = "Output power"]
pub mod txpower;
#[doc = "TIFS (rw) register accessor: Interframe spacing in us\n\nYou can [`read`](crate::Reg::read) this register and get [`tifs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifs`]
module"]
#[doc(alias = "TIFS")]
pub type Tifs = crate::Reg<tifs::TifsSpec>;
#[doc = "Interframe spacing in us"]
pub mod tifs;
#[doc = "RSSISAMPLE (r) register accessor: RSSI sample\n\nYou can [`read`](crate::Reg::read) this register and get [`rssisample::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssisample`]
module"]
#[doc(alias = "RSSISAMPLE")]
pub type Rssisample = crate::Reg<rssisample::RssisampleSpec>;
#[doc = "RSSI sample"]
pub mod rssisample;
#[doc = "FECONFIG (rw) register accessor: Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`feconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feconfig`]
module"]
#[doc(alias = "FECONFIG")]
pub type Feconfig = crate::Reg<feconfig::FeconfigSpec>;
#[doc = "Config register"]
pub mod feconfig;
#[doc = "DFEMODE (rw) register accessor: Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)\n\nYou can [`read`](crate::Reg::read) this register and get [`dfemode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfemode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfemode`]
module"]
#[doc(alias = "DFEMODE")]
pub type Dfemode = crate::Reg<dfemode::DfemodeSpec>;
#[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
pub mod dfemode;
#[doc = "DFESTATUS (r) register accessor: DFE status information\n\nYou can [`read`](crate::Reg::read) this register and get [`dfestatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfestatus`]
module"]
#[doc(alias = "DFESTATUS")]
pub type Dfestatus = crate::Reg<dfestatus::DfestatusSpec>;
#[doc = "DFE status information"]
pub mod dfestatus;
#[doc = "DFECTRL1 (rw) register accessor: Various configuration for Direction finding\n\nYou can [`read`](crate::Reg::read) this register and get [`dfectrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfectrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfectrl1`]
module"]
#[doc(alias = "DFECTRL1")]
pub type Dfectrl1 = crate::Reg<dfectrl1::Dfectrl1Spec>;
#[doc = "Various configuration for Direction finding"]
pub mod dfectrl1;
#[doc = "DFECTRL2 (rw) register accessor: Start offset for Direction finding\n\nYou can [`read`](crate::Reg::read) this register and get [`dfectrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfectrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfectrl2`]
module"]
#[doc(alias = "DFECTRL2")]
pub type Dfectrl2 = crate::Reg<dfectrl2::Dfectrl2Spec>;
#[doc = "Start offset for Direction finding"]
pub mod dfectrl2;
#[doc = "SWITCHPATTERN (rw) register accessor: GPIO patterns to be used for each antenna\n\nYou can [`read`](crate::Reg::read) this register and get [`switchpattern::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switchpattern::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@switchpattern`]
module"]
#[doc(alias = "SWITCHPATTERN")]
pub type Switchpattern = crate::Reg<switchpattern::SwitchpatternSpec>;
#[doc = "GPIO patterns to be used for each antenna"]
pub mod switchpattern;
#[doc = "CLEARPATTERN (w) register accessor: Clear the GPIO pattern array for antenna control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearpattern::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearpattern`]
module"]
#[doc(alias = "CLEARPATTERN")]
pub type Clearpattern = crate::Reg<clearpattern::ClearpatternSpec>;
#[doc = "Clear the GPIO pattern array for antenna control"]
pub mod clearpattern;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "DFE packet EasyDMA channel"]
pub use self::dfepacket::Dfepacket;
#[doc = r"Cluster"]
#[doc = "DFE packet EasyDMA channel"]
pub mod dfepacket;
#[doc = "CRCSTATUS (r) register accessor: CRC status\n\nYou can [`read`](crate::Reg::read) this register and get [`crcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcstatus`]
module"]
#[doc(alias = "CRCSTATUS")]
pub type Crcstatus = crate::Reg<crcstatus::CrcstatusSpec>;
#[doc = "CRC status"]
pub mod crcstatus;
#[doc = "RXMATCH (r) register accessor: Received address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmatch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmatch`]
module"]
#[doc(alias = "RXMATCH")]
pub type Rxmatch = crate::Reg<rxmatch::RxmatchSpec>;
#[doc = "Received address"]
pub mod rxmatch;
#[doc = "RXCRC (r) register accessor: CRC field of previously received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrc`]
module"]
#[doc(alias = "RXCRC")]
pub type Rxcrc = crate::Reg<rxcrc::RxcrcSpec>;
#[doc = "CRC field of previously received packet"]
pub mod rxcrc;
#[doc = "DAI (r) register accessor: Device address match index\n\nYou can [`read`](crate::Reg::read) this register and get [`dai::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dai`]
module"]
#[doc(alias = "DAI")]
pub type Dai = crate::Reg<dai::DaiSpec>;
#[doc = "Device address match index"]
pub mod dai;
#[doc = "PDUSTAT (r) register accessor: Payload status\n\nYou can [`read`](crate::Reg::read) this register and get [`pdustat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdustat`]
module"]
#[doc(alias = "PDUSTAT")]
pub type Pdustat = crate::Reg<pdustat::PdustatSpec>;
#[doc = "Payload status"]
pub mod pdustat;
#[doc = "PCNF0 (rw) register accessor: Packet configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnf0`]
module"]
#[doc(alias = "PCNF0")]
pub type Pcnf0 = crate::Reg<pcnf0::Pcnf0Spec>;
#[doc = "Packet configuration register 0"]
pub mod pcnf0;
#[doc = "PCNF1 (rw) register accessor: Packet configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnf1`]
module"]
#[doc(alias = "PCNF1")]
pub type Pcnf1 = crate::Reg<pcnf1::Pcnf1Spec>;
#[doc = "Packet configuration register 1"]
pub mod pcnf1;
#[doc = "BASE0 (rw) register accessor: Base address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`base0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base0`]
module"]
#[doc(alias = "BASE0")]
pub type Base0 = crate::Reg<base0::Base0Spec>;
#[doc = "Base address 0"]
pub mod base0;
#[doc = "BASE1 (rw) register accessor: Base address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`base1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base1`]
module"]
#[doc(alias = "BASE1")]
pub type Base1 = crate::Reg<base1::Base1Spec>;
#[doc = "Base address 1"]
pub mod base1;
#[doc = "PREFIX0 (rw) register accessor: Prefixes bytes for logical addresses 0-3\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefix0`]
module"]
#[doc(alias = "PREFIX0")]
pub type Prefix0 = crate::Reg<prefix0::Prefix0Spec>;
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub mod prefix0;
#[doc = "PREFIX1 (rw) register accessor: Prefixes bytes for logical addresses 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefix1`]
module"]
#[doc(alias = "PREFIX1")]
pub type Prefix1 = crate::Reg<prefix1::Prefix1Spec>;
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub mod prefix1;
#[doc = "TXADDRESS (rw) register accessor: Transmit address select\n\nYou can [`read`](crate::Reg::read) this register and get [`txaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txaddress`]
module"]
#[doc(alias = "TXADDRESS")]
pub type Txaddress = crate::Reg<txaddress::TxaddressSpec>;
#[doc = "Transmit address select"]
pub mod txaddress;
#[doc = "RXADDRESSES (rw) register accessor: Receive address select\n\nYou can [`read`](crate::Reg::read) this register and get [`rxaddresses::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxaddresses::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxaddresses`]
module"]
#[doc(alias = "RXADDRESSES")]
pub type Rxaddresses = crate::Reg<rxaddresses::RxaddressesSpec>;
#[doc = "Receive address select"]
pub mod rxaddresses;
#[doc = "CRCCNF (rw) register accessor: CRC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`crccnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccnf`]
module"]
#[doc(alias = "CRCCNF")]
pub type Crccnf = crate::Reg<crccnf::CrccnfSpec>;
#[doc = "CRC configuration"]
pub mod crccnf;
#[doc = "CRCPOLY (rw) register accessor: CRC polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`]
module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC polynomial"]
pub mod crcpoly;
#[doc = "CRCINIT (rw) register accessor: CRC initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinit`]
module"]
#[doc(alias = "CRCINIT")]
pub type Crcinit = crate::Reg<crcinit::CrcinitSpec>;
#[doc = "CRC initial value"]
pub mod crcinit;
#[doc = "DAB (rw) register accessor: Description collection: Device address base segment n\n\nYou can [`read`](crate::Reg::read) this register and get [`dab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dab`]
module"]
#[doc(alias = "DAB")]
pub type Dab = crate::Reg<dab::DabSpec>;
#[doc = "Description collection: Device address base segment n"]
pub mod dab;
#[doc = "DAP (rw) register accessor: Description collection: Device address prefix n\n\nYou can [`read`](crate::Reg::read) this register and get [`dap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dap`]
module"]
#[doc(alias = "DAP")]
pub type Dap = crate::Reg<dap::DapSpec>;
#[doc = "Description collection: Device address prefix n"]
pub mod dap;
#[doc = "DACNF (rw) register accessor: Device address match configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dacnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacnf`]
module"]
#[doc(alias = "DACNF")]
pub type Dacnf = crate::Reg<dacnf::DacnfSpec>;
#[doc = "Device address match configuration"]
pub mod dacnf;
#[doc = "BCC (rw) register accessor: Bit counter compare\n\nYou can [`read`](crate::Reg::read) this register and get [`bcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcc`]
module"]
#[doc(alias = "BCC")]
pub type Bcc = crate::Reg<bcc::BccSpec>;
#[doc = "Bit counter compare"]
pub mod bcc;
#[doc = "CTESTATUS (r) register accessor: CTEInfo parsed from received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`ctestatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctestatus`]
module"]
#[doc(alias = "CTESTATUS")]
pub type Ctestatus = crate::Reg<ctestatus::CtestatusSpec>;
#[doc = "CTEInfo parsed from received packet"]
pub mod ctestatus;
#[doc = "MHRMATCHCONF (rw) register accessor: Search pattern configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mhrmatchconf`]
module"]
#[doc(alias = "MHRMATCHCONF")]
pub type Mhrmatchconf = crate::Reg<mhrmatchconf::MhrmatchconfSpec>;
#[doc = "Search pattern configuration"]
pub mod mhrmatchconf;
#[doc = "MHRMATCHMASK (rw) register accessor: Pattern mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mhrmatchmask`]
module"]
#[doc(alias = "MHRMATCHMASK")]
pub type Mhrmatchmask = crate::Reg<mhrmatchmask::MhrmatchmaskSpec>;
#[doc = "Pattern mask"]
pub mod mhrmatchmask;
#[doc = "SFD (rw) register accessor: IEEE 802.15.4 start of frame delimiter\n\nYou can [`read`](crate::Reg::read) this register and get [`sfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfd`]
module"]
#[doc(alias = "SFD")]
pub type Sfd = crate::Reg<sfd::SfdSpec>;
#[doc = "IEEE 802.15.4 start of frame delimiter"]
pub mod sfd;
#[doc = "CTEINLINECONF (rw) register accessor: Configuration for CTE inline mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cteinlineconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cteinlineconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cteinlineconf`]
module"]
#[doc(alias = "CTEINLINECONF")]
pub type Cteinlineconf = crate::Reg<cteinlineconf::CteinlineconfSpec>;
#[doc = "Configuration for CTE inline mode"]
pub mod cteinlineconf;
#[doc = "PACKETPTR (rw) register accessor: Packet pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packetptr`]
module"]
#[doc(alias = "PACKETPTR")]
pub type Packetptr = crate::Reg<packetptr::PacketptrSpec>;
#[doc = "Packet pointer"]
pub mod packetptr;
#[doc = "Unspecified"]
pub use self::cstones::Cstones;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod cstones;
#[doc = "Unspecified"]
pub use self::rtt::Rtt;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rtt;
