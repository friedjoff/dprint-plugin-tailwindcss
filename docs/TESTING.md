# Quick Reference: Testing

## Run All Tests

```bash
# Unit tests
cargo test --lib --target x86_64-unknown-linux-gnu

# Configuration validation
./test-config.sh

# Sorting logic validation
./test-sorting.sh
```

## Test Results Summary

✅ **50/50 tests passing**

### Sorter Tests (23 tests)
- ✅ Class parsing (simple, important, negative, arbitrary, variants)
- ✅ Sorting algorithm (simple, complex, mixed)
- ✅ Category priority (layout, spacing, typography)
- ✅ Variant priority (responsive, state)
- ✅ Real-world examples

### Extractor Tests (14 tests)
- ✅ HTML attribute extraction (class, className)
- ✅ Function call extraction (clsx, classnames, cn)
- ✅ JSX expression extraction
- ✅ Position tracking
- ✅ Real-world examples (React, Vue)

### Configuration Tests (8 tests)
- ✅ Default configuration
- ✅ Custom values
- ✅ Invalid types handling
- ✅ Unknown properties detection
- ✅ File matching extensions

### Plugin Tests (5 tests)
- ✅ Plugin info
- ✅ License text
- ✅ Config resolution
- ✅ Config updates

## Configuration Schema

```typescript
interface TailwindCSSConfig {
  enabled?: boolean;              // Default: true
  tailwindConfig?: string;        // Default: null
  tailwindFunctions?: string[];   // Default: ["classnames", "clsx", "ctl", "cva", "tw"]
  tailwindAttributes?: string[];  // Default: ["class", "className"]
}
```

## Supported File Extensions

`.html`, `.htm`, `.jsx`, `.tsx`, `.vue`, `.svelte`, `.astro`

## Sorting Examples

```
"z-10 p-4 mt-2" → "mt-2 p-4 z-10"
"hover:bg-blue-500 bg-red-500" → "bg-red-500 hover:bg-blue-500"
"!text-red-500 text-blue-500" → "text-blue-500 !text-red-500"
"-mt-4 mt-4 pt-4" → "mt-4 -mt-4 pt-4"
"w-[100px] w-full" → "w-full w-[100px]"
```

## Example Configurations

See `test-configs/` directory for examples:
- `valid-default.json` - Uses all defaults
- `valid-custom.json` - Full customization
- `valid-disabled.json` - Plugin disabled
- `invalid-wrong-type.json` - Type errors (handled gracefully)
- `invalid-unknown-property.json` - Unknown properties (reported in diagnostics)

## Documentation

- Configuration: `docs/CONFIGURATION.md`
- Step 3 Summary: `docs/STEP_3_SUMMARY.md`
- Step 4 Summary: `docs/STEP_4_SUMMARY.md`
- Core plugin structure: `docs/CORE_PLUGIN_STRUCTURE.md`

## Status

✅ **Step 3: Configuration Options - COMPLETED**
✅ **Step 4: TailwindCSS Class Sorting Logic - COMPLETED**
