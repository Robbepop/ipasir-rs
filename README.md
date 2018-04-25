# IPASIR interface for Rust

|       Docs        |       Crates.io      |
|:-----------------:|:--------------------:|
| [![docs][1]][2]   | [![crates][3]][4]    |

## What it is

IPASIR is a simple C interface to incremental SAT solvers.
(It stands for Reentrant Incremental Sat solver API, in reverse.)
This interface is supported by a few different solvers because it is used in the SAT competition's incremental track.
The IPASIR distribution, containing the interface and some sample solvers,
can be found at [this GitHub repository](https://github.com/biotomas/ipasir).
This IPASIR library is an attempt to semi-soundly allow Rust programs to interface with such SAT solver libraries.

## How the FFI is structured

For users of this FFI there are two distinct ways of usage.
Users can build their application on top of the `raw` module that offers direct but unsafe calls
into the C-API.
The recommended way to use this FFI is to use the `Solver` type that acts as safe wrapper around
the C-API.

Allocate a new solver instance with: `ipasir::Solver::init()`

## License

Licensed under either of

 * Apache license, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Dual licence: [![badge][license-mit-badge]](LICENSE-MIT) [![badge][license-apache-badge]](LICENSE-APACHE)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[license-mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license-apache-badge]: https://img.shields.io/badge/license-APACHE-orange.svg

## Release Notes

### 0.2.0 - 25th April 2018

- Renamed `raw` module to `sys` to better fit with the rest of the ecosystem.
- `Lit::to_raw` is no longer publicly visible.
- Removed `LitOrEnd` and `EndOfClause`.
- Split `Solver::add` API into `Solver::add_lit` and `Solver::finalize_clause`.
- Add `Clause::len` and `Clause::get` methods.
- Add `Lit::new_unchecked` constructor.
- Make `SolveControl` now publicly visible. (Was accidentally private before.)

### 0.1.0 - 24th April 2018

- Initial Release

[1]: https://docs.rs/ipasir/badge.svg
[2]: https://docs.rs/ipasir/
[3]: https://img.shields.io/crates/v/ipasir.svg
[4]: https://crates.io/crates/ipasir/
