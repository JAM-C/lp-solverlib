use super::super::Expression;

use super::Term;

use std::ops::Add;

impl<'a> Add<&Term<'a>> for &Term<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: &Term<'a>) -> Self::Output {
        Expression::new(&[*self, *rhs], 0.0)
    }
}

impl<'a> Add<Term<'a>> for Term<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: Term<'a>) -> Self::Output {
        Expression::new(&[self, rhs], 0.0)
    }
}

impl<'a> Add<f64> for Term<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::new(&[self], rhs)
    }
}

impl<'a> Add<f64> for &Term<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::new(&[*self], rhs)
    }
}

impl<'a> Add<Term<'a>> for f64 {
    type Output = Expression<'a>;

    fn add(self, rhs: Term<'a>) -> Self::Output {
        rhs + self
    }
}

impl<'a> Add<&Term<'a>> for f64 {
    type Output = Expression<'a>;

    fn add(self, rhs: &Term<'a>) -> Self::Output {
        rhs + self
    }
}