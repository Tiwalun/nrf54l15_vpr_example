use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = usize::MAX;

read_write_csr! {
    /// Nordic Control Register
    NordicCtrl: 0x7C0,
    mask: MASK,
}

read_write_csr_field! {
    NordicCtrl,
    /// Control bit to enable Real-Time Peripherals
    enable_rt_periph: 0,
}

read_write_csr_field! {
    NordicCtrl,
    /// Enable remap feature
    enable_remap: 3,
}

read_write_csr_field! {
    NordicCtrl,
    /// Enables the generation of IRQ number COUNTER_IRQ_NUM
    cnt_irq_enable: 6,
}

read_write_csr_field! {
    NordicCtrl,
    /// Arbitration priority on bus
    vpr_bus_prio: 7,
}

read_write_csr_field! {
    NordicCtrl,
    /// Arbitration priority on bus
    nordic_key: [16:31],
}
