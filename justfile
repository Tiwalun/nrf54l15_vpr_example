app_folder := "app-core"
app_target := "thumbv8m.main-none-eabihf"

flpr_folder := "flpr-core"
flpr_target := "riscv32emc-unknown-none-elf"

app_elf :=  f"{{app_folder}}/target/{{app_target}}/release/nrf54l15-app"
flpr_elf :=  f"{{flpr_folder}}/target/{{flpr_target}}/release/flpr-core"

build_app:
    cd {{app_folder}} && cargo build --release

lint_app: (_lint "app" app_folder)

lint_vpr: (_lint "flpr" flpr_folder)

_lint name folder:
    cd {{folder}} && cargo clippy --release --message-format json | clippy-sarif | tee {{justfile_dir()}}/clippy_{{name}}.sarif.tmp | sarif-fmt
    cat clippy_{{name}}.sarif.tmp \
        | jq --arg pwd {{folder}} '.runs[].results[].locations[].physicalLocation.artifactLocation.uri |= $pwd + "/" + .' \
        | jq --arg pwd {{folder}} '.runs[].results[].relatedLocations[].physicalLocation.artifactLocation.uri |= $pwd + "/" + .' \
        > clippy_{{name}}.sarif

flash_app: build_app
    probe-rs download {{app_elf}} --binary-format elf --chip nrf54l15

build_vpr:
    cd {{flpr_folder}} && cargo build --release
    rust-objcopy -O ihex {{flpr_elf}} flpr-core.hex


flash_vpr: build_vpr
    probe-rs download flpr-core.hex --binary-format hex --chip nrf54l15

reset:
    probe-rs reset --chip nrf54l15

build: build_app build_vpr

zizmor:
    zizmor .

lint-ci: lint_app lint_vpr

lint-local: zizmor lint-ci

ci: build lint-ci

flash: flash_app flash_vpr

run: flash_vpr build_app
    probe-rs run --chip nrf54l15 {{app_elf}}
