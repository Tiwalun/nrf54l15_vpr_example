app_folder := "app-core"
flpr_folder := "flpr-core"

app_elf :=  f"{{app_folder}}/target/thumbv8m.main-none-eabihf/release/nrf54l15-app"
flpr_elf :=  f"{{flpr_folder}}/target/riscv32emc-unknown-none-elf/release/flpr-core"

build_app:
    cd {{app_folder}} && cargo build --release

lint_app:
    cd {{app_folder}} && cargo clippy --release

flash_app: build_app
    probe-rs download {{app_elf}} --binary-format elf --chip nrf54l15

build_vpr:
    cd {{flpr_folder}} && cargo build --release
    rust-objcopy -O ihex {{flpr_elf}} flpr-core.hex

lint_vpr:
    cd {{flpr_folder}} && cargo clippy --release

flash_vpr: build_vpr
    probe-rs download flpr-core.hex --binary-format hex --chip nrf54l15

reset:
    probe-rs reset --chip nrf54l15

build: build_app build_vpr

zizmor:
    zizmor .

lint: lint_app lint_vpr zizmor
    cd flpr-core && cargo clippy --release


ci: build lint

flash: flash_app flash_vpr

run: flash_vpr build_app
    probe-rs run --chip nrf54l15 {{app_elf}}
