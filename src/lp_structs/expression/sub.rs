use super::super::Term;

use super::Expression;

use std::ops::Sub;

impl<'a> Sub<&Expression<'a>> for &Expression<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: &Expression<'a>) -> Expression<'a> {
        self + &(rhs * -1.0)
    }
}

impl<'a> Sub<&Term<'a>> for &Expression<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: &Term<'a>) -> Expression<'a> {
        self + &(rhs * -1.0)
    }
}

impl<'a> Sub<&Expression<'a>> for &Term<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: &Expression<'a>) -> Self::Output {
        rhs - self
    }
}

impl<'a> Sub<Term<'a>> for Expression<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: Term<'a>) -> Self::Output {
        self + (rhs * -1.0)
    }
}

impl<'a> Sub<Expression<'a>> for Term<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: Expression<'a>) -> Self::Output {
        rhs - self
    }
}

impl<'a> Sub<f64> for &Expression<'a> {
    type Output = Expression<'a>;

    fn sub(self, rhs: f64) -> Self::Output {
        let mut expr = self.clone();
        expr.constant -= rhs;
        expr
    }
}

impl<'a> Sub<f64> for Expression<'a> {
    type Output = Expression<'a>;

    fn sub(mut self, rhs: f64) -> Self::Output {
        self.constant -= rhs;
        self
    }
}

impl<'a> Sub<&Expression<'a>> for f64 {
    type Output = Expression<'a>;

    fn sub(self, rhs: &Expression<'a>) -> Self::Output {
        rhs - self
    }
}

impl<'a> Sub<Expression<'a>> for f64 {
    type Output = Expression<'a>;

    fn sub(self, rhs: Expression<'a>) -> Self::Output {
        rhs - self
    }
}
