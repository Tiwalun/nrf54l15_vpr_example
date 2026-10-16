use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0xFFFF;

read_write_csr! {
    /// Nordic Control Register
    Cnt1: 0x7d7,
    mask: MASK,
}

read_write_csr_field! {
    Cnt1,
    /// CNT1 value
    ///
    ///  Shares a physical register with MCYCLEH (upper 16 bits)
    value: [0:15]
}
