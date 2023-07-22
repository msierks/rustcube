# rustcube

A Gamecube emulator in the Rust programming language. Work is in progress to boot the Gamecube BIOS.

## Build and Run

Rustcube is built with [Cargo, the Rust package manager](https://www.rust-lang.org/).

Currently, Rustcube takes a single argument to run. This can be the Gamecube bios(IPL.bin), The one I've been testing with has a SHA-1 of `015808f637a984acde6a06efa7546e278293c6ee`. You could also run DOL, ISO and GCM files.

You can build and run the emulator with:

```
cargo run -- <PathToIPL/DOL/ISO/GCM>
```

Enable debug logging

```
RUST_LOG=debug cargo run -- <PathToIPL/DOL/ISO/GCM>
```

## Debugging

A basic debugger has been created with gtk-rs. Though it is very much a work in progress, which means it's missing many features and may not function correctly.

Run the debugger with following:
```
cargo run -p debugger -- <PathToIPL/DOL/ISO/GCM>
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
