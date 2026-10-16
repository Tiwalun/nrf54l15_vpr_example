use riscv::write_only_csr;

const MASK: usize = usize::MAX;

write_only_csr! {
    /// Nordic Control Register
    CntAdd: 0x7D4,
    mask: MASK,
}
