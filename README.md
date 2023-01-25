# Black Pill

## Preparation

Black Pill uses a STM32F411 which is an Arm Cortex M4 with FPU core. More info about the chip itself can be found in [Reference manual PDF](https://www.st.com/resource/en/reference_manual/dm00119316-stm32f411xc-e-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf).

In order to cross-compile, you'll need to add target used for this microcontroller thumbv7em-none-eabihf, using following command

```shell
rustup target add thumbv7em-none-eabihf
```

Install Probe-rs-debugger

```shell
cargo install --force  probe-rs-debugger

code --install-extension probe-rs-debugger-0.4.2.vsix
```

## Toolchain

- [Probe](https://probe.rs/)
- cargo-embed
- cargo-flash

### Usage

Basics

```shell
# In your cargo project directory, call
cargo flash --release --chip <chip_name>

# Don't know if your target is supported
# by cargo flash and what it's name is?
cargo flash --list-chips

# You can run your examples as usual with
cargo flash --example <your_example>
```
