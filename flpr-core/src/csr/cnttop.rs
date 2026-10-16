use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = usize::MAX;

read_write_csr! {
    /// Nordic Control Register
    CntTop: 0x7D3,
    mask: MASK,
}

read_write_csr_field! {
    CntTop,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    cnt0_reload: [0:15]
}
read_write_csr_field! {
    CntTop,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    cnt1_reload: [16:31]
}
