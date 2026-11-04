//! Puts `device.x` on the linker search path. The `riscv-rt` linker script (`link.x`) includes it
//! because this crate enables the `device` feature of `riscv-rt`.

use std::{env, fs, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("device.x"), include_bytes!("device.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=device.x");
}
