use std::{fmt::Display, ops::Mul};
use super::Term;

// Represents a variable in the linear program with a unique index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variable<'a> {
    Named{ name: &'a str},
    Temp{ id: usize },
}

impl<'a> Variable<'a> {
    pub fn new(name: &'a str) -> Self {
        Variable::Named{name}
    }
}

impl Display for Variable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Variable::Named{name} => write!(f, "{}", name),
            Variable::Temp{id} => write!(f, "x_{}", id),
        }
    }
}

impl<'a> Mul<f64> for Variable<'a> {
    type Output = Term<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        Term::new(rhs, self)
    }
}

impl<'a> Mul<Variable<'a>> for f64 {
    type Output = Term<'a>;

    fn mul(self, rhs: Variable<'a>) -> Self::Output {
        Term::new( self, rhs)
    }
}

impl<'a> Mul<f64> for &Variable<'a> {
    type Output = Term<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        Term::new( rhs, *self)
    }
}

impl<'a> Mul<&Variable<'a>> for f64 {
    type Output = Term<'a>;

    fn mul(self, rhs: &Variable<'a>) -> Self::Output {
        Term::new(self, *rhs)
    }
}