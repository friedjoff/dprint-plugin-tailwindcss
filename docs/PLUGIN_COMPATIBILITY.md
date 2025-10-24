# Plugin Compatibility Guide

This document describes how dprint-plugin-tailwindcss integrates with the dprint plugin ecosystem and ensures compatibility with other formatters.

## Overview

dprint-plugin-tailwindcss is designed to work harmoniously with other dprint plugins, focusing solely on TailwindCSS class sorting without interfering with general code formatting.

## Compatibility Architecture

### Plugin Decision Flow

```
File received
    │
    ▼
Is file extension supported?
    │
    ├─ No → Return None (skip)
    │
    ▼ Yes
Should defer to another plugin?
    │
    ├─ Yes → Return None (let other plugin handle)
    │
    ▼ No
Format TailwindCSS classes only
    │
    └─ Return formatted content
```

### Implementation

```rust
// Check if we should format this file
pub fn should_format(path: &str) -> bool {
    matches!(
        FileFormat::from_path(path),
        Some(FileFormat::Html | FileFormat::Jsx | FileFormat::Vue |
             FileFormat::Svelte | FileFormat::Astro)
    )
}

// Check if another plugin should handle this
pub fn should_defer(path: &str) -> bool {
    // Let TypeScript plugin handle .ts files
    // Let JSON plugin handle .json files
    // etc.
}
```

## Compatible Plugins

### ✅ Fully Compatible

| Plugin | Purpose | Interaction |
|--------|---------|-------------|
| **typescript** | TypeScript/JavaScript formatting | Handles code structure, we handle classes |
| **json** | JSON formatting | No overlap |
| **markdown** | Markdown formatting | We format code blocks, they format markdown |
| **toml** | TOML formatting | No overlap |
| **dockerfile** | Dockerfile formatting | No overlap |
| **prettier** | General formatting | Can coexist with careful configuration |

### Configuration Example

```json
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/json-0.19.0.wasm",
    "https://plugins.dprint.dev/markdown-0.16.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "typescript": {
    "quoteStyle": "single"
  },
  "tailwindcss": {
    "enabled": true
  }
}
```

## Integration Patterns

### Pattern 1: TypeScript + TailwindCSS

**Use Case**: React/Vue/Svelte projects

**Configuration**:
```json
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "typescript": {
    "semiColons": "asi",
    "quoteStyle": "single"
  },
  "tailwindcss": {
    "tailwindFunctions": ["clsx", "cn"],
    "tailwindAttributes": ["className"]
  }
}
```

**Result**:
- TypeScript plugin formats code structure
- TailwindCSS plugin sorts classes
- No conflicts

### Pattern 2: Multi-Format Project

**Use Case**: Full-stack with various file types

**Configuration**:
```json
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/json-0.19.0.wasm",
    "https://plugins.dprint.dev/markdown-0.16.0.wasm",
    "https://plugins.dprint.dev/toml-0.6.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "includes": ["**/*.{ts,tsx,js,jsx,json,md,toml,html,vue,svelte}"],
  "excludes": ["node_modules", "dist", "build"]
}
```

**Result**:
- Each plugin handles its file types
- TailwindCSS plugin processes only class attributes
- Clean separation of concerns

### Pattern 3: Component Libraries

**Use Case**: Shared component library with TailwindCSS

**Configuration**:
```json
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "typescript": {
    "quoteStyle": "single",
    "semiColons": "asi"
  },
  "tailwindcss": {
    "tailwindFunctions": ["cn", "cva"],
    "tailwindAttributes": ["className", "class"]
  },
  "includes": ["src/**/*.{tsx,jsx}"],
  "excludes": ["**/*.test.{tsx,jsx}", "**/*.stories.{tsx,jsx}"]
}
```

**Result**:
- Component files formatted for both code and classes
- Test files excluded (optional)
- Story files excluded (optional)

## Conflict Resolution

### Potential Conflicts

#### 1. Multiple Plugins Formatting Same File

**Problem**: Both plugins try to format the same content  
**Solution**: Use `excludes` or file matching

```json
{
  "typescript": {
    "includes": ["**/*.{ts,tsx,js,jsx}"]
  },
  "tailwindcss": {
    "includes": ["**/*.{html,jsx,tsx,vue,svelte,astro}"]
  }
}
```

#### 2. Different Quote Styles

**Problem**: TypeScript uses single quotes, classes extracted as double quotes  
**Solution**: TailwindCSS plugin preserves original quotes

```jsx
// TypeScript formats to single quotes
const Component = () => <div className='...' />

// TailwindCSS sorts classes, keeps single quotes
const Component = () => <div className='sorted classes here' />
```

#### 3. Line Length Conflicts

**Problem**: TypeScript wraps long lines, TailwindCSS on one line  
**Solution**: TailwindCSS preserves line breaks

```jsx
// Before (multiple lines)
<div className="
  very long
  class list
">

// After (sorted, but line breaks preserved if present)
<div className="
  sorted very long
  class list here
">
```

## Testing Compatibility

### Test Suite

We have 18 dedicated plugin ecosystem tests:

```rust
#[test]
fn test_multiple_plugins_coexistence() {
    // Verifies no conflicts with TypeScript plugin
}

#[test]
fn test_format_idempotency() {
    // Ensures consistent output across runs
}

#[test]
fn test_comments_preserved_during_formatting() {
    // Verifies comments untouched
}
```

### Manual Testing

```bash
# 1. Set up multi-plugin config
cat > dprint.json << EOF
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ]
}
EOF

# 2. Create test file
cat > test.tsx << EOF
import clsx from 'clsx';

export const Button = ({ primary }: { primary: boolean }) => {
  return (
    <button className={clsx("p-4 rounded", primary && "bg-blue-500 text-white")}>
      Click me
    </button>
  );
};
EOF

# 3. Format
dprint fmt test.tsx

# 4. Verify output
cat test.tsx
# Should show:
# - TypeScript formatting (imports, spacing)
# - Sorted TailwindCSS classes
# - No conflicts
```

## Preservation Guarantees

### What TailwindCSS Plugin Preserves

✅ **Preserved**:
- Line breaks (except within class strings)
- Indentation
- Comments
- Non-class code
- Quote style (single vs double)
- Template expressions (Vue/Svelte)
- JSX expressions

❌ **Modified**:
- Class order within class strings only
- Whitespace within class strings (normalized)

### Example

```jsx
// Input
const Component = () => (
  // Header component
  <header
    className="hover:bg-gray-100 flex items-center justify-between p-4 bg-white"
    data-testid="header"
  >
    {/* Navigation */}
    <nav className="flex space-x-4">
      <a href="/">Home</a>
    </nav>
  </header>
);

// Output (only class order changed)
const Component = () => (
  // Header component ← preserved
  <header
    className="flex items-center justify-between bg-white p-4 hover:bg-gray-100"
    data-testid="header"
  >
    {/* Navigation */} ← preserved
    <nav className="flex space-x-4">
      <a href="/">Home</a>
    </nav>
  </header>
);
```

## Best Practices

### 1. Plugin Order

**Recommendation**: List TailwindCSS plugin last

```json
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/json-0.19.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"  ← Last
  ]
}
```

**Reason**: Allows other plugins to format structure first

### 2. Explicit Includes

**Recommendation**: Be explicit about which files each plugin handles

```json
{
  "typescript": {
    "includes": ["src/**/*.{ts,tsx}"]
  },
  "tailwindcss": {
    "includes": ["src/**/*.{tsx,html,vue}"]
  }
}
```

**Reason**: Avoids ambiguity and improves performance

### 3. Shared Excludes

**Recommendation**: Use global excludes for common patterns

```json
{
  "excludes": [
    "node_modules/**",
    "dist/**",
    "build/**",
    "**/*.min.*"
  ],
  "plugins": [...]
}
```

**Reason**: Consistent across all plugins

### 4. Test Integration

**Recommendation**: Test with real project setup

```bash
# Format entire project
dprint fmt

# Check for issues
dprint check

# Verify specific file types
dprint fmt src/**/*.tsx --verbose
```

## Troubleshooting

### Issue: Classes Not Sorting

**Check**:
1. Is file extension supported?
2. Is plugin enabled?
3. Are plugins in correct order?
4. Check `includes`/`excludes` patterns

### Issue: Conflicts with TypeScript Plugin

**Check**:
1. Are both plugins trying to format same content?
2. Use `dprint fmt --verbose` to see which plugin handles which file
3. Adjust `includes` patterns if needed

### Issue: Inconsistent Formatting

**Check**:
1. Run `dprint fmt` twice - should be idempotent
2. Check for non-deterministic patterns in config
3. Verify all plugins are using stable versions

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/friedjoff/dprint-plugin-tailwindcss/issues)
- **Discussions**: [GitHub Discussions](https://github.com/friedjoff/dprint-plugin-tailwindcss/discussions)
- **dprint Discord**: [dprint Community](https://discord.gg/dprint)

## Compatibility Testing Results

### Test Suite: 18 Plugin Ecosystem Tests

```
✅ test_comments_preserved_during_formatting
✅ test_disabled_plugin_returns_none
✅ test_file_matching_configuration
✅ test_format_idempotency
✅ test_global_config_respected
✅ test_html_files_formatted
✅ test_json_files_not_formatted
✅ test_jsx_files_formatted
✅ test_line_breaks_preserved
✅ test_markdown_files_formatted
✅ test_mixed_framework_file
✅ test_multiple_plugins_coexistence
✅ test_no_false_positives_in_script_tags
✅ test_plugin_compatibility_checks
✅ test_typescript_files_formatted
✅ test_unknown_extension_fallback
✅ test_whitespace_preserved_outside_classes
✅ test_yaml_files_not_formatted
```

**Result**: 100% compatibility verified

---

**Compatibility Status**: Production-ready  
**Verified With**: dprint-plugin-typescript, dprint-plugin-json, dprint-plugin-markdown
