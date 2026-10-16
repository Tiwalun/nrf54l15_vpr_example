use riscv::{read_write_csr, read_write_csr_field};

const MASK: usize = 0xFFFF;

read_write_csr! {
    /// Nordic Control Register
    Dir: 0xBC1,
    mask: MASK,
}

read_write_csr_field! {
    Dir,
    /// pins
    pins: [0:15]
}
