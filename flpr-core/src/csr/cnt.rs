use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0xFFFF_FFFF;

read_write_csr! {
    /// Nordic Control Register
    Cnt: 0x7D2,
    mask: MASK,
}

read_write_csr_field! {
    Cnt,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    cnt0: [0:15]
}
read_write_csr_field! {
    Cnt,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    cnt1: [16:31]
}
