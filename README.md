# IPASIR interface for Rust

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
