use super::super::Term;

use super::Expression;

use std::ops::SubAssign;

impl<'a> SubAssign<&Expression<'a>> for Expression<'a> {
    fn sub_assign(&mut self, rhs: &Expression<'a>) {
        *self = &*self + &(rhs * -1.0);
    }
}

impl<'a> SubAssign<Expression<'a>> for Expression<'a> {
    fn sub_assign(&mut self, rhs: Expression<'a>) {
        *self = &*self + &(rhs * -1.0);
    }
}

impl<'a> SubAssign<&Term<'a>> for Expression<'a> {
    fn sub_assign(&mut self, rhs: &Term<'a>) {
        *self = &*self + &(rhs * -1.0)
    }
}

impl<'a> SubAssign<f64> for Expression<'a> {
    fn sub_assign(&mut self, rhs: f64) {
        self.constant -= rhs;
    }
}
