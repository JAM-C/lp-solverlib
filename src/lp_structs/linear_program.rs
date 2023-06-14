use std::fmt::Display;

use super::Constraint;
use super::Objective;
use super::Variable as V;
use super::VariableContainer;

/// Represents a linear program with a vector of constraints
/// and an objective function
pub struct LinearProgram<'a> {
    pub objective: Objective<'a>,
    pub constraints: Vec<Constraint<'a>>,
}

impl Display for LinearProgram<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.objective)?;
        for constraint in &self.constraints {
            writeln!(f, "{}", constraint)?;
        }
        Ok(())
    }
}

impl<'a> VariableContainer<'a> for LinearProgram<'a> {
    type I = std::vec::IntoIter<V<'a>>;

    fn variables_iter(&self) -> Self::I {
        let mut vars = self.objective.variables_iter()
            .chain(self.constraints.iter().flat_map(|c| c.variables_iter()))
            .collect::<Vec<_>>();
        vars.dedup();
        vars.into_iter()
    }
}
