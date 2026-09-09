//! Field-Level Forming (FLF) Framework
//!
//! A framework for representing, evolving, and learning from computational structures.
//! This implementation demonstrates the FLF hypothesis through concrete, executable code.

pub mod error;
pub mod field;
pub mod structure;
pub mod validation;
pub mod experience;
pub mod evolution;
pub mod problem;

pub use error::{Error, Result};
pub use field::Field;
pub use structure::ComputationalStructure;
pub use validation::Validator;
pub use experience::Experience;
pub use evolution::Evolver;
pub use problem::Problem;

/// FLF System: The main orchestrator for field-level forming
pub struct FLFSystem {
    field: Field,
    evolver: Evolver,
    validator: Validator,
}

impl FLFSystem {
    /// Create a new FLF system with default parameters
    pub fn new() -> Self {
        Self {
            field: Field::new(),
            evolver: Evolver::new(),
            validator: Validator::new(),
        }
    }

    /// Process a problem through the FLF pipeline
    pub fn process_problem(&mut self, problem: Problem) -> Result<Experience> {
        // 1. Represent the problem
        let representation = problem.represent()?;

        // 2. Generate multiple computational structures
        let mut structures = self.field.generate_structures(&representation)?;

        // 3. Validate each structure independently
        let mut valid_structures = Vec::new();
        for mut structure in structures {
            let score = self.validator.score(&structure, &problem)?;
            structure.set_score(score);
            if self.validator.validate(&structure, &problem)? {
                valid_structures.push(structure);
            }
        }

        if valid_structures.is_empty() {
            return Err(Error::NoValidStructures);
        }

        // 4. Execute the best structure
        let best_structure = valid_structures.iter().max_by_key(|s| s.score()).unwrap();
        let result = best_structure.execute(&representation)?;

        // 5. Generate experience from results
        let experience = Experience::from_execution(&problem, best_structure, &result)?;

        // 6. Store experience in the field
        self.field.store_experience(experience.clone())?;

        // 7. Update field and structures based on experience
        self.evolver.evolve(&mut self.field, &experience)?;

        Ok(experience)
    }

    /// Get the current state of the field
    pub fn field_state(&self) -> &Field {
        &self.field
    }
}

impl Default for FLFSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flf_system_creation() {
        let system = FLFSystem::new();
        assert_eq!(system.field_state().experience_count(), 0);
    }

    #[test]
    fn test_flf_system_processes_problem() {
        let mut system = FLFSystem::new();
        let problem = Problem::new(
            "test_problem".to_string(),
            vec![1.0, 2.0],
            vec![3.0],
        );
        let result = system.process_problem(problem);
        assert!(result.is_ok());
    }

    #[test]
    fn test_flf_system_learns() {
        let mut system = FLFSystem::new();
        let problems = vec![
            Problem::new("p1".to_string(), vec![1.0, 2.0], vec![3.0]),
            Problem::new("p2".to_string(), vec![2.0, 3.0], vec![5.0]),
            Problem::new("p3".to_string(), vec![0.5, 1.5], vec![2.0]),
        ];
        
        let mut experiences = Vec::new();
        for problem in problems {
            if let Ok(exp) = system.process_problem(problem) {
                experiences.push(exp);
            }
        }
        
        assert_eq!(experiences.len(), 3);
    }
}
