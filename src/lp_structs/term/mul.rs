use super::Term;

use std::ops::Mul;

impl<'a> Mul<f64> for Term<'a> {
    type Output = Term<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            coefficient: self.coefficient * rhs,
            variable: self.variable,
        }
    }
}

impl<'a> Mul<Term<'a>> for f64 {
    type Output = Term<'a>;

    fn mul(self, rhs: Term<'a>) -> Self::Output {
        rhs * self
    }
}

impl<'a> Mul<f64> for &Term<'a> {
    type Output = Term<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        Term {
            coefficient: self.coefficient * rhs,
            variable: self.variable,
        }
    }
}

impl<'a> Mul<&Term<'a>> for f64 {
    type Output = Term<'a>;

    fn mul(self, rhs: &Term<'a>) -> Self::Output {
        rhs * self
    }
}
