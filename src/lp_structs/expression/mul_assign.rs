use std::ops::MulAssign;
use super::Expression;


impl MulAssign<f64> for Expression<'_> {
    fn mul_assign(&mut self, rhs: f64) {
        self.constant *= rhs;
        for term in self.terms.iter_mut() {
            term.coefficient *= rhs;
        }
    }
}