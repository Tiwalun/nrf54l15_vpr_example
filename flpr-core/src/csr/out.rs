use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0xFFFF;

read_write_csr! {
    /// Nordic Control Register
    Out: 0xBC0,
    mask: MASK,
}

read_write_csr_field! {
    Out,
    /// pins
    pins: [0:15]
}
