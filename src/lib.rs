/// Rust native FFI for the IPASIR interface for incremental SAT solvers.
/// 
/// Manual: http://www.cs.utexas.edu/users/moore/acl2/manuals/current/manual/index-seo.php/IPASIR____IPASIR

mod ipasir;
mod wrapper;

/// The raw IPASIR functions.
/// 
/// Prefer using the provided safe API instead.
pub mod sys {
    pub use super::ipasir::{
        ipasir_signature,
        ipasir_init,
        ipasir_release,
        ipasir_add,
        ipasir_assume,
        ipasir_solve,
        ipasir_val,
        ipasir_failed,
        ipasir_set_terminate,
        ipasir_set_learn
    };
}

/// The forwarding callbacks for IPASIR.
pub mod callbacks {
    pub use super::wrapper::{
        ipasir_set_learn_callback,
        ipasir_set_terminate_callback
    };
}

pub use self::wrapper::{
    Solver,
    Lit,
    EndOfClause,
    LitOrEnd,
    SolveResult,
    ValResult,
    Clause,
    Error,
    Result
};
