//! Validation of computational structures against problems

use crate::error::Result;
use crate::problem::Problem;
use crate::structure::ComputationalStructure;

/// Validator checks if structures correctly solve problems
pub struct Validator {
    threshold: f64,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            threshold: 0.5,
        }
    }

    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Validate that a structure can solve the problem
    pub fn validate(&self, structure: &ComputationalStructure, problem: &Problem) -> Result<bool> {
        let repr = problem.represent()?;
        let output = structure.execute(&repr)?;

        let error = self.calculate_error(&output, &problem.expected_output);
        let success = error < (1.0 - self.threshold);

        Ok(success)
    }

    /// Calculate normalized error between actual and expected outputs
    fn calculate_error(&self, actual: &[f64], expected: &[f64]) -> f64 {
        if actual.is_empty() && expected.is_empty() {
            return 0.0;
        }

        let max_len = actual.len().max(expected.len());
        if max_len == 0 {
            return 0.0;
        }

        let mut total_error = 0.0;
        for i in 0..max_len {
            let a = actual.get(i).copied().unwrap_or(0.0);
            let e = expected.get(i).copied().unwrap_or(0.0);
            total_error += (a - e).abs();
        }

        (total_error / max_len as f64).min(1.0)
    }

    /// Score a structure's performance
    pub fn score(&self, structure: &ComputationalStructure, problem: &Problem) -> Result<f64> {
        let repr = problem.represent()?;
        let output = structure.execute(&repr)?;
        let error = self.calculate_error(&output, &problem.expected_output);
        Ok(1.0 - error)
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::Operation;

    #[test]
    fn test_validator_creation() {
        let validator = Validator::new();
        assert!(validator.threshold > 0.0);
    }

    #[test]
    fn test_error_calculation() {
        let validator = Validator::new();
        let actual = vec![1.0, 2.0];
        let expected = vec![1.0, 2.0];
        let error = validator.calculate_error(&actual, &expected);
        assert!(error < 0.01);
    }

    #[test]
    fn test_score_structure() {
        let validator = Validator::new();
        let ops = vec![Operation::Linear {
            weights: vec![1.0],
            bias: 0.0,
        }];
        let structure = ComputationalStructure::new("test".to_string(), ops);
        let problem = Problem::new("prob".to_string(), vec![1.0], vec![1.0]);

        let score = validator.score(&structure, &problem).unwrap();
        assert!(score >= 0.0 && score <= 1.0);
    }
}
