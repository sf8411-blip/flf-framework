//! The Field: Storage and generation of computational structures

use crate::error::Result;
use crate::experience::Experience;
use crate::problem::Representation;
use crate::structure::ComputationalStructure;
use std::collections::HashMap;

/// The Field stores experiences and generates structures
pub struct Field {
    experiences: Vec<Experience>,
    structures: HashMap<String, ComputationalStructure>,
    generation: usize,
}

impl Field {
    pub fn new() -> Self {
        Self {
            experiences: Vec::new(),
            structures: HashMap::new(),
            generation: 0,
        }
    }

    /// Generate multiple computational structures for a representation
    pub fn generate_structures(&mut self, _representation: &Representation) -> Result<Vec<ComputationalStructure>> {
        let mut rng = rand::thread_rng();
        let mut structures = Vec::new();

        for i in 0..5 {
            let depth = 1 + (i % 3);
            let mut structure = ComputationalStructure::random(
                format!("struct_{}_{}", self.generation, i),
                depth,
                &mut rng,
            );
            structure.generation = self.generation;
            structures.push(structure);
        }

        Ok(structures)
    }

    /// Store an experience in the field
    pub fn store_experience(&mut self, experience: Experience) -> Result<()> {
        self.experiences.push(experience);
        Ok(())
    }

    /// Get all stored experiences
    pub fn experiences(&self) -> &[Experience] {
        &self.experiences
    }

    /// Get experiences of a certain quality threshold
    pub fn good_experiences(&self, threshold: f64) -> Vec<&Experience> {
        self.experiences
            .iter()
            .filter(|exp| exp.quality() >= threshold)
            .collect()
    }

    /// Increment generation counter
    pub fn next_generation(&mut self) {
        self.generation += 1;
    }

    /// Get current generation number
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Get experience count
    pub fn experience_count(&self) -> usize {
        self.experiences.len()
    }

    /// Get average solution quality
    pub fn average_quality(&self) -> f64 {
        if self.experiences.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.experiences.iter().map(|e| e.quality()).sum();
        sum / self.experiences.len() as f64
    }

    /// Get best solution quality
    pub fn best_quality(&self) -> f64 {
        self.experiences
            .iter()
            .map(|e| e.quality())
            .fold(0.0, f64::max)
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::new();
        assert_eq!(field.generation(), 0);
        assert_eq!(field.experience_count(), 0);
    }

    #[test]
    fn test_structure_generation() {
        let mut field = Field::new();
        let repr = Representation::new("test".to_string(), vec![1.0, 2.0], vec![3.0]);
        let structures = field.generate_structures(&repr).unwrap();
        assert_eq!(structures.len(), 5);
    }

    #[test]
    fn test_experience_storage() {
        let mut field = Field::new();
        let exp = Experience {
            problem_id: "test".to_string(),
            solution_score: 0.8,
            structure_fingerprint: "fp".to_string(),
            generation: 0,
            timestamp: 0,
        };
        field.store_experience(exp).unwrap();
        assert_eq!(field.experience_count(), 1);
    }

    #[test]
    fn test_generation_increment() {
        let mut field = Field::new();
        assert_eq!(field.generation(), 0);
        field.next_generation();
        assert_eq!(field.generation(), 1);
    }
}
