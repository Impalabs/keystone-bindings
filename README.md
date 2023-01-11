<p align="center">
  <b style="font-size: 2em">KEYSTONE-ENGINE BINDINGS</b>
  <br/>
  <span style="font-size: 1.5em">Unofficial Rust bindings for the Keystone Engine</b>
</p>

<hr/>

<p align="center">
  <img src="https://img.shields.io/github/license/impalabs/keystone-bindings?style=for-the-badge&color=ff9901" alt="shields.io license" />
  <img src="https://img.shields.io/github/v/release/impalabs/keystone-bindings?style=for-the-badge&color=f38702" alt="shields.io version" />
  <a href="https://docs.rs/hyperpom"><img src="https://img.shields.io/badge/keystone-v0.9.2-e77600?style=for-the-badge" alt="shields.io supported keystone version" /></a>
  <br/>
  <a href="https://crates.io/crates/hyperpom"><img src="https://img.shields.io/crates/v/hyperpom?color=cd5300&style=for-the-badge" alt="shields.io crates.io" /></a>
  <a href="https://docs.rs/hyperpom"><img src="https://img.shields.io/badge/docs.rs-rustdoc-bf4200?style=for-the-badge" alt="shields.io crates.io" /></a>
</p>

<hr/>

These Rust bindings are an alternative to the ones available in the [official repository](https://github.com/keystone-engine/keystone/tree/master/bindings/rust) and support version 0.9.2 of the Keystone Engine.

## Why Release New Bindings If Official Ones Exist?

To publish a crate on crates.io, all its dependencies must also come from crates.io. There is already a [crate](https://crates.io/crates/keystone) providing bindings for Keystone. However, the latest version it supports, is version 0.9.0 of Keystone. This version has bugs, which, fortunately, are fixed in version 0.9.2. For this reason, and because the current crate of Keystone has not been updated for years, these bindings can be used instead.

These bindings are heavily inspired from the official ones and do not alter the API too much compared to the original.

## Example

This example, taken from the [official repo](https://github.com/keystone-engine/keystone/blob/0.9.2/bindings/rust/examples/asm.rs), should work out of the box.

Add the following dependency to `Cargo.toml`:

```toml
keystone-engine = { version = "0.1.0", features = ["build-from-src"] }
```

**Note:** You can use either `build-from-src` to build the Keystone Engine or `use-system-lib` if you have already installed the Keystone library on your system.

You should now be able to run the following code:

```rust
use keystone_engine::*;

fn main() {
    let engine =
        Keystone::new(Arch::X86, Mode::MODE_32).expect("Could not initialize Keystone engine");

    engine
        .option(OptionType::SYNTAX, OptionValue::SYNTAX_NASM)
        .expect("Could not set option to nasm syntax");

    let result = engine
        .asm("mov ah, 0x80".to_string(), 0)
        .expect("Could not assemble");

    println!("ASM result: {}", result);

    if let Err(err) = engine.asm("INVALID".to_string(), 0) {
        println!("Error: {}", err);
    }
}
```

## Credits

 * [Keystone Assembler Engine](http://www.keystone-engine.org/) by Nguyen Anh Quynh <aquynh@gmail.com>
 * [Rust bindings](https://github.com/keystone-engine/keystone/tree/master/bindings/rust) by Remco Verhoef <remco@dutchcoders.io>
