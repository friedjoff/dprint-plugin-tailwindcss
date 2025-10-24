# dprint-plugin-tailwindcss

dprint Wasm plugin for TailwindCSS

## Overview

This plugin sorts and formats TailwindCSS class names in your code, similar to [prettier-plugin-tailwindcss](https://github.com/tailwindlabs/prettier-plugin-tailwindcss) but for the [dprint](https://dprint.dev/) formatter.

## Implementation Plan

### 1. Project Setup

- [x] Initialize Rust project with Wasm target support
  - Create `Cargo.toml` with dependencies: `dprint-core`, `wasm-bindgen`, `serde`, `serde_json`
  - Set crate type to `["cdylib"]` for Wasm compilation
- [x] Set up build configuration for Wasm32 target
  - Configure `wasm-pack` or manual `wasm32-unknown-unknown` compilation
- [x] Create plugin schema configuration structure
- [x] **Verify:** Build project successfully with `cargo build --target wasm32-unknown-unknown` and confirm `.wasm` file is generated

### 2. Core Plugin Structure

Following [dprint Wasm plugin development docs](https://github.com/dprint/dprint/blob/main/docs/wasm-plugin-development.md):

- [x] Implement required exports:
  - `get_plugin_info()` - Returns plugin metadata (name, version, config schema)
  - `get_license_text()` - Returns MIT license text
  - `get_resolved_config(config: &str)` - Parses and validates configuration
  - `set_global_config(global_config: &str)` - Sets global dprint config
  - `set_file_path(file_path: &str)` - Sets current file being formatted
  - `format(file_text: &str, range: &FormatRange, override_config: &str)` - Main formatting function
- [x] **Verify:** Load plugin in dprint with `dprint config add <plugin-url>` and run `dprint --plugins` to confirm plugin is recognized

### 3. Configuration Options

- [x] Define configuration schema:
  - `tailwindConfig` - Path to tailwind.config.js (optional)
  - `tailwindFunctions` - Custom function names containing class lists (default: `["classnames", "clsx", "ctl", "cva", "tw"]`)
  - `tailwindAttributes` - HTML attributes to format (default: `["class", "className"]`)
  - `enabled` - Enable/disable the plugin (default: `true`)
- [x] **Verify:** Test config parsing with valid and invalid configurations, confirm appropriate error messages are returned for invalid configs

### 4. TailwindCSS Class Sorting Logic

- [x] Parse TailwindCSS configuration (if provided)
  - Load and parse `tailwind.config.js` to understand custom utilities
  - Extract custom class prefixes and modifiers
- [x] Implement class name detection:
  - Find class attributes in HTML/JSX/Vue/Svelte templates
  - Find utility function calls (e.g., `clsx()`, `classnames()`)
  - Support template literals and string concatenation
- [x] Implement sorting algorithm:
  - Base utilities (layout, spacing, etc.) first
  - Modifiers (hover, focus, responsive breakpoints) in consistent order
  - Custom classes last
  - Preserve variant order (e.g., `dark:hover:` before class name)
- [x] Handle special cases:
  - Arbitrary values (e.g., `w-[100px]`)
  - Important modifier (`!`)
  - Negative values (e.g., `-mt-4`)
- [x] **Verify:** Create test cases with unsorted classes and confirm output matches expected order: `"z-10 p-4 mt-2"` becomes `"mt-2 p-4 z-10"` (following TailwindCSS recommended order)

### 5. File Format Support

- [ ] HTML/HTM files
- [ ] JSX/TSX (React) files
- [ ] Vue single-file components
- [ ] Svelte components
- [ ] Astro components
- [ ] Use existing dprint language plugins as parsers where possible
- [ ] **Verify:** Format sample files of each type and confirm classes are sorted without breaking syntax or losing content

### 6. Parsing Strategy

- [ ] Leverage dprint's plugin ecosystem:
  - Use `dprint-plugin-typescript` for JSX/TSX parsing
  - Use `dprint-plugin-markup` for HTML parsing
  - Implement custom parsers for Vue/Svelte if needed
- [ ] Extract class attributes and utility functions from AST
- [ ] Preserve original formatting except for class order
- [ ] **Verify:** Format files with complex nesting, comments, and mixed content; confirm non-class code remains unchanged

### 7. Integration Points

- [ ] Hook into dprint's formatting pipeline
- [ ] Ensure compatibility with other dprint plugins
- [ ] Handle incremental formatting (range formatting)
- [ ] Preserve comments and whitespace outside class strings
- [ ] **Verify:** Use plugin alongside `dprint-plugin-typescript` and `dprint-plugin-json` in a project, confirm no conflicts occur

### 8. Testing

- [ ] Unit tests for class sorting algorithm
- [ ] Integration tests with various file formats
- [ ] Test with real-world TailwindCSS projects
- [ ] Test custom configurations
- [ ] Test edge cases (malformed classes, mixed content)
- [ ] **Verify:** Run full test suite with `cargo test` and achieve >90% code coverage; all tests pass

### 9. Build and Distribution

- [ ] Compile to Wasm module
- [ ] Publish to dprint plugin registry
- [ ] Create GitHub releases with compiled `.wasm` file
- [ ] Document installation and usage
- [ ] **Verify:** Install plugin from published URL using `dprint config add`, confirm it downloads and works in a fresh project

### 10. Documentation

- [ ] Installation instructions
- [ ] Configuration options
- [ ] Examples for different frameworks
- [ ] Troubleshooting guide
- [ ] Contribution guidelines
- [ ] **Verify:** Follow documentation from scratch in a new environment, confirm all instructions work without prior knowledge of the project

## Technical Architecture

```
┌─────────────────────────────────────┐
│   dprint CLI / Editor Integration   │
└──────────────┬──────────────────────┘
               │
               ▼
│  └───────────────────────────────┘ │
│                                     │
│  ┌───────────────────────────────┐ │
│  │  Class Attribute Extractor    │ │
│  │  - HTML attributes            │ │
│  │  - Utility functions          │ │
│  └───────────────────────────────┘ │
│                                     │
│  ┌───────────────────────────────┐ │
│  │  TailwindCSS Sorter           │ │
│  │  - Parse class names          │ │
│  │  - Sort by category           │ │
│  │  - Preserve variants          │ │
│  └───────────────────────────────┘ │
│                                     │
│  ┌───────────────────────────────┐ │
│  │  Config Loader (optional)     │ │
│  │  - tailwind.config.js         │ │
│  └───────────────────────────────┘ │
└─────────────────────────────────────┘
```

## Development Workflow

1. Set up Rust development environment
2. Implement basic plugin interface
3. Add class detection for HTML
4. Implement TailwindCSS sorting logic
5. Extend to JSX/TSX support
6. Add configuration options
7. Test with real projects
8. Optimize performance
9. Publish to registry

## References

- [dprint Wasm Plugin Development Guide](https://github.com/dprint/dprint/blob/main/docs/wasm-plugin-development.md)
- [prettier-plugin-tailwindcss](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)
- [TailwindCSS Class Sorting](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier)
- [dprint Plugin Registry](https://dprint.dev/plugins/)

## License

MIT
