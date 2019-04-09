pub use crate::{
    ffi::sys::*,

    Result,

    Sign,
    Var,
    Lit,
    InvalidLitVal,
    Clause,
    LitIter,

    SolverErrorKind,
    SolverError,
    SolveResponse,
    ResponseError,
    LitValue,
    IpasirSolver,
    SolveControl,
};
use std::{
    os::raw::{
        c_int,
        c_void,
    },
    ffi::CStr,
    marker,
    mem,
};

/// The incremental solver implementing the IPASIR interface.
pub struct Solver {
    ptr: *mut SysSolver,
    terminate_cb: Option<Box<Box<FnMut() -> SolveControl>>>,
    learn_cb: Option<Box<Box<FnMut(Clause)>>>,
}

unsafe impl marker::Send for Solver {}
unsafe impl marker::Sync for Solver {}

impl Solver {
    /// Returns a raw representation of this solver that is consumable by the IPASIR interface.
    fn raw_mut(&mut self) -> *mut SysSolver {
        self.ptr
    }
}

impl IpasirSolver for Solver {
    fn signature(&self) -> &'static str {
        let c_chars = unsafe{ ipasir_signature() };
        let c_str = unsafe{ CStr::from_ptr(c_chars) };
        c_str.to_str()
             .expect("The IPASIR implementation returned invalid UTF-8.")
    }

    fn init() -> Solver {
        Solver {
            ptr: unsafe{ ipasir_init() },
            terminate_cb: None,
            learn_cb: None,
        }
    }

    fn add_clause<I, L>(&mut self, lits: I)
    where
        I: IntoIterator<Item = L>,
        L: Into<Lit>,
    {
        for lit in lits.into_iter() {
            unsafe { ipasir_add(self.raw_mut(), lit.into().to_raw()) }
        }
        unsafe { ipasir_add(self.raw_mut(), 0) }
    }

    fn assume(&mut self, lit: Lit) {
        unsafe{ ipasir_assume(self.raw_mut(), lit.to_raw()) }
    }

    fn solve(&mut self) -> Result<SolveResponse> {
        match unsafe{ ipasir_solve(self.raw_mut()) } {
            0 => Ok(SolveResponse::Interrupted),
            10 => Ok(SolveResponse::Sat),
            20 => Ok(SolveResponse::Unsat),
            invalid => Err(ResponseError::Solve(invalid).into())
        }
    }

    fn val(&mut self, lit: Lit) -> Result<LitValue> {
        match unsafe{ ipasir_val(self.raw_mut(), lit.to_raw()) } {
            0 => Ok(LitValue::DontCare),
            p if p == lit.to_raw() => Ok(LitValue::True),
            n if n == -lit.to_raw() => Ok(LitValue::False),
            invalid => Err(InvalidLitVal(invalid).into())
        }
    }

    fn failed(&mut self, lit: Lit) -> Result<bool> {
        match unsafe{ ipasir_failed(self.raw_mut(), lit.to_raw()) } {
            0 => Ok(true),
            1 => Ok(false),
            invalid => Err(ResponseError::Failed(invalid).into())
        }
    }

    fn set_terminate<F>(&mut self, cb: F)
    where
        F: FnMut() -> SolveControl + 'static,
    {
        self.terminate_cb = Some(Box::new(Box::new(cb)));
        unsafe {
            ipasir_set_terminate(
                self.raw_mut(),
                self.terminate_cb.as_mut().unwrap().as_mut() as *const _ as *const c_void,
                ipasir_set_terminate_callback
            )
        }
    }

    fn set_learn<F>(&mut self, max_len: usize, cb: F)
    where
        F: FnMut(Clause) + 'static
    {
        self.learn_cb = Some(Box::new(Box::new(cb)));
        unsafe {
            ipasir_set_learn(
                self.raw_mut(),
                self.learn_cb.as_mut().unwrap().as_mut() as *const _ as *const c_void,
                max_len as c_int,
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

/// The raw callback for the C side of the IPASIR implementation of `ipasir_set_terminate`.
///
/// # Note
/// 
/// This simply forwards to the real user-provided implementation
/// of the user provided callback.
/// 
/// Don't use this directly!
extern "C" fn ipasir_set_terminate_callback(state: *const c_void) -> c_int
{
    let cb: &mut Box<FnMut() -> SolveControl> = unsafe {
        mem::transmute(state)
    };
    match cb() {
        SolveControl::Continue => 0,
        SolveControl::Stop => 1
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
extern "C" fn ipasir_set_learn_callback(state: *const c_void, learnt_clause: *const c_int)
{
    let cb: &mut Box<FnMut(Clause)> = unsafe {
        mem::transmute(state)
    };
    let mut count_lits = 0;
    for n in 0.. {
        if unsafe { *learnt_clause.offset(n) } != 0 {
            count_lits += 1;
        }
    }
    let lits_slice = unsafe {
        std::mem::transmute::<&[c_int], &[Lit]>(
            std::slice::from_raw_parts(learnt_clause, count_lits))
    };
    cb(Clause::from(lits_slice))
}
