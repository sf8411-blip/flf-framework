## Field-Level Forming (FLF) Framework

A Rust implementation of the Field-Level Forming hypothesis, demonstrating how computational structures can evolve and learn from experience.

### Quick Start

```bash
cargo build --release
cargo test --all
cargo run --release --example digital_petri_dish
```

### Project Structure

- **flf-core**: Core FLF implementation
  - `field.rs` - Field storage and structure generation
  - `structure.rs` - Computational structures (operations and execution)
  - `problem.rs` - Problem representation
  - `experience.rs` - Experience generation from solutions
  - `validation.rs` - Structure validation
  - `evolution.rs` - Structural improvement through experience

- **flf-examples**: Example applications
  - `digital_petri_dish.rs` - Demonstrates FLF solving a problem sequence

### FLF Pipeline

1. **Represent** - Convert problem to computational form
2. **Generate** - Create multiple computational structures
3. **Validate** - Check structure validity independently
4. **Execute** - Run best structure on problem
5. **Experience** - Generate knowledge from results
6. **Store** - Keep experience in field
7. **Evolve** - Update structures based on experience
8. **Repeat** - Apply to next problem

### Success Criteria (Phase 1)

- [x] Represent problems computationally
- [x] Generate multiple candidate structures
- [x] Validate structures independently
- [x] Execute structures and capture results
- [x] Generate and store experiences
- [x] Evolve field based on experience
- [x] Track measurable improvement
- [x] Demonstrate on problem sequence

### Architecture

```
FLFSystem
├── Field
│   ├── Experiences
│   ├── Structures
│   └── Generation counter
├── Validator
│   └── Scoring logic
└── Evolver
    └── Mutation strategy
```

### Scientific Validity

The implementation emphasizes:
- **Correctness** over optimization
- **Reproducibility** through deterministic testing
- **Measurable improvement** via experience tracking
- **Independent validation** of solutions
