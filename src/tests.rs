use crate::{
    Lit,
    Clause,
    IpasirSolver,
    SolveResponse,
    Result,
    LitValue,
    SolveControl,
};
use std::convert::{
    TryFrom,
};

/// The solver state of the test solver.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SolverState {
    /// The input phase.
    Input,
    /// After evaluating to SAT.
    Sat,
    /// After evaluating to UNSAT.
    Unsat,
}

/// The test solver implementing the IPASIR interface.
struct TestSolver {
    /// The current solver state.
    state: SolverState,
    /// All clauses.
    clauses: Vec<OwnedClause>,
    /// All assumption literals.
    assumptions: Vec<Lit>,
}

impl TestSolver {
    /// Returns the current solver state.
    pub fn solver_state(&self) -> SolverState {
        self.state
    }

    /// Returns an iterator over all clauses.
    pub fn clauses(&self) -> impl Iterator<Item = &OwnedClause> {
        self.clauses.iter()
    }
}

/// A clause that owns its literals.
struct OwnedClause {
    /// The literals of the clause.
    lits: Vec<Lit>,
}

impl<'a> From<Clause<'a>> for OwnedClause {
    fn from(clause: Clause<'a>) -> Self {
        Self {
            lits: clause.iter().collect(),
        }
    }
}

impl<I, L> From<I> for OwnedClause
where
    I: IntoIterator<Item = L>,
    L: Into<Lit>,
{
    fn from(lits: I) -> Self {
        Self {
            lits: lits.into_iter().map(Into::into).collect()
        }
    }
}

impl Default for TestSolver {
    fn default() -> Self {
        Self {
            state: SolverState::Input,
            clauses: Vec::new(),
            assumptions: Vec::new(),
        }
    }
}

impl IpasirSolver for TestSolver {
    fn signature(&self) -> &'static str {
        "TestSolver"
    }

    fn init() -> Self {
        Self::default()
    }

    fn add_clause<I, L>(&mut self, lits: I)
    where
        I: IntoIterator<Item = L>,
        L: Into<Lit>,
    {
        self.clauses.push(OwnedClause::from(lits))
    }

    fn assume(&mut self, lit: Lit) {
        self.assumptions.push(lit)
    }

    fn solve(&mut self) -> Result<SolveResponse> {
        self.state = SolverState::Sat;
        Ok(SolveResponse::Sat)
    }

    fn val(&mut self, _lit: Lit) -> Result<LitValue> {
        Ok(LitValue::DontCare)
    }

    fn failed(&mut self, _lit: Lit) -> Result<bool> {
        Ok(false)
    }

    fn set_terminate<F>(&mut self, _callback: F)
    where
        F: FnMut() -> SolveControl + 'static
    {}

    fn set_learn<F>(&mut self, _max_len: usize, _callback: F)
    where
        F: FnMut(Clause) + 'static
    {}
}

#[test]
fn state_after_init() {
    assert_eq!(TestSolver::init().solver_state(), SolverState::Input)
}

#[test]
fn signature() {
    assert_eq!(TestSolver::init().signature(), "TestSolver")
}

#[test]
fn add_clause() {
    let mut solver = TestSolver::init();
    assert_eq!(solver.clauses().count(), 0);
    solver.add_clause(
        [1, 2, 3].iter().map(|val| Lit::try_from(*val).unwrap()));
    assert_eq!(solver.clauses().count(), 1);
}
