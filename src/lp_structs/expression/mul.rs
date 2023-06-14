use super::{Expression, Term};

use std::ops::Mul;

impl<'a> Mul<f64> for &Expression<'a> {
    type Output = Expression<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        Expression::new(
            &self
                .terms
                .iter()
                .map(|term| term * rhs)
                .collect::<Vec<Term>>(),
            self.constant * rhs,
        )
    }
}

impl<'a> Mul<&Expression<'a>> for f64 {
    type Output = Expression<'a>;

    fn mul(self, rhs: &Expression<'a>) -> Self::Output {
        rhs * self
    }
}

impl<'a> Mul<f64> for Expression<'a> {
    type Output = Expression<'a>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.terms =
            self
                .terms
                .iter()
                .map(|term| term * rhs)
                .collect();
        self.constant *= rhs;
        self
    }
}

impl<'a> Mul<Expression<'a>> for f64 {
    type Output = Expression<'a>;

    fn mul(self, rhs: Expression<'a>) -> Self::Output {
        rhs * self
    }
}
