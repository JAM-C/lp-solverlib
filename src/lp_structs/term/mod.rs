
use super::Variable;

// Represents a term in the linear program
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Term<'a> {
    pub coefficient: f64,
    pub variable: Variable<'a>,
}

impl<'a> Term<'a> {
    pub fn new(coefficient: f64, variable: Variable<'a>) -> Self {
        Self { variable, coefficient }
    }
}

mod add;
mod mul;
mod div {
    use super::Term;
    use std::ops::Div;

    impl<'a> Div<f64> for Term<'a> {
        type Output = Term<'a>;

        fn div(mut self, rhs: f64) -> Self::Output {
            self.coefficient /= rhs;
            self            
        }
        
    }

    impl<'a> Div<f64> for &Term<'a> {
        type Output = Term<'a>;

        fn div(self, rhs: f64) -> Self::Output {
            Term {
                coefficient: self.coefficient / rhs,
                variable: self.variable,
            }
        }
        
    }
}

impl<'a> From<Variable<'a>> for Term<'a> {
    fn from(val: Variable<'a>) -> Self {
        Self { variable: val, coefficient: 1.0 }
    }
}
