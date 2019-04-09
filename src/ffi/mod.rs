//! IPASIR FFI solver and C bindings.

pub mod sys;
mod solver;

pub use self::solver::Solver;
