use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0xFFFF;

read_write_csr! {
    /// Nordic Control Register
    Cnt0: 0x7d5,
    mask: MASK,
}

read_write_csr_field! {
    Cnt0,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    value: [0:15]
}
