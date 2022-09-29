# rustcube [![Build status](https://ci.appveyor.com/api/projects/status/ganyydat21is3coa/branch/master?svg=true)](https://ci.appveyor.com/project/msierks/rustcube/branch/master)

A Gamecube emulator in the Rust programming language. Work is in progress to boot the Gamecube BIOS.

[Documentation](http://msierks.github.io/rustcube/rustcube/)

## Screenshot
![screenshot](media/screenshot.png)

## Build and Run

Rustcube is built with [Cargo, the Rust package manager](https://www.rust-lang.org/).

Currently, Rustcube takes a single argument to run. This can be the Gamecube bios(IPL.bin), The one I've been testing with has a SHA-1 of `015808f637a984acde6a06efa7546e278293c6ee`. You could also use a [DOL](http://www.emulator-zone.com/doc.php/gamecube/) file.

You can build and run the emulator with:

```
cargo build --release && ./target/release/rustcube <PathToIPL/DOL>
```

You'll want to use the `--release` flag to turn optimizations on, otherwise it will run slowly. 

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any 
additional terms or conditions.
