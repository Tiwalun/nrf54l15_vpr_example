
build_app:
    cd app-core && cargo build --release
    rust-objcopy -O ihex app-core/target/thumbv8m.main-none-eabihf/release/rust-moonlight app-core.hex
    
flash_app: build_app
    probe-rs download app-core.hex --binary-format hex --chip nrf54l15

build_vpr:
    cd flpr-core && cargo build --release
    rust-objcopy -O ihex flpr-core/target/riscv32emc-unknown-none-elf/release/flpr-core flpr-core.hex
    
flash_vpr: build_vpr
    probe-rs download flpr-core.hex --binary-format hex --chip nrf54l15

reset: 
    probe-rs reset --chip nrf54l15

build: build_app build_vpr

flash: flash_app flash_vpr

run: flash_vpr build_app
    probe-rs run --chip nrf54l15 app-core/target/thumbv8m.main-none-eabihf/release/rust-moonlight 
