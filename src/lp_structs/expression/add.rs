use super::super::Term;

use super::Expression;

use std::ops::Add;

impl<'a> Add<&Expression<'a>> for &Expression<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: &Expression<'a>) -> Expression<'a> {
        let mut expr = self.clone();
        expr.terms.extend(rhs.terms.clone());
        expr.consolidate();
        expr
    }
}

impl<'a> Add<&Term<'a>> for &Expression<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: &Term<'a>) -> Expression<'a> {
        let mut expr = self.clone();
        expr.terms.push(*rhs);
        expr.consolidate();
        expr
    }
}

impl<'a> Add<&Expression<'a>> for &Term<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: &Expression<'a>) -> Expression<'a> {
        rhs + self
    }
}

impl<'a> Add<Term<'a>> for Expression<'a> {
    type Output = Expression<'a>;

    fn add(mut self, rhs: Term<'a>) -> Expression {
        self.terms.push(rhs);
        self.consolidate();
        self
    }
}

impl<'a> Add<Expression<'a>> for Term<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: Expression<'a>) -> Expression {
        rhs + self
    }
}

impl<'a> Add<f64> for &Expression<'a> {
    type Output = Expression<'a>;

    fn add(self, rhs: f64) -> Expression<'a> {
        let mut expr = self.clone();
        expr.constant += rhs;
        expr
    }
}

impl<'a> Add<&Expression<'a>> for f64 {
    type Output = Expression<'a>;

    fn add(self, rhs: &Expression<'a>) -> Expression<'a> {
        rhs + self
    }
}

impl<'a> Add<f64> for Expression<'a> {
    type Output = Expression<'a>;

    fn add(mut self, rhs: f64) -> Expression<'a> {
        self.constant += rhs;
        self
    }
}

impl<'a> Add<Expression<'a>> for f64 {
    type Output = Expression<'a>;

    fn add(self, rhs: Expression<'a>) -> Expression<'a> {
        rhs + self
    }
}
