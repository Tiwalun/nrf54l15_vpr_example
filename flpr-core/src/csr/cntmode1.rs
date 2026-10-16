use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0x7;

read_write_csr! {
    /// Nordic Control Register
    CntMode1: 0x7D1,
    mask: MASK,
}

read_write_csr_field! {
    CntMode1,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    cnt_mode1: [0:2]
}
