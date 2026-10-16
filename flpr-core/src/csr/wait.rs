use riscv::read_write_csr;

const MASK: usize = 0xFFFF_FFFF;

read_write_csr! {
    /// Nordic Control Register
    Wait: 0x7dc,
    mask: MASK,
}
