//! Digital Petri Dish: A concrete demonstration of FLF
//!
//! This example shows the FLF system solving a sequence of problems,
//! learning from each, and improving over generations.

use flf_core::{FLFSystem, Problem};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Digital Petri Dish - FLF Framework Demo            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let mut system = FLFSystem::new();
    let num_generations = 5;

    for generation in 0..num_generations {
        println!("─── Generation {} ───", generation);

        let problems = create_problem_sequence(generation);

        for (idx, problem) in problems.iter().enumerate() {
            match system.process_problem(problem.clone()) {
                Ok(experience) => {
                    println!(
                        "  Problem {}: score = {:.4}",
                        idx,
                        experience.quality()
                    );
                }
                Err(e) => {
                    eprintln!("  Problem {}: ERROR - {}", idx, e);
                }
            }
        }

        let field = system.field_state();
        println!(
            "  Generation {}: avg_quality={:.4}, best_quality={:.4}",
            generation,
            field.average_quality(),
            field.best_quality()
        );
        println!();
    }

    println!("╔════════════════════════════════════════════════════════════╗");
    let field = system.field_state();
    println!("║                        Final Results                        ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║ Total Experiences: {:<48} ║", field.experience_count());
    println!("║ Average Quality:   {:<48} ║", format!("{:.4}", field.average_quality()));
    println!("║ Best Quality:      {:<48} ║", format!("{:.4}", field.best_quality()));
    println!("╚════════════════════════════════════════════════════════════╝");

    Ok(())
}

fn create_problem_sequence(generation: usize) -> Vec<Problem> {
    vec![
        Problem::new(
            format!("gen{}_prob0", generation),
            vec![1.0, 2.0],
            vec![3.0],
        ),
        Problem::new(
            format!("gen{}_prob1", generation),
            vec![0.5, 1.5],
            vec![2.0],
        ),
        Problem::new(
            format!("gen{}_prob2", generation),
            vec![2.0, 3.0],
            vec![5.0],
        ),
    ]
}
