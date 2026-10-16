use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0x7;

read_write_csr! {
    /// Nordic Control Register
    CntMode0: 0x7D0,
    mask: MASK,
}

read_write_csr_field! {
    CntMode0,
    /// Counter value
    ///
    ///  Shares a physical register with MCYCLEH
    cnt_mode0: [0:2]
}
