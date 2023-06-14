use super::{Expression, Term};

use std::ops::Div;

impl<'a> Div<f64> for &Expression<'a> {
    type Output = Expression<'a>;

    fn div(self, rhs: f64) -> Self::Output {
        Expression::new(
            &self
                .terms
                .iter()
                .map(|term| term / rhs)
                .collect::<Vec<Term>>(),
            self.constant / rhs,
        )
    }
}

impl<'a> Div<f64> for Expression<'a> {
    type Output = Expression<'a>;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.terms =
            self
                .terms
                .iter()
                .map(|term| term / rhs)
                .collect();
        self.constant /= rhs;
        self
    }
}
