use std::collections::{HashMap, HashSet};

use array2d::Array2D;

use crate::{
    lp_structs::{
        Evaluable, EvaluationContext, EvaluationError, RecursionGuard, VariableContainer,
    },
    Constraint, Expression, LinearProgram, Objective, Operator, Substitutable, Term, Variable,
};

struct ProgramTransformer<'a> {
    constraints: Vec<Constraint<'a>>,
    objective: Objective<'a>,
    objective_flipped: bool,
    substitutions: HashMap<Variable<'a>, Expression<'a>>,
}

impl<'a> Substitutable<'a> for ProgramTransformer<'a> {
    fn substitute(&mut self, var: &crate::Variable<'a>, expr: &Expression<'a>) {
        for c in &mut self.constraints {
            c.substitute(var, expr);
        }
        self.objective.substitute(var, expr);
        self.substitutions.insert(*var, expr.clone());
    }
}


impl<'a> VariableContainer<'a> for ProgramTransformer<'a> {
    type I = std::collections::hash_set::IntoIter<Variable<'a>>;

    fn variables_iter(&self) -> Self::I {
        let mut variables = HashSet::new();
        for c in &self.constraints {
            variables.extend(c.variables_iter());
        }
        variables.extend(self.objective.variables_iter());
        variables.into_iter()
    }
}

impl<'a> ProgramTransformer<'a> {
    pub fn new<'b>(program: &LinearProgram<'b>) -> ProgramTransformer<'b> {
        ProgramTransformer {
            constraints: program.constraints.clone(),
            objective: program.objective.clone(),
            objective_flipped: false,
            substitutions: HashMap::new(),
            solved_values: HashMap::new(),
        }
    }

    fn make_constraints_single_sided(&mut self) {
        self.constraints.iter_mut().for_each(|c| {
            c.lhs -= Expression::from(c.rhs.terms.clone());
            c.rhs -= c.lhs.constant;
            c.lhs.constant = 0.0;
            c.rhs.terms = vec![];
        })
    }

    fn make_objective_maximizing(&mut self) {
        if !self.objective.maximize {
            self.objective.expression *= -1.0;
            self.objective.maximize = true;
            self.objective_flipped = true;
        }
    }

    fn make_all_single_variable_constraints_unit_constraints(&mut self) {
        for c in &mut self.constraints {
            if c.lhs.terms.len() == 1 && c.rhs.terms.is_empty() {
                // if the constraint is in the form of nx >= y
                let Term {
                    coefficient,
                    variable: _,
                } = c.lhs.terms[0];
                c.lhs /= coefficient;
                c.rhs /= coefficient;
                if coefficient < 0.0 {
                    c.operator = c.operator.flip();
                }
            }
        }
    }

    // Replaces all variables that are not lower bounded with
    // a new variable that is lower bounded by zero sometimes
    // known as a slack variable
    fn make_non_zero_bounded_variables_positive(&mut self) {
        let mut vars_for_substitution = vec![];
        for c in &mut self.constraints {
            if c.lhs.terms.len() == 1 && c.operator == Operator::GTE && c.rhs.terms.is_empty() {
                // if the constraint is in the form of nx >= y
                let Term {
                    coefficient,
                    variable,
                } = c.lhs.terms[0];
                assert!(coefficient == 1.0);
                vars_for_substitution.push((variable, c.rhs.clone()));
            }
        }
        for (var, expr) in vars_for_substitution {
            let new_var = self.next_temp_variable();
            self.substitute(&var, &(expr + 1.0 * new_var));
        }
        self.make_constraints_single_sided();
    }

    // Identifies all variables that are lower bounded by zero
    // and stores them in the zero_bounded_variables set
    fn identify_zero_bounded_variables(&self) -> HashSet<Variable<'a>>{
        let mut zero_bounded_variables = HashSet::new();
        for c in &self.constraints {
            if c.lhs.terms.len() == 1
                && c.operator == Operator::GTE
                && c.rhs.terms.is_empty()
                && c.lhs.constant == 0.0
                && c.rhs.constant == 0.0
            {
                // if the constraint is in the form of nx >= y
                let Term {
                    coefficient,
                    variable,
                } = c.lhs.terms[0];
                if coefficient > 0.0 {
                    zero_bounded_variables.insert(variable);
                }
            }
        }
        zero_bounded_variables
    }

    // Replaces all variables that are not lower bounded with
    // the difference of two new variables that are both lower
    // bounded
    fn eliminate_non_lower_bounded_variables(&mut self) {
        let zero_bounded_variables = self.identify_zero_bounded_variables();
        for var in self.variables_iter() {
            if !zero_bounded_variables.contains(&var) {
                let new_var_1 = self.next_temp_variable();
                let new_var_2 = self.next_temp_variable();
                self.substitute(&var, &((1.0 * new_var_1) + (-1.0 * new_var_2)));
            }
        }
    }

    fn eliminate_positivity_contraints(&mut self) {
        self.constraints.retain(|c| {
            !(
                c.lhs.terms.len() == 1
                && c.operator == Operator::GTE
                && c.rhs.terms.is_empty()
                && c.lhs.constant == 0.0
                && c.rhs.constant == 0.0
                && c.lhs.terms[0].coefficient > 0.0
            )
        });
    }

    fn make_all_variables_positive(&mut self) {
        // makes all constraints in the form of nx >= y into x >= z
        self.make_all_single_variable_constraints_unit_constraints();

        // replace all variables with a non-zero lower bound with a new variable that is lower bounded by zero
        self.make_non_zero_bounded_variables_positive();

        // replace all variables that are not lower bounded with the difference of two new variables that are both lower bounded
        self.eliminate_non_lower_bounded_variables();

        // since all variables are now constrained to be positive, we can remove the positivity constraints
        self.eliminate_positivity_contraints(); 
        
        // clean up the constraints
        self.make_constraints_single_sided()
    }

    fn make_all_constraints_equality_constraints(&mut self) {
        // we cannot generate a new variable while iterating mutably over the constraints,
        // so we back out of the loop and try again if we need to generate a new variable
        // this is a bit of a hack, but it works
        // TODO: implement a more elegant solution
        // TODO: assess the performance impact of this
        let mut completed = false;
        while !completed {
            completed = true;
            let next_var = self.next_temp_variable();
            for c in &mut self.constraints {
                match c.operator {
                    Operator::GTE => {
                        c.lhs += 1.0 * next_var;
                        c.operator = Operator::Eq;
                        completed = false;
                        break;
                    }
                    Operator::LTE => {
                        c.lhs += -1.0 * next_var;
                        c.operator = Operator::Eq;
                        completed = false;
                        break;
                    }
                    _ => {}
                }
            }
        }
        assert!(self.constraints.iter().all(|c| c.operator == Operator::Eq));
    }

    fn standardize(&mut self) {
        self.make_constraints_single_sided();
        self.make_all_variables_positive();
        self.make_all_constraints_equality_constraints();

        self.make_objective_maximizing();

    }
}
struct SimplexTableau<'a> {
    tableau: Array2D<f64>,
    variables: Vec<Variable<'a>>,
    substitutions: HashMap<Variable<'a>, Expression<'a>>,
    solved_values: HashMap<Variable<'a>, f64>,
}

impl<'a> EvaluationContext<'a> for SimplexTableau<'a> {
    fn evaluate(&self, var: &Variable<'a>) -> Result<f64, crate::lp_structs::EvaluationError> {
        if let Some(value) = self.solved_values.get(var) {
            Ok(*value)
        } else if let Some(expr) = self.substitutions.get(var) {
            Ok(expr.evaluate_with_recursion_guard(self, &RecursionGuard::new(*var))?)
        } else {
            Err(EvaluationError::UnsolvedVariable)
        }
    }

    fn evaluate_with_recursion_guard(
        &self,
        var: &Variable<'a>,
        guard: &RecursionGuard,
    ) -> Result<f64, crate::lp_structs::EvaluationError> {
        if guard.contains(var) {
            Err(EvaluationError::CircularDependency)
        } else if let Some(value) = self.solved_values.get(var) {
            Ok(*value)
        } else if let Some(expr) = self.substitutions.get(var) {
            let new_guard = guard.next(*var);
            Ok(expr.evaluate_with_recursion_guard(self, &new_guard)?)
        } else {
            Err(EvaluationError::UnsolvedVariable)
        }
    }
}


fn make_simplex_tableau<'a>(program: &LinearProgram<'a>) -> SimplexTableau<'a> {
    let mut tf = ProgramTransformer::new(program);
    tf.standardize();

    let variables: Vec<Variable<>> = tf.variables_iter().collect();

    let mut tableau = Array2D::filled_with(0.0, tf.constraints.len() + 1, variables.len() + 2);

    let final_column = tableau.num_columns() - 1;

    fn var_to_index<'a>(var: &Variable<'a>, variables: &[Variable<'a>]) -> usize {
        variables.iter().position(|v| v == var).unwrap() + 1
    }

    tableau[(0, 0)]= 1.0;

    for term in tf.objective.expression.terms.iter() {
        tableau[(0, var_to_index(&term.variable, &variables))] = term.coefficient;
    }

    for (index, constraint) in tf.constraints.iter().enumerate() {
        for term in constraint.lhs.terms.iter() {
            tableau[(index + 1, var_to_index(&term.variable, &variables))] = term.coefficient;
        }
        tableau[(index + 1, final_column)] = constraint.rhs.constant;
    }

    SimplexTableau {
        tableau,
        variables,
        substitutions: tf.substitutions,
        solved_values: HashMap::new(),
    }
}

impl SimplexTableau<'_> {
    
}

pub fn simplex_method_solver(program: &LinearProgram) {
    let tableau = make_simplex_tableau(&program);
}
