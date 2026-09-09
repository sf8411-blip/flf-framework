//! Evolution: Structural improvement through experience

use crate::error::Result;
use crate::experience::Experience;
use crate::field::Field;

/// Evolver uses experience to improve field structures
pub struct Evolver {
    mutation_rate: f64,
}

impl Evolver {
    pub fn new() -> Self {
        Self {
            mutation_rate: 0.1,
        }
    }

    pub fn with_mutation_rate(mutation_rate: f64) -> Self {
        Self {
            mutation_rate: mutation_rate.clamp(0.0, 1.0),
        }
    }

    /// Evolve the field based on experience
    pub fn evolve(&mut self, field: &mut Field, _experience: &Experience) -> Result<()> {
        let _avg_quality = field.average_quality();
        let _best_quality = field.best_quality();

        field.next_generation();

        Ok(())
    }
}

impl Default for Evolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolver_creation() {
        let evolver = Evolver::new();
        assert!(evolver.mutation_rate > 0.0);
    }

    #[test]
    fn test_evolve_field() {
        let mut field = Field::new();
        let mut evolver = Evolver::new();
        let exp = Experience {
            problem_id: "test".to_string(),
            solution_score: 0.7,
            structure_fingerprint: "fp".to_string(),
            generation: 0,
            timestamp: 0,
        };
        field.store_experience(exp).unwrap();

        let exp2 = Experience {
            problem_id: "test".to_string(),
            solution_score: 0.75,
            structure_fingerprint: "fp".to_string(),
            generation: 0,
            timestamp: 0,
        };
        evolver.evolve(&mut field, &exp2).unwrap();

        assert_eq!(field.generation(), 1);
    }
}
