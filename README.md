# `RISC-V-quickstart`

> A template for building applications for GD32VF103xx, RISCV32 based microcontrollers

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup default beta`

- The `cargo generate` subcommand. [Installation instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the RISCV32 targets. Run:

``` console
$ rustup target add riscv32imac-unknown-none-elf
```

## Using this template

**NOTE**: This is the very short version that only covers building programs. For the long version, which additionally covers flashing, running and debugging programs, check [the embedded Rust book][book].

[book]: https://rust-embedded.github.io/book

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/astepkowski/gd32vf103-rust-template
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. Set a default compilation targetin `.cargo/config`, e.g.:

``` toml
[build]
target = "riscv32imac-unknown-none-elf"

```

3. Enter the memory region information into the `memory.x` file.

``` console
$ cat memory.x
/* Linker script for the GD32VF103C8T6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 32K
}
```

4. Build the template application or one of the examples.

``` console
$ cargo build
```

# License

This template is licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT).
