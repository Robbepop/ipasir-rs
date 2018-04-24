use std::os::raw::{
    c_char,
    c_int,
    c_void
};

extern "C" {
    /// Return the name and the version of the incremental SAT solving library.
    pub fn ipasir_signature() -> *const c_char;

    /// Construct a new solver and return a pointer to it.
    /// 
    /// # Note
    /// 
    /// Use the returned pointer as the first parameter in each
    /// of the following functions.
    /// 
    /// # States
    /// 
    /// - Required state: *N/A*
    /// - State after: `INPUT`
    pub fn ipasir_init() -> *mut c_void;

    /// Release the solver, i.e., all its resources and
    /// allocated memory (runs destructors).
    /// 
    /// # Note
    /// 
    /// The solver pointer must not be used for any purposes
    /// after this call.
    /// 
    /// # States
    /// 
    /// - Required state: `INPUT` or `SAT` or `UNSAT`
    /// - State after: **undefined**
    pub fn ipasir_release(solver: *mut c_void);

    /// Add the given literal into the currently added clause
    /// of finalize the clause with a 0 (zero).
    /// 
    /// # Note
    /// 
    /// Clauses added this way cannot be removed.
    /// 
    /// The addition of removable clauses can be simulated
    /// using activation literals and assumptions.
    /// 
    /// # Encoding
    /// 
    /// Literals are encoded as (non-zero) integers as in the
    /// [DIMACS][dimacs-fmt] formats.  They have to be smaller 
    /// or equal to `INT_MAX` and strictly larger than `INT_MIN`
    /// (to avoid negation overflow).  This applies to all the
    /// literal arguments in API functions.
    /// 
    /// [dimacs-fmt]: http://www.satcompetition.org/2009/format-benchmarks2009.html
    /// 
    /// # States
    /// 
    /// - Required state: `INPUT` or `SAT` or `UNSAT`
    /// - State after: `INPUT`
    pub fn ipasir_add(solver: *mut c_void, lit_or_zero: c_int);

    /// Add an assumption for the next SAT search (the next call
    /// to `ipasir_solve`).
    /// 
    /// # Note
    /// 
    /// After calling `ipasir_solve` all the previously added assumptions are cleared.
    /// 
    /// # States
    /// 
    /// - Required state: `INPUT` or `SAT` or `UNSAT`
    /// - State after: `INPUT`
    pub fn ipasir_assume(solver: *mut c_void, lit: c_int);

    /// Solve the formula with specified clauses under the specified assumptions.
    /// 
    /// # States
    /// 
    /// - If the formula is satisfiable the function returns `10`
    ///   and the state of the solver is changed to `SAT`.
    /// - If the formula is unsatisfiable the function returns `20`
    ///   and the state of the solver is changed to `UNSAT`.
    /// - If the search is interrupted (see `ipasir_set_terminate`) the function returns `0`
    ///   and the state of the solver remains `INPUT`.
    /// 
    /// This function can be called in any defined state of the solver.
    /// 
    /// - Required state: `INPUT` or `SAT` or `UNSAT`
    /// - State after: `INPUT` or `SAT` or `UNSAT`
    pub fn ipasir_solve(solver: *mut c_void) -> c_int;

    /// Get the truth value of the given literal in the found satisfying assignment.
    /// 
    /// # Return
    /// 
    /// Returns `lit` if `true`, `-lit` if `false`, and `0` if not important (don't-care).
    /// 
    /// # Note
    /// 
    /// This function can only be used if `ipasir_solve` has returned `10` and no
    /// `ipasir_add` nor `ipasir_assume` has been called since then, i.e., the state
    /// of the solver is `SAT`.
    /// 
    /// # States
    /// 
    /// - Required state: `SAT`
    /// - State after: `SAT`
    pub fn ipasir_val(solver: *mut c_void, lit: c_int) -> c_int;

    /// Check if the given assumption literal was used to prove the
    /// unsatisfiability of the formula under the assumptions
    /// used for the last SAT search.  Return `1` if so, `0` otherwise.
    /// 
    /// # Note
    /// 
    /// This function can only be used if `ipasir_solve` has returned `20` and
    /// no `ipasir_add` or `ipasir_assume` has been called since then, i.e.,
    /// the state of the solver is `UNSAT`.
    /// 
    /// # States
    /// 
    /// - Required state: `UNSAT`
    /// - State after: `UNSAT`
    pub fn ipasir_failed(solver: *mut c_void, lit: c_int) -> c_int;

    /// Set a callback function used to indicate a termination requirement to the solver.
    /// The solver will periodically call this function and check its return value during
    /// the search.
    /// 
    /// # Note
    /// 
    /// The `ipasir_set_terminate` function can be called in any state of the solver,
    /// the state remains unchanged after the call.
    /// 
    /// # Callback
    /// 
    /// The callback function is of the form `fn(state: *mut c_void) -> c_int` and
    /// 
    ///   - it returns a non-zero value if the solver should terminate.
    ///   - the solver calls the callback function with the parameter `state`
    ///     having the value passed in the second parameter of the `ipasir_set_terminate`
    ///     function.
    /// 
    /// # States
    /// 
    /// - Required state: `INPUT` or `SAT` or `UNSAT`
    /// - State after: `INPUT` or `SAT` or `UNSAT`
    pub fn ipasir_set_terminate(
        solver: *mut c_void,
        state: *mut c_void,
        terminate: extern fn(state: *mut c_void) -> c_int
    );

    /// Set a callback function used to extract learned clauses up to a given length from the solver.
    /// The solver will call this function for each learned clause that satisfies the maximum length
    /// (literal count) condition.
    /// 
    /// # Note
    /// 
    /// The `ipsair_set_learn` function can be called in any state of the
    /// solver, the state remains unchanged after the call.
    /// 
    /// # Callback
    /// 
    /// The callback function is of the form `fn(state: *mut c_void, clause: *mut c_int)` and
    ///   - the solver calls the callback function with the parameter `state`
    ///     having the value passed in the second parameter of the `ipasir_set_terminate` function
    ///   - the `clause` argument is a pointer to a null terminated integer array containing the learned clause.
    ///     The solver can change the data at the memory location that `clause` points to after the
    ///     function call.
    /// 
    /// # States
    /// 
    /// - Required state: `INPUT` or `SAT` or `UNSAT`
    /// - State after: `INPUT` or `SAT` or `UNSAT`
    pub fn ipasir_set_learn(
        solver: *mut c_void,
        state: *mut c_void,
        max_length: c_int,
        learn: extern fn(state: *mut c_void, clause: *mut c_int)
    );
}
