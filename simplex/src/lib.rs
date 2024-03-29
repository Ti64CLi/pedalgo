//! Implementation of the Simplex algorithm
//! definition of the simplex object

pub mod app;
pub mod constraint;
pub mod linear_function;
mod polyhedron;
mod error;

use crate::linear_function::{Coefficient, Variable};
use constraint::Constraints;
use linear_function::LinearFunction;
use std::collections::HashSet;
use itertools::Itertools;
use crate::error::SimplexError;

#[derive(Debug, Clone)]
pub struct LinearProgram {
    pub linear_function: LinearFunction,
    pub constraints: Constraints,
}

/// Simplex object
#[derive(Debug, Clone)]
pub struct Simplex {
    index: usize,
    historic: Vec<LinearProgram>,
}

impl LinearProgram {
    pub fn pivot(&mut self, var: String) -> Result<(), SimplexError> {
        let max_constraint_index = self.constraints.most_restrictive(&var).ok_or(SimplexError::Unbounded)?;
        self.constraints.pivot(max_constraint_index, &var);
        self.linear_function.replace(&var, &self.constraints[max_constraint_index].right);
        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.constraints.is_valid()
    }

    pub fn is_unbounded(&self) -> bool {
        self.linear_function.var_iter()
            .any(|v| self.constraints.most_restrictive(v).is_none())
    }

    /// only works on a proper linear program which is verif by is_valid function
    pub fn point(&self) -> Vec<f32> {
        if !self.is_valid() {
            panic!("Linear program is not valid");
        }
        let variables = self.non_gap_variables();
        let mut point = vec![0.0; variables.len()];

        for constraint in self.constraints.iter() {
            if let Some(left_variable) = constraint.left.name_single_variable() {
                if let Some(index) = variables.iter().position(|v| *v == left_variable) {
                    point[index] = constraint.right.constant;
                }
            }
        }
        point
    }

    pub fn values(&self) -> Vec<(Variable, Coefficient)> {
        let variables = self.non_gap_variables();
        let values = self.point();

        variables.into_iter().zip(values).collect()
    }

    /// Give every non gap variables of a linear program sorted by alphabetical order
    pub fn non_gap_variables(&self) -> Vec<String> {
        let mut var_set: HashSet<Variable> = HashSet::from_iter(self.linear_function.non_gap_variables());
        for v in self.constraints.non_gap_variables() {
            var_set.insert(v);
        }
        var_set.into_iter().sorted().collect()
    }

    fn out_of_base_variables(&self) -> Vec<Variable> {
        let mut variables = HashSet::new();
        for constraint in self.constraints.iter() {
            for var in constraint.right.var_iter() {
                variables.insert(var);
            }
        }
        variables.into_iter().cloned().collect()
    }
}

impl Simplex {
    fn is_first_step(&self) -> bool {
        self.index == 0
    }

    pub fn next_step(&mut self, use_bland_rule: bool) -> Result<(), SimplexError> {
        if let Some(var) = self
            .current_state()
            .linear_function
            .first_positive_coefficient(use_bland_rule)
        {
            if self.index == self.historic.len() - 1 {
                let mut new = self.current_state().clone();
                new.pivot(var)?;
                self.historic.push(new);
            }
            self.index += 1;
            Ok(())
        } else {
            Err(SimplexError::AlreadyOptimal)
        }
    }

    pub fn previous_step(&mut self) {
        if !self.is_first_step() {
            self.index -= 1;
        }
    }

    /// Returns a reference to the current state of the algorithm
    pub fn current_state(&self) -> &LinearProgram {
        &self.historic[self.index]
    }

    pub fn current_point(&self) -> Vec<f32> {
        self.current_state().point()
    }

    pub fn current_values(&self) -> Vec<(Variable, Coefficient)> {
        self.current_state().values()
    }
}

impl From<LinearProgram> for Simplex {
    fn from(value: LinearProgram) -> Self {
        Simplex {
            index: 0,
            historic: vec![value],
        }
    }
}

impl std::fmt::Display for LinearProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "max {}", self.linear_function)?;
        write!(f, "{}", self.constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_gap_variables() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x + y <= 2\n x + 2y <= 3").unwrap(),
        };
        assert_eq!(
            lp.non_gap_variables(),
            HashSet::from(["x".to_string(), "y".to_string()])
        );
    }

    #[test]
    fn test_point_1() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x + y <= 2\n x + 2y <= 3").unwrap(),
        };
        assert_eq!(lp.point(), vec![0.0, 0.0]);
    }

    #[test]
    // ne passe pas
    fn test_point_2() {
        use std::str::FromStr;
        let lp = LinearProgram {
            linear_function: LinearFunction::from_str("x + 2y").unwrap(),
            constraints: Constraints::compile("x <= 200\n 300 - x + 2y >= 0").unwrap(),
        };
        let mut simplex = Simplex::from(lp);
        simplex.next_step(true).unwrap();
        assert_eq!(simplex.current_point(), vec![200.0, 0.0]);
    }
}
