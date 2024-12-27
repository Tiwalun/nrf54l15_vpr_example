
## How to generate

First, get the SVD file from https://github.com/zephyrproject-rtos/hal_nordic/blob/master/nrfx/mdk/nrf54l15_flpr.svd.

Install svd2rust and form, then run:

```bash
svd2rust -i nrf54l15_flpr.svd  --target riscv
rm -rf src/
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
```

With svd2rust version 0.33.4, compilation fails because of an interrupt conversion
error. Edit the `src/interrupt.rs` file manually to fix.
