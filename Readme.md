# `air001-quickstart`

> A template for building applications for Air001 microcontrollers

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi
```

## Using this template

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/air001-rs/air001-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. Build the template application or one of the examples.

``` console
$ cargo build --release
$ cargo build --release --example blinky
```

## Flashing

Use PyOCD (> 0.36) and DAPLink to flash the program.

``` console
$ pyocd load --target air001 -f 100khz --format elf target/thumbv6m-none-eabi/release/examples/blinky
```
