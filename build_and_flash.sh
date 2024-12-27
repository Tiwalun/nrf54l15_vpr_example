#!/usr/bin/env bash
set -e

APP_CORE_DIR=app-core
FLPR_CORE_DIR=flpr-core

pushd $APP_CORE_DIR
cargo build --release
popd

pushd $FLPR_CORE_DIR
cargo build --release
popd

# To use rust-objcopy, the cargo-binutils tools need to be installed.

rust-objcopy -O ihex $APP_CORE_DIR/target/thumbv8m.main-none-eabihf/release/rust-moonlight app-core.hex
nrfutil device program --firmware app-core.hex --serial-number "$1" --options verify=VERIFY_READ

#FLPR_ELF="$FLPR_CORE_DIR/target/riscv32emc-unknown-none-elf/release/flpr-core"

#RELOC_OFFSET="0x1FFF0000"
#
## Temporary hack to relocation sections into RRAM
#objcopy --change-section-lma .text-$RELOC_OFFSET $FLPR_ELF
#objcopy --change-section-lma .text.dummy-$RELOC_OFFSET $FLPR_ELF
#objcopy --change-section-lma .rodata-$RELOC_OFFSET $FLPR_ELF

rust-objcopy -O ihex $FLPR_CORE_DIR/target/riscv32emc-unknown-none-elf/release/flpr-core flpr-core.hex
nrfutil device program --firmware flpr-core.hex --serial-number "$1" --options chip_erase_mode=ERASE_RANGES_TOUCHED_BY_FIRMWARE,verify=VERIFY_READ
nrfutil device fw-verify --firmware app-core.hex --serial-number "$1"

nrfutil device reset --reset-kind RESET_PIN --serial-number "$1"
