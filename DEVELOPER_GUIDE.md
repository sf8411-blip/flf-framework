# FLF Framework - Developer Guide

## 🚀 Quick Start for Developers

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- Git
- Text editor or IDE (VS Code recommended with rust-analyzer extension)

### Initial Setup

```bash
# Clone repository
git clone https://github.com/sf8411-blip/flf-framework
cd flf-framework

# Verify installation
cargo build
cargo test --all
cargo run --release --example digital_petri_dish
```

---

## 📁 Project Structure Guide

```
flf-framework/
├── Cargo.toml                      # Workspace configuration
├── README.md                       # User documentation
├── COMPLETION_SUMMARY.md          # Project completion overview
├── PHASE_1_REPORT.md             # Detailed Phase 1 analysis
├── PHASE_2_PLAN.md               # Phase 2 development roadmap
├── DEVELOPER_GUIDE.md            # This file
├── CONTRIBUTING.md               # Contribution guidelines
├── CHANGELOG.md                  # Version history
├── LICENSE                       # MIT License
├── .gitignore                    # Git ignore configuration
│
├── .github/
│   └── workflows/
│       └── ci.yml               # CI/CD pipeline configuration
│
├── flf-core/                    # Core library (production code)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Main entry point & FLFSystem
│       ├── error.rs            # Error type definitions
│       ├── problem.rs          # Problem & Representation types
│       ├── structure.rs        # Operation & ComputationalStructure
│       ├── validation.rs       # Validator implementation
│       ├── experience.rs       # Experience type & logic
│       ├── field.rs            # Field storage & generation
│       └── evolution.rs        # Evolution mechanism
│
├── flf-examples/               # Example applications
│   ├── Cargo.toml
│   └── examples/
│       └── digital_petri_dish.rs  # Main demo application
│
└── tests/                      # Integration tests
    ├── integration_tests.rs    # Comprehensive test suite
    └── README.md              # Testing documentation
```

---

## 🔄 Development Workflow

### Step 1: Create a Feature Branch

```bash
# Update main branch
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/your-feature-name

# Example
git checkout -b feature/add-gaussian-mutation
```

### Step 2: Make Your Changes

**Edit files in `flf-core/src/`:**

```rust
// Example: Adding a new operation type
pub enum Operation {
    Linear { weights: Vec<f64>, bias: f64 },
    Activation { kind: String },
    Compose { left: Box<Operation>, right: Box<Operation> },
    // NEW: Your new operation
    CustomOp { param: f64 },
}
```

**Add comprehensive tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_op() {
        let op = Operation::CustomOp { param: 0.5 };
        let result = op.apply(&[1.0, 2.0]).unwrap();
        assert_eq!(result.len(), 2);
    }
}
```

### Step 3: Run Tests Locally

```bash
# Run all tests
cargo test --all

# Run with output visible
cargo test --all -- --nocapture

# Run specific test
cargo test test_custom_op -- --nocapture

# Test in release mode
cargo test --all --release
```

### Step 4: Code Quality Checks

```bash
# Format code
cargo fmt

# Check formatting (won't modify)
cargo fmt -- --check

# Lint code
cargo clippy --all-targets --all-features -- -D warnings

# Build release version
cargo build --release

# Generate and view documentation
cargo doc --open
```

### Step 5: Commit Changes

```bash
# Stage all changes
git add .

# Commit with descriptive message
git commit -m "feat: Add custom operation type to Operation enum"

# View git log
git log --oneline -5
```

### Step 6: Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Then go to GitHub and create a Pull Request
```

---

## 🧠 Understanding the Codebase

### Core Architecture

**The 7-Step FLF Pipeline:**

```
1. REPRESENT
   Problem → Representation
   
2. GENERATE
   Representation → [Struct1, Struct2, ...]
   
3. VALIDATE
   Each structure scored independently
   
4. EXECUTE
   Best structure runs on problem
   
5. EXPERIENCE
   Results converted to Experience
   
6. STORE
   Experience added to Field
   
7. EVOLVE
   Field updated for next iteration
```

### Key Data Types

**Problem Solving:**
```rust
// Input to the system
pub struct Problem {
    pub id: String,
    pub input_space: Vec<f64>,      // Problem parameters
    pub expected_output: Vec<f64>,  // Desired solution
    pub difficulty: f64,             // Problem complexity
}

// Computational representation
pub struct Representation {
    pub id: String,
    pub input_space: Vec<f64>,
    pub expected_output: Vec<f64>,
    pub constraints: Vec<String>,   // Problem constraints
}
```

**Solution Structures:**
```rust
// Basic computational operation
pub enum Operation {
    Linear { weights: Vec<f64>, bias: f64 },
    Activation { kind: String },
    Compose { left: Box<Operation>, right: Box<Operation> },
}

// Collection of operations that solves a problem
pub struct ComputationalStructure {
    pub id: String,
    pub operations: Vec<Operation>,
    pub generation: usize,  // Which generation created this
    pub score: f64,         // Quality score [0.0, 1.0]
}
```

**Learning:**
```rust
// What was learned from execution
pub struct Experience {
    pub problem_id: String,
    pub solution_score: f64,           // How well it solved
    pub structure_fingerprint: String, // What structure was used
    pub generation: usize,             // When it was learned
    pub timestamp: u64,                // Unix timestamp
}

// Storage for all experiences
pub struct Field {
    experiences: Vec<Experience>,      // All learning history
    structures: HashMap<String, ComputationalStructure>,
    generation: usize,                 // Current evolution stage
}
```

---

## 💡 Adding New Features

### Example 1: Adding a New Operation Type

**File: `flf-core/src/structure.rs`**

```rust
// 1. Add to enum
pub enum Operation {
    Linear { weights: Vec<f64>, bias: f64 },
    Activation { kind: String },
    Compose { left: Box<Operation>, right: Box<Operation> },
    // NEW OPERATION TYPE
    BatchNorm { scale: f64, offset: f64 },
}

// 2. Implement apply method
impl Operation {
    pub fn apply(&self, input: &[f64]) -> Result<Vec<f64>> {
        match self {
            // ... existing cases ...
            Operation::BatchNorm { scale, offset } => {
                // Normalize input
                let mean = input.iter().sum::<f64>() / input.len() as f64;
                let variance = input.iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>() / input.len() as f64;
                let std_dev = variance.sqrt() + 1e-6;
                
                Ok(input.iter()
                    .map(|x| (x - mean) / std_dev * scale + offset)
                    .collect())
            }
        }
    }
}

// 3. Add tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_batch_norm() {
        let op = Operation::BatchNorm { scale: 1.0, offset: 0.0 };
        let input = vec![1.0, 2.0, 3.0];
        let output = op.apply(&input).unwrap();
        
        assert_eq!(output.len(), 3);
        // Mean should be ~0
        let mean = output.iter().sum::<f64>() / 3.0;
        assert!(mean.abs() < 0.01);
    }
}
```

**File: `tests/integration_tests.rs`**

```rust
#[test]
fn test_batch_norm_operation_integration() {
    let ops = vec![
        Operation::Linear { weights: vec![1.0, 2.0], bias: 0.0 },
        Operation::BatchNorm { scale: 1.0, offset: 0.0 },
    ];
    let structure = ComputationalStructure::new("bn_test".to_string(), ops);
    let problem = Problem::new("bn_prob".to_string(), vec![1.0, 2.0], vec![3.0]);
    
    let result = structure.execute(&problem.represent().unwrap()).unwrap();
    assert!(!result.is_empty());
}
```

### Example 2: Extending the Validator

**File: `flf-core/src/validation.rs`**

```rust
pub struct Validator {
    threshold: f64,
    // NEW: Additional validation strategy
    enable_stability_check: bool,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            threshold: 0.5,
            enable_stability_check: false,
        }
    }
    
    // NEW: Stability checking method
    pub fn check_stability(&self, structure: &ComputationalStructure) -> Result<bool> {
        // Run structure multiple times with same input
        // Check if outputs are consistent
        Ok(true)
    }
    
    pub fn validate(&self, structure: &ComputationalStructure, problem: &Problem) -> Result<bool> {
        let repr = problem.represent()?;
        let output = structure.execute(&repr)?;
        
        let error = self.calculate_error(&output, &problem.expected_output);
        let score_ok = error < (1.0 - self.threshold);
        
        // NEW: Additional stability check
        let stability_ok = if self.enable_stability_check {
            self.check_stability(structure)?
        } else {
            true
        };
        
        Ok(score_ok && stability_ok)
    }
}
```

---

## 🧪 Testing Best Practices

### Writing Good Tests

```rust
#[test]
fn test_descriptive_name_describes_what_is_tested() {
    // ARRANGE: Set up test data
    let input = vec![1.0, 2.0];
    let expected = vec![3.0];
    let problem = Problem::new("test".to_string(), input, expected);
    
    // ACT: Perform the action
    let result = problem.represent();
    
    // ASSERT: Verify the result
    assert!(result.is_ok());
    let repr = result.unwrap();
    assert_eq!(repr.input_space.len(), 2);
    assert_eq!(repr.expected_output.len(), 1);
}
```

### Test Organization

```rust
// Unit tests go in the same file as code
// flf-core/src/problem.rs:

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_problem_creation() { }
    
    #[test]
    fn test_problem_representation() { }
    
    #[test]
    fn test_representation_errors() { }
}

// Integration tests go in tests/ directory
// tests/integration_tests.rs:

#[test]
fn test_full_pipeline() { }

#[test]
fn test_multi_generation_learning() { }
```

### Running Tests

```bash
# All tests
cargo test --all

# Show println! output
cargo test --all -- --nocapture

# Only failing tests
cargo test --all -- --test-threads=1

# Specific test
cargo test test_problem_creation -- --exact --nocapture

# In release mode (faster)
cargo test --release --all
```

---

## 📚 Documentation Guidelines

### Writing Doc Comments

```rust
/// Brief one-line description.
///
/// Longer explanation of what this does, when to use it, and any
/// important details or edge cases to be aware of.
///
/// # Arguments
///
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
///
/// Description of what is returned and what it means
///
/// # Errors
///
/// Description of what errors can occur and when
///
/// # Examples
///
/// ```
/// use flf_core::Problem;
///
/// let problem = Problem::new(
///     "test".to_string(),
///     vec![1.0, 2.0],
///     vec![3.0],
/// );
/// assert!(problem.represent().is_ok());
/// ```
pub fn example_function(param1: Type, param2: Type) -> Result<ReturnType> {
    // Implementation
}
```

### Viewing Documentation

```bash
# Generate docs for project
cargo doc

# Open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items --open
```

---

## 🎯 Common Development Tasks

### Debugging

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo run --example digital_petri_dish

# Detailed backtrace
RUST_BACKTRACE=full cargo run --example digital_petri_dish

# In tests
RUST_BACKTRACE=1 cargo test test_name -- --nocapture
```

### Performance Analysis

```bash
# Build release version
cargo build --release

# Time the example
time ./target/release/examples/digital_petri_dish

# With profiling (Linux)
perf record ./target/release/examples/digital_petri_dish
perf report
```

### Cleaning Up

```bash
# Remove build artifacts
cargo clean

# Format all code
cargo fmt

# Automatically fix clippy warnings
cargo clippy --fix --allow-no-vcs
```

### Updating Dependencies

```bash
# Check for updates
cargo outdated

# Update all dependencies
cargo update

# Update specific crate
cargo update -p serde
```

---

## 🔧 Code Style Guidelines

### Formatting

```rust
// Use cargo fmt - no custom rules needed
// Line length: 100 characters (flexible for URLs)

// Good
let very_long_variable_name = some_function(param1, param2, param3);

// Also good - multiple lines
let result = some_complex_function(
    parameter_one,
    parameter_two,
    parameter_three,
);
```

### Naming Conventions

```rust
// Constants - UPPER_SNAKE_CASE
const MAX_ITERATIONS: usize = 1000;

// Functions and variables - snake_case
fn process_problem() { }
let field_state = system.field_state();

// Types - PascalCase
struct ComputationalStructure { }
pub enum Operation { }

// Private items - _leading_underscore (optional)
fn _internal_helper() { }
```

### Error Handling

```rust
// Always use Result<T> for fallible operations
fn risky_operation() -> Result<Value> {
    let data = get_data()
        .map_err(|_| Error::CustomError("message".to_string()))?;
    
    if data.is_invalid() {
        return Err(Error::ValidationFailed("reason".to_string()));
    }
    
    Ok(processed_data)
}

// Don't use unwrap() in library code
// It's okay in examples and tests
```

### Comments

```rust
// Single line comments for explanation
let result = calculate_value(); // Brief explanation

// Use doc comments for public items
/// This function does...
pub fn function() { }

// Use // for implementation details
// Calculate the mean of the input
let mean = input.iter().sum::<f64>() / input.len() as f64;
```

---

## 📋 Pre-Commit Checklist

Before committing, run:

```bash
# 1. Format code
cargo fmt

# 2. Run tests
cargo test --all

# 3. Check linting
cargo clippy --all-targets --all-features -- -D warnings

# 4. Build release
cargo build --release

# 5. Check git status
git status

# 6. Review changes
git diff

# 7. Stage and commit
git add .
git commit -m "type: description"
```

Or create a git hook:

```bash
#!/bin/bash
# .git/hooks/pre-commit
set -e

cargo fmt
cargo test --all
cargo clippy --all-targets --all-features -- -D warnings

echo "✅ All checks passed!"
```

---

## 🚀 Submitting a Pull Request

### Before Submitting

- [ ] Tests pass: `cargo test --all`
- [ ] Code formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Documentation complete
- [ ] Commit message clear
- [ ] Tests for new features
- [ ] No breaking changes to public API

### PR Description Template

```markdown
## Description
Brief description of what this PR does

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Related Issues
Closes #123

## Testing
Describe the testing done

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Code formatted
- [ ] No clippy warnings
```

---

## 📞 Getting Help

### Documentation
- API docs: `cargo doc --open`
- README: Main repository documentation
- CONTRIBUTING.md: Contribution guidelines
- PHASE_*.md: Phase-specific details

### Asking Questions
1. Check existing issues/discussions
2. Review code comments
3. Read relevant test cases
4. Create new discussion if needed

### Common Issues

**Compilation Error: "cannot find type `X`"**
- Make sure to add `use` statements
- Check module visibility (pub keyword)
- Verify dependency is in Cargo.toml

**Test Failures**
- Run `cargo test --all -- --nocapture` to see output
- Check if test expectations are correct
- Verify input data in test

**Formatting Issues**
- Run `cargo fmt` to auto-fix
- Check line length (100 chars)
- Use consistent indentation

---

## 🎓 Learning Resources

### Rust Documentation
- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

### Project Resources
- README.md - Project overview
- PHASE_1_REPORT.md - Architecture details
- Test files - Usage examples
- Doc comments - API documentation

---

## ✨ Tips and Tricks

### Useful cargo commands

```bash
# See all targets
cargo build --help

# Tree of dependencies
cargo tree

# Check without building
cargo check

# Expand macros
cargo expand

# Assembly code
cargo asm --all

# Size of binary
cargo bloat --release
```

### VS Code Extensions

- rust-analyzer - IDE features
- CodeLLDB - Debugging
- Better TOML - Cargo.toml support
- Rust Syntax - Syntax highlighting

---

**Happy Coding! 🚀**

For questions or issues, open a discussion on GitHub.
