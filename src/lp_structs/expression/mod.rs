use std::{
    fmt::Display,
};

use itertools::Itertools;

use super::traits::RecursionGuard;
use super::{Substitutable, VariableContainer, Variable as V};

use super::Term;

// Represents an expression in the linear program
#[derive(Clone, Debug, PartialEq)]
pub struct Expression<'a> {
    pub terms: Vec<Term<'a>>,
    pub constant: f64,
}

impl Expression<'_> {
    pub fn new<'a>(terms: &[Term<'a>], constant: f64) -> Expression<'a> {
        let mut exp = Expression {
            terms: terms.to_vec(),
            constant,
        };
        exp.consolidate();
        exp
    }

    pub fn zero() -> Expression<'static> {
        Expression {
            terms: vec![],
            constant: 0.0,
        }
    }

    pub fn consolidate(&mut self) {
        self.terms = self
            .terms
            .iter()
            .group_by(|term| term.variable)
            .into_iter()
            .map(|(v, group)| {
                let coefficient = group.map(|x| x.coefficient).sum();
                Term {
                    variable: v,
                    coefficient,
                }
            })
            .collect::<Vec<Term>>();
        self.terms
            .sort_by(|a, b| b.coefficient.partial_cmp(&a.coefficient).unwrap());
    }
}

impl<'a> Substitutable<'a> for Expression<'a> {
    fn substitute(&mut self, variable: &V<'a>, expression: &Expression<'a>) {
        self.terms = self
            .terms
            .iter()
            .flat_map(|sub_term| {
                if &sub_term.variable == variable {
                    expression
                        .terms
                        .iter()
                        .map(|term| Term {
                            variable: term.variable,
                            coefficient: term.coefficient * sub_term.coefficient,
                        })
                        .collect()
                } else {
                    vec![*sub_term]
                }
            })
            .collect();
        self.consolidate();
    }
}

impl<'a> VariableContainer<'a> for Expression<'a> {
    type I = std::vec::IntoIter<crate::Variable<'a>>;

    fn variables_iter(&self) -> Self::I {
        let mut vars = self.terms.iter().map(|term| term.variable).collect::<Vec<_>>();
        vars.dedup();
        vars.into_iter()
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut first = true;
        for term in &self.terms {
            if term.coefficient == 0.0 {
                continue;
            }
            if !first && term.coefficient > 0.0 {
                write!(f, " + ")?;
            }
            if term.coefficient < 0.0 {
                if first {
                    write!(f, "-")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            if term.coefficient.abs() != 1.0 {
                write!(f, "{}", term.coefficient.abs())?;
            }
            write!(f, "{}", term.variable)?;

            first = false;
        }
        if self.constant == 0.0 {
            return Ok(());
        } else if self.constant > 0.0 {
            if first {
                write!(f, "{}", self.constant)?;
            } else {
                write!(f, " + {}", self.constant)?;
            }
        } else if first {
            write!(f, "-{}", self.constant.abs())?;
        } else {
            write!(f, " - {}", self.constant.abs())?;
        }
        Ok(())
    }
}

impl<'a> From<Term<'a>> for Expression<'a> {
    fn from(term: Term<'a>) -> Expression<'a> {
        Expression {
            terms: vec![term],
            constant: 0.0,
        }
    }
}

impl<'a, X> From<X> for Expression<'a>
where
    X: IntoIterator<Item = Term<'a>>
{
    fn from(terms: X) -> Expression<'a> {
        Expression {
            terms: terms.into_iter().collect(),
            constant: 0.0,
        }
    }
}


impl<'a> From<V<'a>> for Expression<'a> {
    fn from(variable: V<'a>) -> Expression<'a> {
        Expression {
            terms: vec![variable.into()],
            constant: 0.0,
        }
    }
}

impl<'a> super::Evaluable<'a> for Expression<'a> {
    fn evaluate<X>(&self, source: &X) -> Result<f64, super::EvaluationError>
    where
        X: super::EvaluationContext<'a> {
        self.terms
            .iter()
            .map(|term| {
                let value = source.evaluate(&term.variable)?;
                Ok(term.coefficient * value)
            })
            .sum::<Result<f64, super::EvaluationError>>()
            .map(|x| x + self.constant)
    }

    fn evaluate_with_recursion_guard<X>(&self, source: &X, guard: &RecursionGuard) -> Result<f64, super::EvaluationError>
    where
        X: super::EvaluationContext<'a> {
        self.terms
            .iter()
            .map(|term| {
                let value = source.evaluate_with_recursion_guard(&term.variable, guard)?;
                Ok(term.coefficient * value)
            })
            .sum::<Result<f64, super::EvaluationError>>()
            .map(|x| x + self.constant)
    }
}

mod div_assign;
mod div;
mod mul_assign;
mod mul;
mod add_assign;
mod add;
mod sub_assign;
mod sub;


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_expression_display() {
        let x: V = V::new("x");
        let y: V = V::new("y");
        let expr = (-3.0 * x) + 
            (1.5 * y) + 2.0;
        assert_eq!(format!("{}", expr), "1.5y - 3x + 2");
    }

    #[test]
    fn test_expression_substitute() {
        let x: V = V::new("x");
        let y: V = V::new("y");
        let mut expr = (x * -3.0) + (y * 1.5) + 2.0;
        let expr2 = Expression::from(y * 1.0) - 2.0;
        expr.substitute(&x, &expr2);
        assert_eq!(format!("{}", expr), "-1.5y + 2");
    }
}
