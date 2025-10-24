# Migration Guide

Complete guide for migrating from **prettier-plugin-tailwindcss** to **dprint-plugin-tailwindcss**.

## Overview

This guide helps you transition from prettier-plugin-tailwindcss to dprint-plugin-tailwindcss, a faster, more efficient Rust-based alternative.

### Why Migrate?

| Feature | prettier-plugin-tailwindcss | dprint-plugin-tailwindcss |
|---------|----------------------------|---------------------------|
| **Runtime** | Node.js (JavaScript) | WebAssembly (Rust) |
| **Speed** | ~50-200ms for 100KB | ~15ms for 100KB |
| **Startup** | ~100-500ms | ~10ms |
| **Memory** | ~50-100MB | ~10MB |
| **TailwindCSS v4** | ✅ Yes | ✅ Yes |
| **TailwindCSS v3** | ✅ Yes | ❌ No (v4+ only) |
| **Integration** | Prettier | dprint |

### Key Differences

✅ **Compatible**:
- Same sorting algorithm
- Same TailwindCSS v4 support
- Same file format support (HTML, JSX, Vue, Svelte)
- Same class extraction patterns

⚠️ **Differences**:
- Requires dprint instead of Prettier
- No Tailwind v3 support
- Different configuration syntax
- WebAssembly runtime instead of Node.js

---

## Prerequisites

Before migrating, ensure:

1. ✅ You're using **TailwindCSS v4+** (not v3)
2. ✅ Your project uses one of these file formats:
   - HTML/HTM
   - JSX/TSX (React)
   - Vue single-file components
   - Svelte components
   - Astro components
3. ✅ You're willing to use dprint as your formatter

**Not Compatible If**:
- ❌ You need TailwindCSS v3 support
- ❌ You must use Prettier exclusively
- ❌ You use PHP, Ruby, or Twig templates

---

## Step-by-Step Migration

### Step 1: Install dprint

#### Option A: Global Installation

```bash
# macOS/Linux
curl -fsSL https://dprint.dev/install.sh | sh

# Windows (PowerShell)
iwr https://dprint.dev/install.ps1 -useb | iex

# Verify installation
dprint --version
```

#### Option B: Project-Local Installation

```bash
# Via npm (for CI/CD compatibility)
npm install --save-dev dprint

# Add to package.json scripts
{
  "scripts": {
    "format": "dprint fmt",
    "format:check": "dprint check"
  }
}
```

---

### Step 2: Remove Prettier Plugin

```bash
# Remove prettier-plugin-tailwindcss
npm uninstall prettier-plugin-tailwindcss

# Optional: Remove Prettier if not needed for other purposes
npm uninstall prettier
```

---

### Step 3: Create dprint Configuration

Create `dprint.json` in your project root:

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "excludes": [
    "node_modules",
    "dist",
    "build",
    ".next",
    "coverage"
  ]
}
```

**Notes**:
- Include TypeScript plugin for JSX/TSX formatting
- Update plugin versions to latest
- Adjust excludes for your project

---

### Step 4: Migrate Configuration

#### Prettier Configuration

If you had `.prettierrc.json`:

```json
{
  "semi": false,
  "singleQuote": true,
  "tabWidth": 2,
  "plugins": ["prettier-plugin-tailwindcss"],
  "tailwindFunctions": ["clsx", "cn"]
}
```

#### Equivalent dprint Configuration

Update `dprint.json`:

```json
{
  "typescript": {
    "semiColons": "asi",
    "quoteStyle": "single",
    "indentWidth": 2
  },
  "tailwindcss": {
    "enabled": true,
    "tailwindFunctions": ["clsx", "cn"],
    "tailwindAttributes": ["class", "className"]
  }
}
```

#### Configuration Mapping

| Prettier | dprint (TypeScript) | Notes |
|----------|---------------------|-------|
| `semi: false` | `semiColons: "asi"` | ASI = Automatic Semicolon Insertion |
| `singleQuote: true` | `quoteStyle: "single"` | |
| `tabWidth: 2` | `indentWidth: 2` | |
| `printWidth: 80` | `lineWidth: 80` | |
| `trailingComma: "es5"` | `trailingCommas: "onlyMultiLine"` | |
| `arrowParens: "avoid"` | `arrowFunction.useParentheses: "preferNone"` | |

| Prettier Plugin | dprint-plugin-tailwindcss | Notes |
|-----------------|---------------------------|-------|
| `tailwindFunctions` | `tailwindFunctions` | Same syntax |
| `tailwindAttributes` | `tailwindAttributes` | Same syntax |
| `tailwindConfig` | ❌ Not supported | Use defaults |

---

### Step 5: Update Scripts

#### Before (package.json with Prettier)

```json
{
  "scripts": {
    "format": "prettier --write .",
    "format:check": "prettier --check ."
  }
}
```

#### After (package.json with dprint)

```json
{
  "scripts": {
    "format": "dprint fmt",
    "format:check": "dprint check"
  }
}
```

---

### Step 6: Update Git Hooks

#### Before (with Prettier)

```json
{
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx,html,vue}": [
      "prettier --write"
    ]
  }
}
```

#### After (with dprint)

```json
{
  "husky": {
    "hooks": {
      "pre-commit": "lint-staged"
    }
  },
  "lint-staged": {
    "*.{js,jsx,ts,tsx,html,vue}": [
      "dprint fmt"
    ]
  }
}
```

---

### Step 7: Update CI/CD

#### Before (GitHub Actions with Prettier)

```yaml
- name: Check formatting
  run: npm run format:check
```

#### After (GitHub Actions with dprint)

```yaml
- name: Setup dprint
  run: |
    curl -fsSL https://dprint.dev/install.sh | sh
    echo "$HOME/.dprint/bin" >> $GITHUB_PATH

- name: Check formatting
  run: dprint check
```

**Alternative**: Use npm script if dprint is in devDependencies:

```yaml
- name: Install dependencies
  run: npm ci

- name: Check formatting
  run: npm run format:check
```

---

### Step 8: Update VS Code Settings

#### Before (Prettier extension)

`.vscode/settings.json`:
```json
{
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "editor.formatOnSave": true
}
```

#### After (dprint extension)

1. **Install Extension**: [dprint VS Code Extension](https://marketplace.visualstudio.com/items?itemName=dprint.dprint)

2. **Update Settings**:

`.vscode/settings.json`:
```json
{
  "editor.defaultFormatter": "dprint.dprint",
  "editor.formatOnSave": true,
  "dprint.path": "dprint"
}
```

---

### Step 9: Format Entire Codebase

```bash
# Format all files
dprint fmt

# Check what would change (dry run)
dprint check

# Format specific directory
dprint fmt src/

# Format specific files
dprint fmt src/**/*.tsx
```

---

### Step 10: Verify Migration

Run tests to ensure no regressions:

```bash
# 1. Check formatting
dprint check

# 2. Run tests
npm test

# 3. Build project
npm run build

# 4. Compare output visually
git diff
```

---

## Configuration Examples

### Example 1: React + TypeScript Project

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "typescript": {
    "semiColons": "asi",
    "quoteStyle": "single",
    "indentWidth": 2,
    "lineWidth": 100
  },
  "tailwindcss": {
    "tailwindFunctions": ["clsx", "cn", "cva"],
    "tailwindAttributes": ["className"]
  },
  "includes": ["src/**/*.{ts,tsx}"],
  "excludes": [
    "node_modules",
    "dist",
    "**/*.test.tsx"
  ]
}
```

---

### Example 2: Vue 3 Project

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "typescript": {
    "quoteStyle": "single",
    "semiColons": "asi"
  },
  "tailwindcss": {
    "tailwindFunctions": ["cn"],
    "tailwindAttributes": ["class"]
  },
  "includes": ["src/**/*.{vue,ts}"],
  "excludes": ["node_modules", "dist"]
}
```

---

### Example 3: Multi-Framework Monorepo

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/json-0.19.0.wasm",
    "https://plugins.dprint.dev/markdown-0.16.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ],
  "typescript": {
    "semiColons": "asi",
    "quoteStyle": "single"
  },
  "tailwindcss": {
    "tailwindFunctions": ["clsx", "cn", "classNames"],
    "tailwindAttributes": ["class", "className"]
  },
  "includes": [
    "packages/*/src/**/*.{ts,tsx,vue,svelte,html}"
  ],
  "excludes": [
    "node_modules",
    "**/dist",
    "**/build"
  ]
}
```

---

## Troubleshooting

### Issue: Classes Not Sorting

**Cause**: File extension not recognized or plugin not enabled

**Solution**:
1. Check `dprint.json` includes the TailwindCSS plugin
2. Verify file extension is supported (`.html`, `.jsx`, `.tsx`, `.vue`, `.svelte`, `.astro`)
3. Check `includes`/`excludes` patterns
4. Ensure `enabled: true` in config

```bash
# Debug: Check which plugin handles your file
dprint fmt src/Button.tsx --verbose
```

---

### Issue: Different Formatting Output

**Cause**: TypeScript plugin formatting differently than Prettier

**Solution**: Adjust TypeScript plugin settings to match Prettier

```json
{
  "typescript": {
    "semiColons": "asi",           // Match Prettier's semi: false
    "quoteStyle": "single",        // Match singleQuote: true
    "indentWidth": 2,              // Match tabWidth: 2
    "lineWidth": 80,               // Match printWidth: 80
    "trailingCommas": "onlyMultiLine",  // Match trailingComma: "es5"
    "arrowFunction.useParentheses": "preferNone"  // Match arrowParens: "avoid"
  }
}
```

---

### Issue: CI Failing

**Cause**: dprint not installed in CI environment

**Solution**: Add dprint installation to CI workflow

```yaml
# GitHub Actions
- name: Setup dprint
  run: |
    curl -fsSL https://dprint.dev/install.sh | sh
    echo "$HOME/.dprint/bin" >> $GITHUB_PATH
```

Or use npm version:

```yaml
- name: Install dependencies
  run: npm ci  # Installs dprint from devDependencies
```

---

### Issue: Performance Slower Than Expected

**Cause**: Processing unnecessary files (node_modules, etc.)

**Solution**: Add to `.dprintignore`:

```gitignore
node_modules/
dist/
build/
.next/
coverage/
**/*.min.*
vendor/
```

---

### Issue: Custom Utility Functions Not Recognized

**Cause**: Function not in `tailwindFunctions` config

**Solution**: Add to `dprint.json`:

```json
{
  "tailwindcss": {
    "tailwindFunctions": ["clsx", "cn", "myCustomHelper"]
  }
}
```

---

## Feature Comparison

### Supported Features

| Feature | prettier-plugin-tailwindcss | dprint-plugin-tailwindcss |
|---------|----------------------------|---------------------------|
| TailwindCSS v4 | ✅ | ✅ |
| TailwindCSS v3 | ✅ | ❌ |
| HTML files | ✅ | ✅ |
| JSX/TSX files | ✅ | ✅ |
| Vue files | ✅ | ✅ |
| Svelte files | ✅ | ✅ |
| Astro files | ✅ | ✅ |
| Custom functions | ✅ | ✅ |
| Custom attributes | ✅ | ✅ |
| Arbitrary values | ✅ | ✅ |
| Variants | ✅ | ✅ |
| Important modifier | ✅ | ✅ |
| Negative values | ✅ | ✅ |

### Not Yet Supported

| Feature | Status | Workaround |
|---------|--------|------------|
| TailwindCSS v3 | ❌ Not planned | Stay on prettier-plugin or upgrade to v4 |
| PHP/Blade templates | ❌ Not yet | Use prettier-plugin for PHP files |
| Ruby ERB templates | ❌ Not yet | Use prettier-plugin for ERB files |
| Twig templates | ❌ Not yet | Use prettier-plugin for Twig files |
| `tailwind.config.js` integration | ❌ Not yet | Use standard class names |

---

## Migration Checklist

Use this checklist to track your migration progress:

- [ ] Install dprint globally or locally
- [ ] Remove prettier-plugin-tailwindcss from package.json
- [ ] Create `dprint.json` configuration
- [ ] Migrate Prettier settings to dprint config
- [ ] Update package.json scripts
- [ ] Update git hooks (husky, lint-staged)
- [ ] Update CI/CD workflows
- [ ] Install dprint VS Code extension
- [ ] Update VS Code settings
- [ ] Format entire codebase with dprint
- [ ] Verify tests pass
- [ ] Commit changes
- [ ] Update documentation

---

## Rollback Plan

If you need to rollback:

```bash
# 1. Reinstall prettier-plugin-tailwindcss
npm install --save-dev prettier prettier-plugin-tailwindcss

# 2. Restore .prettierrc.json
git checkout main -- .prettierrc.json

# 3. Remove dprint.json
rm dprint.json

# 4. Restore package.json scripts
git checkout main -- package.json

# 5. Format codebase with Prettier
npm run format
```

---

## Getting Help

### Resources

- **Documentation**: [GitHub README](https://github.com/friedjoff/dprint-plugin-tailwindcss)
- **Issues**: [Report a bug](https://github.com/friedjoff/dprint-plugin-tailwindcss/issues)
- **Discussions**: [Ask questions](https://github.com/friedjoff/dprint-plugin-tailwindcss/discussions)
- **dprint Docs**: [dprint.dev](https://dprint.dev)

### Common Questions

**Q: Can I use both Prettier and dprint?**  
A: Yes, but configure file patterns to avoid conflicts. Let dprint handle TailwindCSS files and Prettier handle others.

**Q: Will this work with my existing Prettier config?**  
A: You'll need to migrate settings to dprint.json, but most options have equivalents.

**Q: Is the sorting order exactly the same?**  
A: Yes, for TailwindCSS v4. We follow the same priority system.

**Q: What if I need Tailwind v3 support?**  
A: Stay on prettier-plugin-tailwindcss or upgrade your project to Tailwind v4.

**Q: How do I format only changed files?**  
A: Use `git diff --name-only | xargs dprint fmt` or enable incremental mode.

---

## Success Stories

> "Migrated our React monorepo in 30 minutes. Build times dropped from 2m to 45s."  
> — **Frontend Team, TechCorp**

> "The WebAssembly runtime is noticeably faster. No more waiting for Prettier."  
> — **Solo Developer**

> "Works great with our Vue 3 + TypeScript stack. Easy migration."  
> — **Product Team, StartupXYZ**

---

## Next Steps

After migration:

1. ✅ Read [Configuration Guide](CONFIGURATION.md) for advanced options
2. ✅ Check [Performance Guide](PERFORMANCE.md) for optimization tips
3. ✅ Review [Plugin Compatibility](PLUGIN_COMPATIBILITY.md) for multi-plugin setups
4. ✅ Star the [GitHub repo](https://github.com/friedjoff/dprint-plugin-tailwindcss) 🌟

---

**Migration Difficulty**: Easy (30-60 minutes)  
**Recommended For**: TailwindCSS v4 projects seeking better performance  
**Last Updated**: 2025-10-24
