use std::fmt::Display;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use super::Expression;
use super::Operator;
use super::Substitutable;
use super::VariableContainer;
use super::Variable as V;

// Represents a constraint in the linear program

#[derive(Clone, Debug)]
pub struct Constraint<'a> {
    pub lhs: Expression<'a>,
    pub operator: Operator,
    pub rhs: Expression<'a>,
}

impl Display for Constraint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.operator, self.rhs)
    }
}

impl<'a> Substitutable<'a> for Constraint<'a> {
    fn substitute(&mut self, var: &crate::Variable<'a>, expr: &Expression<'a>) {
        self.lhs.substitute(var, expr);
        self.rhs.substitute(var, expr);
    }
}

impl<'a> VariableContainer<'a> for Constraint<'a> {
    type I = std::vec::IntoIter<V<'a>>;

    fn variables_iter(&self) -> Self::I {
        let mut vars = self.lhs.variables_iter()
            .chain(self.rhs.variables_iter())
            .collect::<Vec<_>>();
        vars.dedup();
        vars.into_iter()
    }
}

impl<'a> Mul<f64> for Constraint<'a> {
    type Output = Constraint<'a>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<'a> Mul<Constraint<'a>> for f64 {
    type Output = Constraint<'a>;

    fn mul(self, rhs: Constraint<'a>) -> Self::Output {
        rhs * self
    }
}

impl<'a> Mul<f64> for &Constraint<'a> {
    type Output = Constraint<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut c = self.clone();
        c *= rhs;
        c
    }
}

impl<'a> Mul<&Constraint<'a>> for f64 {
    type Output = Constraint<'a>;

    fn mul(self, rhs: &Constraint<'a>) -> Self::Output {
        rhs * self
    }
}

impl<'a> Div<f64> for Constraint<'a> {
    type Output = Constraint<'a>;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<'a> Div<f64> for &Constraint<'a> {
    type Output = Constraint<'a>;

    fn div(self, rhs: f64) -> Self::Output {
        let mut c = self.clone();
        c /= rhs;
        c
    }
}

impl<'a> MulAssign<f64> for Constraint<'a> {
    fn mul_assign(&mut self, rhs: f64) {
        if rhs < 0.0 {
            self.operator = self.operator.flip();
        }
        self.lhs *=rhs;
        self.rhs *= rhs;
    }
}

impl<'a> DivAssign<f64> for Constraint<'a> {
    fn div_assign(&mut self, rhs: f64) {
        if rhs < 0.0 {
            self.operator = self.operator.flip();
        }
        self.lhs /=rhs;
        self.rhs /= rhs;
    }
}