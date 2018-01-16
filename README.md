# ghc-rts-rs

This is code used to link in the GHC RTS in statically to make it possible to
run Haskell inside Rust. This is intended to be used as part of
[curryrs][curryrs] package as a dependency. However, it's functionality is being
separated out as a base library that others can use on their own. You choose
which version of GHC you want to use and the modified source files here will
compile an optimized build with the correct version and DWARF symbols turned on.

It also provides wrappers around the `hs_init` and `hs_exit` functions so that
you can easily start and stop the runtime from your code.

Beyond this functionality the library does nothing else in order to make FFI
nicer to use or simplified. If you are looking for a higher level library
to make Rust/Haskell FFI easier then take a look at [curryrs][curryrs].

# Build Time
This takes an incredibly long time to build unfortunately. I would highly
recommend not changing to newer versions of Rust often, meaning use stable
if possible as that only changes every 6 weeks. Once compiled you shouldn't need
to touch it again unless your Rust or GHC compiler changes.

# Start/Stop the Runtime

```rust
extern crate ghc_rts;
use ghc_rts::{start, stop};

fn main() {
  start(); // Starts the Haskell Runtime. Can be called more than once
  stop();  // Stops the Runtime. Will panic if called more than once as the RTS
           // can't be started again now, nor stopped.
}
```

# Dependencies

You'll need the following tools installed to be able to compile the RTS:
- `libnuma` - Maybe. It's needed on my Linux machine. Still trying to figure
  that out!
- `cabal`
- `ghc`
- [Tools needed to build GHC][ghc]

You might need the static versions in order to work. For instance I needed the
`ghc-static` package on Arch Linux. Play around with it to figure it out!

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[curryrs]: https://www.github.com/mgattozzi/curryrs
[ghc]: https://ghc.haskell.org/trac/ghc/wiki/Building/Preparation/Tools

