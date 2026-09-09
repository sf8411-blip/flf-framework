//! Problem representation and definition

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// A Representation is the computational form of a problem
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Representation {
    pub id: String,
    pub input_space: Vec<f64>,
    pub expected_output: Vec<f64>,
    pub constraints: Vec<String>,
}

impl Representation {
    pub fn new(id: String, input_space: Vec<f64>, expected_output: Vec<f64>) -> Self {
        Self {
            id,
            input_space,
            expected_output,
            constraints: Vec::new(),
        }
    }

    pub fn with_constraints(mut self, constraints: Vec<String>) -> Self {
        self.constraints = constraints;
        self
    }
}

/// A Problem defines what needs to be solved
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub input_space: Vec<f64>,
    pub expected_output: Vec<f64>,
    pub difficulty: f64,
}

impl Problem {
    pub fn new(id: String, input_space: Vec<f64>, expected_output: Vec<f64>) -> Self {
        Self {
            id,
            input_space,
            expected_output,
            difficulty: 1.0,
        }
    }

    pub fn with_difficulty(mut self, difficulty: f64) -> Self {
        self.difficulty = difficulty.clamp(0.0, 10.0);
        self
    }

    /// Convert problem to its computational representation
    pub fn represent(&self) -> Result<Representation> {
        if self.input_space.is_empty() || self.expected_output.is_empty() {
            return Err(Error::RepresentationFailed(
                "Input and output spaces cannot be empty".to_string(),
            ));
        }

        Ok(Representation {
            id: format!("{}_repr", self.id),
            input_space: self.input_space.clone(),
            expected_output: self.expected_output.clone(),
            constraints: vec![
                format!("input_dim={}", self.input_space.len()),
                format!("output_dim={}", self.expected_output.len()),
                format!("difficulty={}", self.difficulty),
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_creation() {
        let problem = Problem::new(
            "test_1".to_string(),
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0],
        );
        assert_eq!(problem.id, "test_1");
        assert_eq!(problem.input_space.len(), 3);
    }

    #[test]
    fn test_problem_representation() {
        let problem = Problem::new(
            "test_2".to_string(),
            vec![1.0, 2.0],
            vec![3.0],
        );
        let repr = problem.represent().unwrap();
        assert_eq!(repr.input_space.len(), 2);
        assert_eq!(repr.expected_output.len(), 1);
    }

    #[test]
    fn test_empty_input_fails() {
        let problem = Problem::new("test_3".to_string(), vec![], vec![1.0]);
        assert!(problem.represent().is_err());
    }

    #[test]
    fn test_problem_with_difficulty() {
        let problem = Problem::new(
            "test_4".to_string(),
            vec![1.0],
            vec![1.0],
        )
        .with_difficulty(5.0);
        assert_eq!(problem.difficulty, 5.0);
    }
}
