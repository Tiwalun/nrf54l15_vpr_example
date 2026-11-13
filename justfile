app_folder := "app-core"
app_target := "thumbv8m.main-none-eabihf"
app_elf :=  f"{{app_folder}}/target/{{app_target}}/release/nrf54l15-app"

flpr_folder := "flpr-core"
flpr_target := "riscv32emc-unknown-none-elf"
flpr_elf :=  f"{{flpr_folder}}/target/{{flpr_target}}/release/flpr-core"

target_dir := f"{{justfile_dir()}}/target"

targetdir:
    mkdir -p {{target_dir}}

build_app: build_vpr
    cd {{app_folder}} && cargo build --release

lint_app: (_lint "app" app_folder)

lint_vpr: (_lint "flpr" flpr_folder)

_lint name folder: targetdir
    cd {{folder}} && cargo clippy --release --message-format json | clippy-sarif | tee {{target_dir}}/clippy_{{name}}.sarif.tmp | sarif-fmt
    jq --arg pwd {{folder}} \
        '(.runs[].results[].locations[]?, .runs[].results[].relatedLocations[]?).physicalLocation.artifactLocation.uri |= $pwd + "/" + .' \
        {{target_dir}}/clippy_{{name}}.sarif.tmp \
        > {{target_dir}}/clippy_{{name}}.sarif

flash_app: build_app
    probe-rs download {{app_elf}} --binary-format elf --chip nrf54l15

build_vpr: targetdir
    cd {{flpr_folder}} && cargo build --release
    rust-objcopy -O binary {{flpr_elf}} {{app_folder}}/input/flpr-core.bin



reset:
    probe-rs reset --chip nrf54l15

build: build_app build_vpr

zizmor:
    zizmor .

lint-ci: lint_app lint_vpr

lint-local: zizmor lint-ci

ci: build lint-ci

flash: flash_app

run: build_app
    probe-rs run --chip nrf54l15 {{app_elf}}
