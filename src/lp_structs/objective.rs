use std::fmt::{Error, Display, Formatter};

use super::{Expression, Substitutable, VariableContainer, Variable as V};

/// Represents the objective function in a linear program
/// The objective function is a vector of terms and a boolean
/// indicating whether the objective is to be maximized
/// or minimized
#[derive(Clone, Debug)]
pub struct Objective<'a> {
    pub expression: Expression<'a>,
    pub maximize: bool,
}

impl<'a> From<Expression<'a>> for Objective<'a> {
    fn from(expression: Expression<'a>) -> Self {
        Objective {
            expression,
            maximize: true,
        }
    }
}

impl<'a> Objective<'a> {
    pub fn new(expression: Expression<'a>, maximize: bool) -> Self {
        Objective {
            expression,
            maximize,
        }
    }
}

impl Display for Objective<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.maximize {
            write!(f, "max: ")?;
        } else {
            write!(f, "min: ")?;
        }
        write!(f, "{}", self.expression)
    }
}

impl<'a> Substitutable<'a> for Objective<'a> {
    fn substitute(&mut self, var: &crate::Variable<'a>, expr: &Expression<'a>) {
        self.expression.substitute(var, expr);
    }
}

impl<'a> VariableContainer<'a> for Objective<'a> {
    type I = std::vec::IntoIter<V<'a>>;

    fn variables_iter(&self) -> Self::I {
        self.expression.variables_iter().collect::<Vec<_>>().into_iter()
    }
}
