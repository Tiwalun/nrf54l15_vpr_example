use riscv::{write_only_csr, write_only_csr_field};

const MASK: usize = 0x1_FFFF;

write_only_csr! {
    /// Nordic Control Register
    Wait1: 0x7db,
    mask: MASK,
}

write_only_csr_field! {
    Wait1,
    /// data
    data: [0:15]
}

write_only_csr_field! {
    Wait1,
    /// If true, write `data` before waiting for cnt1 to reach 0
    write_data: 16
}
