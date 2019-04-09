//! Rust native FFI for the IPASIR interface for incremental SAT solvers.
//! 
//! Visit the IPASIR manual [here][manual].
//!
//! [manual]: http://www.cs.utexas.edu/users/moore/acl2/manuals/current/manual/index-seo.php/IPASIR____IPASIR

#[cfg(feature = "ffi")]
pub mod ffi;

mod types;
mod interface;

#[cfg(test)]
mod tests;

pub use self::{
    types::{
        Sign,
        Var,
        Lit,
        InvalidLitVal,
        Clause,
        LitIter,
    },
    interface::{
        SolverErrorKind,
        SolverError,
        SolveResponse,
        Result,
        ResponseError,
        LitValue,
        IpasirSolver,
        SolveControl,
    },
};
