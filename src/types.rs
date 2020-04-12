use std::{
    os::raw::c_int,
    convert::TryFrom,
    error::Error,
    fmt,
    result::Result as StdResult,
};

/// A variable of the IPASIR implementing solver.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Var(c_int);

impl Var {
    /// Returns the underlying `c_int` representation of `self`
    pub fn to_raw(self) -> c_int {
        self.0
    }
}

/// A literal of the IPASIR implementing solver.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Lit(c_int);

/// Encountered when trying to create a literal with an invalid value.
///
/// # Note
///
/// Invalid values are the following
/// 
/// - `0`: Because it cannot have positive or negative polarity
///        and because it is explicitely forbidden by the IPASIR specification
/// - `INT_MIN`: Because it cannot have positive polarity because
///              `-INT_MIN == INT_MIN`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidLitVal(pub c_int);

impl fmt::Display for InvalidLitVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid literal value {}", self.0)
    }
}

impl Error for InvalidLitVal {}

impl TryFrom<c_int> for Lit {
    type Error = InvalidLitVal;

    fn try_from(val: c_int) -> StdResult<Self, Self::Error> {
        if val == 0 || val == c_int::min_value() {
            return Err(InvalidLitVal(val))
        }
        Ok(Self(val))
    }
}

/// The polarity of a literal.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    /// Positive polarity.
    Pos,
    /// Negative polarity.
    Neg,
}

impl Lit {
    /// Creates a new `Lit` from the given value.
    ///
    /// # Safety
    ///
    /// This does not check if the given value is zero and thus invalid.
    pub unsafe fn new_unchecked(val: c_int) -> Self {
        debug_assert!(val != 0);
        Self(val)
    }

    /// Returns the underlying `c_int` representation of `self`.
    pub fn to_raw(self) -> c_int {
        self.0
    }

    /// Returns the variable of `self`.
    pub fn var(self) -> Var {
        Var(self.to_raw().abs())
    }

    /// Returns the sign of `self`.
    pub fn sign(self) -> Sign {
        if self.to_raw().is_positive() {
            Sign::Pos
        } else {
            Sign::Neg
        }
    }
}

/// A clause from the IPASIR solver.
pub struct Clause<'a> {
    /// The zero-ended literals.
    lits: &'a [Lit],
}

impl<'a> From<&'a [Lit]> for Clause<'a> {
    fn from(lits: &'a [Lit]) -> Self {
        Self { lits }
    }
}

impl<'a> Clause<'a> {
    /// Returns the length of the clause.
    pub fn len(&self) -> usize {
        self.lits.len()
    }

    /// Returns `true` if the clause is empty.
    ///
    /// # Note
    ///
    /// Normally a clause should never be empty.
    pub fn is_empty(&self) -> bool {
        self.lits.len() == 0
    }

    /// Returns an iterator over the literals of the clause.
    pub fn iter(&self) -> LitIter {
        LitIter { iter: self.lits.iter() }
    }
}

impl<'a, Idx> std::ops::Index<Idx> for Clause<'a>
where
    Idx: std::slice::SliceIndex<[Lit]>,
{
    type Output = <[Lit] as std::ops::Index<Idx>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.lits[index]
    }
}

/// Iterator over the literals of a clause.
#[derive(Debug, Clone)]
pub struct LitIter<'a> {
    /// The underlying iterator.
    iter: std::slice::Iter<'a, Lit>,
}

impl<'a> ExactSizeIterator for LitIter<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a> Iterator for LitIter<'a> {
    type Item = Lit;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().cloned()
    }
}

impl<'a> DoubleEndedIterator for LitIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().cloned()
    }
}
