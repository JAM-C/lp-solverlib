use super::Expression;
use std::ops::DivAssign;

impl DivAssign<f64> for Expression<'_> {
    fn div_assign(&mut self, rhs: f64) {
        self.constant /= rhs;
        for term in self.terms.iter_mut() {
            term.coefficient /= rhs;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Variable as V;

    #[test]
    fn test_div_assign() {
        use super::super::Term;
        use super::Expression;
        let x = V::new("X");
        let mut expr: Expression<'_> = Term::new(1.0, x) + 1.0;
        expr /= 2.0;
        assert_eq!(expr, Expression::new(&[Term::new(0.5, x)], 0.5));
    }
}
