use super::{Expression, Variable as V};
pub trait Substitutable<'a> {
    fn substitute(&mut self, var: &V<'a>, expr: &Expression<'a>);
}

pub trait VariableContainer<'a> {

    type I: Iterator<Item = V<'a>> + Sized;

    fn variables_iter(&self) -> Self::I;

    fn next_temp_variable(&self) -> V<'a> {
        let index = self.variables_iter()
            .filter_map(|var| {
                if let V::Temp { id } = var {
                    Some(id)
                } else {
                    None
                }
            })
            .max()
            .map(|id| id + 1)
            .unwrap_or(0);
        V::Temp { id: index }
    }
}

pub enum EvaluationError {
    UnsolvedVariable,
    CircularDependency,
}

pub struct RecursionGuard<'a, 'b> {
variable: V<'a>,
    prev_guard: Option<&'b RecursionGuard<'a, 'b>>
}

impl<'a, 'b> RecursionGuard<'a, 'b> {
    pub fn contains(&self, variable: &V<'a>) -> bool {
        if &self.variable == variable {
            true
        } else {
            match self.prev_guard {
                Some(guard) => guard.contains(variable),
                None => false,
            }
        }
    }

    pub fn next(&'b self, variable: V<'a>) -> RecursionGuard<'a, 'b> {
        RecursionGuard {
            variable,
            prev_guard: Some(self),
        }
    }

    pub fn new(variable: V<'a>) -> RecursionGuard<'a, 'b> {
        RecursionGuard {
            variable,
            prev_guard: None,
        }
    }
}

pub trait Evaluable<'a> {
    fn evaluate<X>(&self, source: &X) -> Result<f64, EvaluationError>
    where
        X: EvaluationContext<'a>;
    
    fn evaluate_with_recursion_guard<X>(&self, source: &X, guard: &RecursionGuard) -> Result<f64, EvaluationError>
    where
        X: EvaluationContext<'a>;
}


pub trait EvaluationContext<'a> {
    fn evaluate(&self, var: &V<'a>) -> Result<f64, EvaluationError>;

    fn evaluate_with_recursion_guard(&self, var: &V<'a>, guard: &RecursionGuard) -> Result<f64, EvaluationError>;
}