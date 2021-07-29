# bad64-sys

[![Build Status]][actions] [![Latest Version]][crates.io] [![Latest Docs]][docs.rs]

[Build Status]: https://img.shields.io/github/workflow/status/yrp604/bad64-sys/Rust
[actions]: https://github.com/yrp604/bad64-sys/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/bad64-sys.svg
[crates.io]: https://crates.io/crates/bad64-sys
[Latest Docs]: https://docs.rs/bad64-sys/badge.svg
[docs.rs]: https://docs.rs/bad64-sys


Bindgen Rust bindings for Binja's arm64 disassembler.

These are just the autogenerated sys bindings, you probably want
[bad64](http://github.com/yrp604/bad64).

## Building

The binding code is not included in the repository. It must be generated by 
manually running the `genbinding` tool included in the repository because doing
it automatically in `build.rs` (as `bindgen`'s user guide [recommends]) causes a
compilation slowdown and [an issue in Cargo]. To build the binding, do the
following:

```
cargo run -p genbinding
```

The result may vary between build environments, but this shouldn't impede the
uses on other platforms (hopefully).

After that, you can build and test the main package:

```
cargo test
```

[recommends]: https://rust-lang.github.io/rust-bindgen/library-usage.html
[an issue in Cargo]: https://github.com/rust-lang/cargo/issues/5237
