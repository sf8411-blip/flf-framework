//! Computational structures that solve problems

use crate::error::{Error, Result};
use crate::problem::Representation;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A computational operation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Operation {
    Linear { weights: Vec<f64>, bias: f64 },
    Activation { kind: String },
    Compose { left: Box<Operation>, right: Box<Operation> },
}

impl Operation {
    pub fn apply(&self, input: &[f64]) -> Result<Vec<f64>> {
        match self {
            Operation::Linear { weights, bias } => {
                if weights.len() != input.len() {
                    return Err(Error::ExecutionFailed(
                        "Weight dimension mismatch".to_string(),
                    ));
                }
                let sum: f64 = weights.iter().zip(input.iter()).map(|(w, x)| w * x).sum::<f64>() + bias;
                Ok(vec![sum.tanh()])
            }
            Operation::Activation { kind } => {
                match kind.as_str() {
                    "tanh" => Ok(input.iter().map(|x| x.tanh()).collect()),
                    "relu" => Ok(input.iter().map(|x| x.max(0.0)).collect()),
                    "sigmoid" => Ok(input.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect()),
                    _ => Err(Error::ExecutionFailed(format!("Unknown activation: {}", kind))),
                }
            }
            Operation::Compose { left, right } => {
                let intermediate = left.apply(input)?;
                right.apply(&intermediate)
            }
        }
    }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        match self {
            Operation::Linear { weights, bias } => {
                for w in weights.iter_mut() {
                    *w += (rng.gen::<f64>() - 0.5) * 0.1;
                }
                *bias += (rng.gen::<f64>() - 0.5) * 0.1;
            }
            _ => {}
        }
    }
}

/// A computational structure that can solve problems
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputationalStructure {
    pub id: String,
    pub operations: Vec<Operation>,
    pub generation: usize,
    pub score: f64,
}

impl ComputationalStructure {
    pub fn new(id: String, operations: Vec<Operation>) -> Self {
        Self {
            id,
            operations,
            generation: 0,
            score: 0.0,
        }
    }

    pub fn random(id: String, depth: usize, rng: &mut impl Rng) -> Self {
        let mut operations = Vec::new();
        for _ in 0..depth {
            let weights = vec![rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5];
            let bias = rng.gen::<f64>() - 0.5;
            operations.push(Operation::Linear { weights, bias });
        }
        Self::new(id, operations)
    }

    pub fn execute(&self, representation: &Representation) -> Result<Vec<f64>> {
        let mut current = representation.input_space.clone();
        for op in &self.operations {
            current = op.apply(&current)?;
        }
        Ok(current)
    }

    pub fn score(&self) -> i32 {
        (self.score * 1000.0) as i32
    }

    pub fn set_score(&mut self, score: f64) {
        self.score = score.clamp(0.0, 1.0);
    }

    pub fn mutate(&mut self, rng: &mut impl Rng) {
        for op in &mut self.operations {
            op.mutate(rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_operation() {
        let op = Operation::Linear {
            weights: vec![1.0, 2.0],
            bias: 0.5,
        };
        let input = vec![1.0, 2.0];
        let output = op.apply(&input).unwrap();
        assert!(output.len() > 0);
    }

    #[test]
    fn test_structure_creation() {
        let ops = vec![Operation::Linear {
            weights: vec![1.0],
            bias: 0.0,
        }];
        let structure = ComputationalStructure::new("test".to_string(), ops);
        assert_eq!(structure.generation, 0);
    }

    #[test]
    fn test_structure_execution() {
        let ops = vec![Operation::Linear {
            weights: vec![1.0, 2.0],
            bias: 0.5,
        }];
        let structure = ComputationalStructure::new("test".to_string(), ops);
        let repr = Representation::new(
            "test_repr".to_string(),
            vec![1.0, 2.0],
            vec![5.5],
        );
        let result = structure.execute(&repr).unwrap();
        assert!(result.len() > 0);
    }

    #[test]
    fn test_structure_mutation() {
        let mut rng = rand::thread_rng();
        let ops = vec![Operation::Linear {
            weights: vec![1.0, 2.0],
            bias: 0.5,
        }];
        let mut structure = ComputationalStructure::new("test".to_string(), ops);
        structure.mutate(&mut rng);
        assert!(structure.score >= 0.0);
    }
}
