//! Experience: Learned knowledge from problem solving

use crate::error::Result;
use crate::problem::Problem;
use crate::structure::ComputationalStructure;
use serde::{Deserialize, Serialize};

/// Experience represents what was learned from solving a problem
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub problem_id: String,
    pub solution_score: f64,
    pub structure_fingerprint: String,
    pub generation: usize,
    pub timestamp: u64,
}

impl Experience {
    pub fn from_execution(
        problem: &Problem,
        structure: &ComputationalStructure,
        result: &[f64],
    ) -> Result<Self> {
        let error: f64 = result
            .iter()
            .zip(&problem.expected_output)
            .map(|(r, e)| (r - e).abs())
            .sum();

        let normalized_error = (error / result.len().max(1) as f64).min(1.0);
        let score = 1.0 - normalized_error;

        let fingerprint = format!("{:?}", structure.operations);

        Ok(Self {
            problem_id: problem.id.clone(),
            solution_score: score,
            structure_fingerprint: fingerprint,
            generation: structure.generation,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }

    pub fn quality(&self) -> f64 {
        self.solution_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::Operation;

    #[test]
    fn test_experience_creation() {
        let problem = Problem::new("test".to_string(), vec![1.0], vec![1.0]);
        let ops = vec![Operation::Linear {
            weights: vec![1.0],
            bias: 0.0,
        }];
        let structure = ComputationalStructure::new("str".to_string(), ops);
        let result = vec![0.9];

        let experience = Experience::from_execution(&problem, &structure, &result).unwrap();
        assert!(experience.solution_score > 0.0);
        assert_eq!(experience.problem_id, "test");
    }

    #[test]
    fn test_experience_quality() {
        let problem = Problem::new("test".to_string(), vec![1.0], vec![1.0]);
        let ops = vec![Operation::Linear {
            weights: vec![1.0],
            bias: 0.0,
        }];
        let structure = ComputationalStructure::new("str".to_string(), ops);
        let result = vec![1.0];

        let experience = Experience::from_execution(&problem, &structure, &result).unwrap();
        assert!(experience.quality() > 0.9);
    }
}
