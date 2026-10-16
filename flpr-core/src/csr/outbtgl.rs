use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0xFFFF;

read_write_csr! {
    /// Nordic Control Register
    OutBTgl: 0xBD2,
    mask: MASK,
}

read_write_csr_field! {
    OutBTgl,
    /// pins
    pins: [0:15]
}
