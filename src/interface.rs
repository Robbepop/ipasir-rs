use crate::{
    Lit,
    InvalidLitVal,
    Clause,
};
use std::{
    error::Error,
    fmt,
    os::raw::c_int,
    result::Result as StdResult,
};

/// An error encountered when using an FII IPASIR solver
/// that returned an invalid response value.
///
/// # Note
///
/// This can only be encountered when working with an FFI IPASIR solver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    /// The `solve` call returned an invalid response.
    Solve(c_int),
    /// The `val` call returned an invalid response.
    Val(c_int),
    /// The `failed` call returned an invalid response.
    Failed(c_int),
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResponseError::Solve(invalid) => write!(f, "invalid response {} from solve", invalid),
            ResponseError::Val(invalid) => write!(f, "invalid response {} from val", invalid),
            ResponseError::Failed(invalid) => write!(f, "invalid response {} from failed", invalid),
        }
    }
}

impl Error for ResponseError {}

/// A kind of a SAT solver error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverErrorKind {
    /// A literal value was invalid.
    Lit(InvalidLitVal),
    /// Encountered when calling an invalid FFI IPASIR solver.
    Response(ResponseError),
    /// Returned when a solver was called in an invalid solver state.
    ///
    /// # Note
    ///
    /// This cannot be communicated by C ffi SAT solvers.
    InvalidSolverState,
}

impl fmt::Display for SolverErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolverErrorKind::Lit(e) => e.fmt(f),
            SolverErrorKind::Response(e) => e.fmt(f),
            SolverErrorKind::InvalidSolverState => write!(f, "invalid solver state"),
        }
    }
}

/// An error encountered at some solver calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolverError {
    /// The kind of the solver error.
    kind: SolverErrorKind,
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Error for SolverError {}

impl SolverError {
    /// Returns the kind of the error.
    pub fn kind(&self) -> &SolverErrorKind {
        &self.kind
    }
}

impl From<InvalidLitVal> for SolverError {
    fn from(err: InvalidLitVal) -> Self {
        Self {
            kind: SolverErrorKind::Lit(err)
        }
    }
}

impl From<ResponseError> for SolverError {
    fn from(err: ResponseError) -> Self {
        Self {
            kind: SolverErrorKind::Response(err)
        }
    }
}

/// Type alias that has a `SolverError` as error variant.
pub type Result<T> = StdResult<T, SolverError>;

/// Possible responses from a call to `ipasir_solve`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SolveResponse {
    /// The solver found the input to be satisfiable.
    Sat = 10,
    /// The solver found the input to be unsatisfiable.
    Unsat = 20,
    /// The solver was interrupted.
    Interrupted = 0
}

/// The assignment of a literal.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LitValue {
    /// Any assignment is okay.
    DontCare,
    /// The literal is `true`.
    True,
    /// The literal is `false`.
    False
}

/// The IPASIR interface a SAT solver has to implement to be conforming.
pub trait IpasirSolver {
    /// Returns name and version of the incremental SAT solving implementation.
    fn signature(&self) -> &'static str;

    /// Return a new incremental SAT solver.
    ///
    /// # States
    ///
    /// - **Required:** N/A
    /// - **After:** INPUT
    fn init() -> Self;

    /// Adds a clause to the solver.
    ///
    /// The clause is defined to contain all yielded literals of the given iterator.
    ///
    /// # Note
    ///
    /// - Clauses added this way cannot be removed.
    /// - The addition of removable clauses can be simulated using
    ///   activation literals and assumptions.
    ///
    /// # States
    ///
    /// - **Required:** any
    /// - **After:** INPUT
    fn add_clause<I, L>(&mut self, lits: I)
    where
        I: IntoIterator<Item = L>,
        L: Into<Lit>;

    /// Adds the given literal as new assumption.
    ///
    /// # States
    ///
    /// - **Required:** any
    /// - **After:** INPUT
    fn assume(&mut self, lit: Lit);

    /// Starts the solving process.
    ///
    /// # States
    ///
    /// - **Required:** any
    /// - **After:** any
    fn solve(&mut self) -> Result<SolveResponse>;

    /// Queries the assignment of the given literal.
    ///
    /// # States
    ///
    /// - **Required:** SAT
    /// - **After:** SAT
    fn val(&mut self, lit: Lit) -> Result<LitValue>;

    /// Queries if the given literal was used to prove unsatisfiability.
    ///
    /// # States
    ///
    /// - **Required:** UNSAT
    /// - **After:** UNSAT
    fn failed(&mut self, lit: Lit) -> Result<bool>;

    /// Set a callback handler used to indicate a terminate requirement to the solver.
    ///
    /// # Note
    ///
    /// The solver will periodically query this handler and check its return value during solving.
    ///
    /// # States
    ///
    /// - **Required:** any
    /// - **After:** same
    fn set_terminate<F>(&mut self, callback: F)
    where
        F: FnMut() -> SolveControl + 'static;

    /// Set a callback function used to extract learned clauses up to a given length from the solver.
    ///
    /// # Note
    ///
    /// - The solver will call this function for each learned clause that is not longer than the maximum length.
    /// - The solver calls the callback function with the parameter `state` that was passed into `set_learn`.
    ///
    /// # States
    ///
    /// - **Required:** any
    /// - **After:** same
    fn set_learn<F>(&mut self, max_len: usize, callback: F)
    where
        F: FnMut(Clause) + 'static;
}

/// Tells the solver to either stop solving process or continue.
/// 
/// # Note
/// 
/// Use this as return type of the `Solver::set_terminate` callback.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SolveControl {
    /// Continue with the solving process.
    Continue = 0,
    /// Stop the solving process.
    Stop = 1
}
