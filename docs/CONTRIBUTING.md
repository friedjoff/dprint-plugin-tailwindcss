# Contributing Guide

Thank you for your interest in contributing to dprint-plugin-tailwindcss! This guide will help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Review Process](#review-process)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful, inclusive, and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Basic understanding of:
  - Rust programming
  - TailwindCSS
  - dprint plugin system (helpful but not required)

### Finding Something to Work On

1. **Browse Issues**: Check [GitHub Issues](https://github.com/friedjoff/dprint-plugin-tailwindcss/issues) for open tasks
2. **Good First Issues**: Look for issues labeled `good-first-issue`
3. **Feature Requests**: Issues labeled `enhancement` are feature requests
4. **Bugs**: Issues labeled `bug` need fixes

### Before You Start

- **Check existing issues/PRs** to avoid duplicate work
- **Comment on the issue** to let others know you're working on it
- **Ask questions** if anything is unclear

## Development Setup

1. **Fork and Clone**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/dprint-plugin-tailwindcss.git
   cd dprint-plugin-tailwindcss
   ```

2. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/friedjoff/dprint-plugin-tailwindcss.git
   ```

3. **Install Rust targets**:
   ```bash
   rustup target add wasm32-unknown-unknown
   rustup target add x86_64-unknown-linux-gnu
   ```

4. **Build and test**:
   ```bash
   cargo build --target x86_64-unknown-linux-gnu
   cargo test --lib --target x86_64-unknown-linux-gnu
   ```

## Making Changes

### 1. Create a Branch

```bash
git checkout -b feat/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

Branch naming conventions:
- `feat/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions/changes
- `chore/` - Maintenance tasks

### 2. Make Your Changes

Follow these guidelines:

#### Adding a New Feature

1. **Update relevant module** (e.g., `src/sorter.rs`, `src/parser.rs`)
2. **Add tests** in the same file or appropriate test module
3. **Update documentation** in `docs/` if needed
4. **Add examples** in README or docs

#### Fixing a Bug

1. **Write a failing test** that reproduces the bug
2. **Fix the bug** in the source code
3. **Verify the test passes**
4. **Add edge case tests** if applicable

#### Example: Adding a New File Format

```rust
// 1. Update FileFormat enum in src/parser.rs
pub enum FileFormat {
    Html,
    Jsx,
    Vue,
    Svelte,
    Astro,
    Php,  // New format
}

// 2. Add extension mapping
impl FileFormat {
    pub fn from_path(path: &str) -> Option<Self> {
        if path.ends_with(".php") {
            return Some(FileFormat::Php);
        }
        // ...
    }
}

// 3. Implement parser
fn parse_php(&self, content: &str) -> Vec<ClassMatch> {
    // Implementation
}

// 4. Add to parse() match
pub fn parse(&self, content: &str, format: FileFormat) -> Vec<ClassMatch> {
    match format {
        FileFormat::Php => self.parse_php(content),
        // ...
    }
}

// 5. Add tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_php_parsing() {
        // Test implementation
    }
}
```

### 3. Keep Your Branch Updated

```bash
git fetch upstream
git rebase upstream/main
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test --lib --target x86_64-unknown-linux-gnu

# Run specific module tests
cargo test --lib --target x86_64-unknown-linux-gnu sorter

# Run specific test
cargo test --lib --target x86_64-unknown-linux-gnu test_basic_class_sorting

# Run with output
cargo test --lib --target x86_64-unknown-linux-gnu -- --nocapture
```

### Writing Tests

**Unit Test Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_with_variants() {
        let input = "hover:bg-blue-500 bg-white";
        let result = sort_classes(input);
        assert_eq!(result, "bg-white hover:bg-blue-500");
    }
    
    #[test]
    fn test_edge_case_empty() {
        assert_eq!(sort_classes(""), "");
    }
}
```

**Integration Test Example**:
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_format_vue_file() {
        let input = r#"
<template>
  <div class="text-center p-4 mt-2">Content</div>
</template>
"#;
        let result = format_content(input, FileFormat::Vue);
        assert!(result.contains("mt-2 p-4 text-center"));
    }
}
```

### Test Requirements

All contributions must:
- ✅ Include tests for new functionality
- ✅ Maintain or improve test coverage
- ✅ Pass all existing tests
- ✅ Include edge case tests where applicable

## Submitting Changes

### 1. Commit Your Changes

Write clear, descriptive commit messages:

```bash
git commit -m "feat: add support for PHP file format"
git commit -m "fix: handle empty class attributes correctly"
git commit -m "docs: update configuration examples"
```

**Commit Message Format**:
```
<type>: <subject>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions/changes
- `refactor`: Code refactoring
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### 2. Push to Your Fork

```bash
git push origin feat/your-feature-name
```

### 3. Create a Pull Request

1. Go to [GitHub repository](https://github.com/friedjoff/dprint-plugin-tailwindcss)
2. Click "New Pull Request"
3. Select your fork and branch
4. Fill out the PR template:
   - **Title**: Clear, concise description
   - **Description**: 
     - What changes were made
     - Why these changes were necessary
     - How to test the changes
   - **Related Issues**: Link related issues with `Fixes #123` or `Relates to #456`
   - **Checklist**: Complete all items

**PR Template Example**:
```markdown
## Description
Added support for PHP file format to enable TailwindCSS class sorting in Blade templates.

## Changes
- Added `FileFormat::Php` enum variant
- Implemented `parse_php()` method
- Added `.php` extension detection
- Added comprehensive tests

## Testing
- Added 5 new tests for PHP parsing
- All existing tests pass
- Manually tested with real Blade templates

## Checklist
- [x] Tests added
- [x] Documentation updated
- [x] All tests pass
- [x] Code formatted with rustfmt
- [x] No clippy warnings

Fixes #123
```

## Code Style

### Rust Style Guidelines

1. **Use rustfmt**:
   ```bash
   cargo fmt
   ```

2. **Use clippy**:
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Follow Rust conventions**:
   - Use snake_case for functions and variables
   - Use CamelCase for types and traits
   - Use SCREAMING_SNAKE_CASE for constants
   - Prefer explicit over implicit

4. **Documentation**:
   ```rust
   /// Sorts TailwindCSS classes according to official order.
   ///
   /// # Arguments
   ///
   /// * `classes` - Space-separated class string
   ///
   /// # Returns
   ///
   /// Sorted class string with same classes
   ///
   /// # Examples
   ///
   /// ```
   /// let sorted = sort_classes("p-4 mt-2");
   /// assert_eq!(sorted, "mt-2 p-4");
   /// ```
   pub fn sort_classes(classes: &str) -> String {
       // Implementation
   }
   ```

5. **Error Handling**:
   ```rust
   // Use Result for recoverable errors
   fn parse_config(data: &str) -> Result<Config, Error> {
       serde_json::from_str(data)
           .map_err(|e| anyhow::anyhow!("Invalid config: {}", e))
   }
   
   // Use Option for optional values
   fn find_class(text: &str) -> Option<&str> {
       // ...
   }
   ```

### Code Organization

- Keep functions short and focused
- Group related functions together
- Use modules to organize code
- Add comments for complex logic
- Avoid deep nesting

**Example**:
```rust
// Good
fn sort_classes(classes: &str) -> String {
    let parsed = parse_classes(classes);
    let sorted = sort_parsed_classes(parsed);
    join_classes(sorted)
}

// Avoid
fn sort_classes(classes: &str) -> String {
    // 100 lines of mixed logic
}
```

## Review Process

### What to Expect

1. **Initial Review**: Maintainer will review within 1-3 days
2. **Feedback**: You may receive requests for changes
3. **Iteration**: Make requested changes and push updates
4. **Approval**: Once approved, PR will be merged
5. **Release**: Changes included in next release

### Review Criteria

- ✅ Code quality and style
- ✅ Test coverage
- ✅ Documentation completeness
- ✅ Backward compatibility
- ✅ Performance impact
- ✅ Alignment with project goals

### Responding to Feedback

- Be responsive and constructive
- Ask for clarification if needed
- Make requested changes promptly
- Update PR description if scope changes
- Mark conversations as resolved

## Common Contribution Scenarios

### Adding a New TailwindCSS Utility

1. Update `get_category_priority()` in `src/sorter.rs`
2. Add test cases
3. Update documentation

### Improving Performance

1. Profile current performance
2. Implement optimization
3. Benchmark improvement
4. Add performance test
5. Document changes

### Fixing a Bug

1. Create failing test
2. Fix bug
3. Verify test passes
4. Add edge case tests
5. Document fix in PR

## Getting Help

- **Questions**: Open a [GitHub Discussion](https://github.com/friedjoff/dprint-plugin-tailwindcss/discussions)
- **Bugs**: Report via [GitHub Issues](https://github.com/friedjoff/dprint-plugin-tailwindcss/issues)
- **Chat**: Join discussions in existing PRs/Issues

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in relevant documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to dprint-plugin-tailwindcss! 🎉
