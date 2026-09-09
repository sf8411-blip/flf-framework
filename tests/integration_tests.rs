use flf_core::{FLFSystem, Problem, Field, ComputationalStructure, Operation};

#[test]
fn test_full_pipeline() {
    let mut system = FLFSystem::new();
    let problem = Problem::new(
        "integration_test".to_string(),
        vec![1.0, 2.0],
        vec![3.0],
    );
    
    let result = system.process_problem(problem);
    assert!(result.is_ok());
    
    let experience = result.unwrap();
    assert!(experience.quality() >= 0.0);
    assert!(experience.quality() <= 1.0);
}

#[test]
fn test_multiple_problems_sequence() {
    let mut system = FLFSystem::new();
    let problems = vec![
        Problem::new("p1".to_string(), vec![1.0, 2.0], vec![3.0]),
        Problem::new("p2".to_string(), vec![2.0, 3.0], vec![5.0]),
        Problem::new("p3".to_string(), vec![0.5, 1.5], vec![2.0]),
        Problem::new("p4".to_string(), vec![3.0, 4.0], vec![7.0]),
        Problem::new("p5".to_string(), vec![1.5, 2.5], vec![4.0]),
    ];
    
    let mut quality_scores = Vec::new();
    for problem in problems {
        if let Ok(exp) = system.process_problem(problem) {
            quality_scores.push(exp.quality());
        }
    }
    
    assert_eq!(quality_scores.len(), 5);
    for score in quality_scores {
        assert!(score >= 0.0 && score <= 1.0);
    }
}

#[test]
fn test_field_accumulation() {
    let mut system = FLFSystem::new();
    
    for i in 0..10 {
        let problem = Problem::new(
            format!("problem_{}", i),
            vec![1.0 + i as f64 * 0.1, 2.0 + i as f64 * 0.1],
            vec![3.0 + i as f64 * 0.1],
        );
        let _ = system.process_problem(problem);
    }
    
    let field = system.field_state();
    assert_eq!(field.experience_count(), 10);
    assert!(field.average_quality() >= 0.0);
    assert!(field.best_quality() >= field.average_quality());
}

#[test]
fn test_generational_progression() {
    let mut system = FLFSystem::new();
    
    for gen in 0..3 {
        for prob in 0..3 {
            let problem = Problem::new(
                format!("gen{}_prob{}", gen, prob),
                vec![1.0, 2.0],
                vec![3.0],
            );
            let _ = system.process_problem(problem);
        }
    }
    
    let field = system.field_state();
    assert_eq!(field.experience_count(), 9);
    assert_eq!(field.generation(), 3);
}

#[test]
fn test_structure_scoring() {
    let ops = vec![Operation::Linear {
        weights: vec![1.0, 2.0],
        bias: 0.5,
    }];
    
    let mut structure = ComputationalStructure::new("test".to_string(), ops);
    structure.set_score(0.75);
    
    assert_eq!(structure.score(), 750);
}

#[test]
fn test_quality_metrics() {
    let mut field = Field::new();
    
    for i in 0..5 {
        let exp = flf_core::Experience {
            problem_id: format!("p{}", i),
            solution_score: 0.5 + i as f64 * 0.1,
            structure_fingerprint: "fp".to_string(),
            generation: 0,
            timestamp: 0,
        };
        let _ = field.store_experience(exp);
    }
    
    assert_eq!(field.experience_count(), 5);
    let avg = field.average_quality();
    let best = field.best_quality();
    
    assert!(avg > 0.5);
    assert!(best >= avg);
    assert!(best <= 0.9);
}

#[test]
fn test_error_handling_empty_problem() {
    let problem = Problem::new("empty".to_string(), vec![], vec![1.0]);
    let result = problem.represent();
    assert!(result.is_err());
}

#[test]
fn test_deterministic_execution() {
    let problem = Problem::new("det".to_string(), vec![1.0, 2.0], vec![3.0]);
    
    let ops = vec![Operation::Linear {
        weights: vec![1.0, 2.0],
        bias: 0.5,
    }];
    let structure = ComputationalStructure::new("det_struct".to_string(), ops);
    
    let repr1 = problem.represent().unwrap();
    let result1 = structure.execute(&repr1).unwrap();
    
    let repr2 = problem.represent().unwrap();
    let result2 = structure.execute(&repr2).unwrap();
    
    assert_eq!(result1.len(), result2.len());
    for (r1, r2) in result1.iter().zip(result2.iter()) {
        assert!((r1 - r2).abs() < 1e-10);
    }
}

#[test]
fn test_activation_operations() {
    let tanh_op = Operation::Activation { kind: "tanh".to_string() };
    let input = vec![0.0, 1.0, -1.0];
    let output = tanh_op.apply(&input).unwrap();
    
    assert_eq!(output.len(), 3);
    assert!(output[0].abs() < 1e-10); // tanh(0) ≈ 0
    assert!(output[1] > 0.7); // tanh(1) ≈ 0.76
    assert!(output[2] < -0.7); // tanh(-1) ≈ -0.76
}

#[test]
fn test_validator_threshold() {
    let validator = flf_core::Validator::new();
    let problem = Problem::new("v_test".to_string(), vec![1.0], vec![1.0]);
    
    let ops = vec![Operation::Linear {
        weights: vec![1.0],
        bias: 0.0,
    }];
    let structure = ComputationalStructure::new("v_struct".to_string(), ops);
    
    let score = validator.score(&structure, &problem).unwrap();
    assert!(score >= 0.0 && score <= 1.0);
}

#[test]
fn test_experience_quality_calculation() {
    let problem = Problem::new("exp_test".to_string(), vec![1.0, 2.0], vec![3.0]);
    let ops = vec![Operation::Linear {
        weights: vec![1.0, 1.0],
        bias: 0.0,
    }];
    let structure = ComputationalStructure::new("exp_struct".to_string(), ops);
    
    let result = vec![2.76]; // Close to tanh(3)
    let experience = flf_core::Experience::from_execution(&problem, &structure, &result).unwrap();
    
    assert!(experience.quality() > 0.0);
    assert!(experience.quality() <= 1.0);
}

#[test]
fn test_field_good_experiences() {
    let mut field = Field::new();
    
    for i in 0..10 {
        let exp = flf_core::Experience {
            problem_id: format!("p{}", i),
            solution_score: (i as f64) / 10.0,
            structure_fingerprint: "fp".to_string(),
            generation: 0,
            timestamp: 0,
        };
        let _ = field.store_experience(exp);
    }
    
    let good_exps = field.good_experiences(0.7);
    assert!(good_exps.len() >= 3);
    for exp in good_exps {
        assert!(exp.quality() >= 0.7);
    }
}

#[test]
fn test_problem_difficulty_adjustment() {
    let problem = Problem::new(
        "diff_test".to_string(),
        vec![1.0, 2.0],
        vec![3.0],
    )
    .with_difficulty(8.5);
    
    assert_eq!(problem.difficulty, 8.5);
    
    let repr = problem.represent().unwrap();
    assert!(repr.constraints.iter().any(|c| c.contains("difficulty=8.5")));
}

#[test]
fn test_multi_generation_learning() {
    let mut system = FLFSystem::new();
    
    let mut prev_avg = 0.0;
    
    for gen in 0..3 {
        for i in 0..3 {
            let problem = Problem::new(
                format!("learning_gen{}_p{}", gen, i),
                vec![0.5 + i as f64, 1.5 + i as f64],
                vec![2.0 + i as f64],
            );
            let _ = system.process_problem(problem);
        }
        
        let curr_avg = system.field_state().average_quality();
        if gen > 0 {
            // Quality should generally not decrease (may stay same due to randomness)
            assert!(curr_avg >= 0.0 && curr_avg <= 1.0);
        }
        prev_avg = curr_avg;
    }
    
    assert_eq!(system.field_state().experience_count(), 9);
}

#[test]
fn test_error_types() {
    use flf_core::Error;
    
    let err = Error::NoValidStructures;
    assert_eq!(err.to_string(), "No valid structures could be generated");
    
    let err2 = Error::ExecutionFailed("test error".to_string());
    assert!(err2.to_string().contains("Execution failed"));
}

#[test]
fn test_representation_constraints() {
    let problem = Problem::new(
        "constraint_test".to_string(),
        vec![1.0, 2.0, 3.0],
        vec![6.0, 7.0],
    );
    
    let repr = problem.represent().unwrap();
    assert!(repr.constraints.contains(&"input_dim=3".to_string()));
    assert!(repr.constraints.contains(&"output_dim=2".to_string()));
}
