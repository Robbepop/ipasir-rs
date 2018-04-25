use ipasir::*;

use std::os::raw::{
    c_int,
    c_void
};
use std::ffi::{
    CStr
};
use std::result;
use std::marker;
use std::mem;

/// The IPASIR result type.
pub type Result<T> = result::Result<T, Error>;

/// The incremental solver implementing the IPASIR interface.
#[derive(Debug, PartialEq, Eq)]
pub struct Solver(*mut c_void);

unsafe impl marker::Send for Solver {}
unsafe impl marker::Sync for Solver {}

/// A literal of the IPASIR implementing solver.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Lit(c_int);

/// An error that can be encountered while using the IPASIR interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Encountered an invalid literal value, i.e., zero.
    InvalidLitVal,
    /// Encountered an invalid solve result from `ipasir_solve`.
    InvalidSolveResult(c_int),
    /// Encountered an invalid value result from `ipasir_val`.
    InvalidValResult(c_int),
    /// Encountered an invalid failed result from `ipasir_failed`.
    InvalidFailedResult(c_int)
}

impl Lit {
    /// Creates a new `Lit` from the given `i32`.
    /// 
    /// # Errors
    /// 
    /// - If the given `i32` value is zero.
    pub fn new(var: i32) -> Result<Lit> {
        if var == 0 {
            return Err(Error::InvalidLitVal)
        }
        Ok(Lit(var as c_int))
    }

    /// Creates a new `Lit` from the given `i32`.
    /// 
    /// # Note
    /// 
    /// This does not check `var` for validity and
    /// thus should be used with care.
    pub unsafe fn new_unchecked(var: i32) -> Lit {
        Lit(var as c_int)
    /// Converts `self` to a raw C-style `int`.
    pub fn to_raw(self) -> c_int {
        self.0
    }
}

/// Represents a literal or the end of a clause.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LitOrEnd(c_int);

/// Represents the end of a clause.
/// 
/// Use to finalize a clause in `ipasir_add`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EndOfClause;

impl From<EndOfClause> for LitOrEnd {
    fn from(_: EndOfClause) -> Self {
        LitOrEnd(0)
    }
}

impl From<Lit> for LitOrEnd {
    fn from(lit: Lit) -> Self {
        LitOrEnd(lit.to_raw())
    }
}

impl LitOrEnd {
    /// Converts `self` into a C-style `int`.
    pub fn to_raw(self) -> c_int {
        self.0
    }
}

/// All viable return values of `ipasir_solve`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SolveResult {
    /// The solver found the input to be satisfiable.
    Sat = 10,
    /// The solver found the input to be unsatisfiable.
    Unsat = 20,
    /// The solver was interrupted.
    Interrupted = 0
}

/// The assignment of a given literal.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValResult {
    /// Any assignment is okay.
    DontCare,
    /// The literal is `true`.
    True,
    /// The literal is `false`.
    False
}

/// Represents a clause of the incremental solver.
#[derive(Debug, PartialEq, Eq)]
pub struct Clause(*mut c_int);

impl Clause {
    /// Returns the length of the clause.
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut cur = self.0;
        while unsafe{ *cur } != 0 {
            len += 1;
            cur = unsafe{ cur.offset(1) };
        }
        len
    }

    /// Returns the i-th literal of this clause.
    pub fn get(&self, i: usize) -> Option<Lit> {
        if i >= self.len() {
            return None
        }
        Some(unsafe{
            Lit::new_unchecked(*self.0.offset(i as isize))
        })
    }
}

impl From<*mut c_int> for Clause {
    fn from(lits: *mut c_int) -> Self {
        Clause(lits)
    }
}

impl Solver {
    /// Returns a short string representation of the incremental solver.
    pub fn signature() -> &'static str {
        let c_chars = unsafe{ ipasir_signature() };
        let c_str = unsafe{ CStr::from_ptr(c_chars) };
        c_str.to_str()
             .expect("The IPASIR implementation returned invalid UTF-8.")
    }

    /// Creates a new incremental solver instance.
    pub fn init() -> Solver {
        Solver(unsafe{ ipasir_init() })
    }
}

impl Solver {
    /// Returns a raw representation of this solver that is consumable by the IPASIR interface.
    fn raw_mut(&mut self) -> *mut c_void {
        self.0
    }
}

impl Solver {
    /// Adds the given literal to the current clause.
    pub fn add_lit(&mut self, lit: Lit) {
        unsafe{ ipasir_add(self.raw_mut(), lit.to_raw()) }
    }

    /// Finalizes the current clause.
    pub fn finalize_clause(&mut self) {
        unsafe{ ipasir_add(self.raw_mut(), 0) }
    }

    /// Adds the given literal as new assumption.
    pub fn assume(&mut self, lit: Lit) {
        unsafe{ ipasir_assume(self.raw_mut(), lit.to_raw()) }
    }

    /// Starts the solving process.
    pub fn solve(&mut self) -> Result<SolveResult> {
        match unsafe{ ipasir_solve(self.raw_mut()) } {
            0 => Ok(SolveResult::Interrupted),
            10 => Ok(SolveResult::Sat),
            20 => Ok(SolveResult::Unsat),
            invalid => Err(Error::InvalidSolveResult(invalid))
        }
    }

    /// Queries the assignment of the given literal.
    pub fn val(&mut self, lit: Lit) -> Result<ValResult> {
        match unsafe{ ipasir_val(self.raw_mut(), lit.to_raw()) } {
            0 => Ok(ValResult::DontCare),
            p if p == lit.to_raw() => Ok(ValResult::True),
            n if n == -lit.to_raw() => Ok(ValResult::False),
            invalid => Err(Error::InvalidValResult(invalid))
        }
    }

    /// Queries if the given literal was used to prove unsatisfiability.
    pub fn failed(&mut self, lit: Lit) -> Result<bool> {
        match unsafe{ ipasir_failed(self.raw_mut(), lit.to_raw()) } {
            0 => Ok(true),
            1 => Ok(false),
            invalid => Err(Error::InvalidFailedResult(invalid))
        }
    }
}

/// The raw callback for the C side of the IPASIR implementation of `ipasir_set_terminate`.
///
/// # Note
/// 
/// This simply forwards to the real user-provided implementation
/// of the user provided callback.
/// 
/// Don't use this directly!
#[no_mangle]
pub extern "C" fn ipasir_set_terminate_callback(cb: *mut c_void) -> c_int
{
    let cb = unsafe{
        mem::transmute::<*mut c_void, fn() -> SolveControl>(cb)
    };
    match cb() {
        SolveControl::Continue => 0,
        SolveControl::Stop => 1
    }
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

impl Solver {
    pub fn set_terminate(
        &mut self,
        callback: fn() -> SolveControl
    ) {
        unsafe{
            ipasir_set_terminate(
                self.raw_mut(),
                callback as *mut c_void,
                ipasir_set_terminate_callback
            )
        }
    }
}

/// The raw callback for the C side of the IPASIR implementation of `ipasir_set_learn`.
///
/// # Note
/// 
/// This simply forwards to the real user-provided implementation
/// of the user provided callback.
/// 
/// Don't use this directly!
#[no_mangle]
pub extern "C" fn ipasir_set_learn_callback(cb: *mut c_void, learnt_clause: *mut c_int)
{
    let cb = unsafe{
        mem::transmute::<*mut c_void, fn(Clause)>(cb)
    };
    let learnt_clause = Clause::from(learnt_clause);
    cb(learnt_clause)
}

impl Solver {
    pub fn set_learn(
        &mut self,
        max_length: usize,
        callback: fn(clause: Clause)
    ) {
        unsafe{
            ipasir_set_learn(
                self.raw_mut(),
                callback as *mut c_void,
                max_length as c_int,
                ipasir_set_learn_callback
            )
        }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe{ ipasir_release(self.raw_mut()) }
    }
}
