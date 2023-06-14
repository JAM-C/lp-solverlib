use super::super::Term;

use super::Expression;

use std::ops::AddAssign;

impl<'a> AddAssign<Expression<'a>> for Expression<'a> {
    fn add_assign(&mut self, rhs: Expression<'a>) {
        self.terms.extend(rhs.terms);
        self.constant += rhs.constant;
        self.consolidate();
    }
}

impl<'a> AddAssign<&Expression<'a>> for Expression<'a> {
    fn add_assign(&mut self, rhs: &Expression<'a>) {
        self.terms.extend(rhs.terms.clone());
        self.constant += rhs.constant;
        self.consolidate();
    }
}

impl<'a> AddAssign<Term<'a>> for Expression<'a> {
    fn add_assign(&mut self, rhs: Term<'a>) {
        self.terms.push(rhs);
        self.consolidate();
    }
}

impl<'a> AddAssign<&Term<'a>> for Expression<'a> {
    fn add_assign(&mut self, rhs: &Term<'a>) {
        self.terms.push(*rhs);
        self.consolidate();
    }
}

impl AddAssign<f64> for Expression<'_> {
    fn add_assign(&mut self, rhs: f64) {
        self.constant += rhs;
    }
}
