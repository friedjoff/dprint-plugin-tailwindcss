# Step 6: Parsing Strategy - Implementation Summary

## Status: ✅ Complete

All 89 tests passing (10 new format-aware tests added), bringing total test count to **89/89 tests**.

## What Was Implemented

### 1. Format-Aware Parser Module (`src/parser.rs`)

Created a sophisticated parsing strategy that handles different file formats intelligently:

#### FileFormat Enum
Detects file type from extension:
- `.html`, `.htm` → `FileFormat::Html`
- `.jsx` → `FileFormat::Jsx`
- `.tsx` → `FileFormat::Tsx`
- `.vue` → `FileFormat::Vue`
- `.svelte` → `FileFormat::Svelte`
- `.astro` → `FileFormat::Astro`

#### FormatParser Struct
Provides format-specific parsing with structure preservation:

```rust
pub struct FormatParser {
    extractor: ClassExtractor,
}

impl FormatParser {
    pub fn parse(&self, content: &str, format: FileFormat) -> Vec<ClassMatch>
}
```

### 2. Format-Specific Parsing Logic

#### HTML/HTM Parser
**Strategy**: Direct parsing of standard HTML class attributes
- Extracts from `class="..."` attributes
- Checks for inline script with utility functions
- Preserves all HTML structure, comments, whitespace

**Key Features**:
- Simple and straightforward
- Minimal processing needed
- Full comment preservation

#### JSX/TSX Parser
**Strategy**: Parse both className attributes and utility functions
- Extracts from `className="..."` and `class="..."`
- Parses template literals: `className={\`...\`}`
- Parses utility functions: `className={clsx(...)}`

**Key Features**:
- React/Preact compatible
- Supports dynamic class expressions
- Handles string concatenation

#### Vue Parser
**Strategy**: Template-section-only parsing with fallback
- Locates `<template>` section
- Extracts classes only from template markup
- Ignores `<script>` and `<style>` sections
- Falls back to full-file parsing if no template found

**Implementation Details**:
```rust
fn extract_vue_template(content: &str) -> Option<ContentSection> {
    // Find <template> opening tag
    let template_start_tag = content.find("<template")?;
    let template_content_start = content[template_start_tag..].find('>')? + template_start_tag + 1;
    
    // Find </template> closing tag
    let template_end = content.find("</template>")?;
    
    Some(ContentSection {
        start: template_content_start,
        content: content[template_content_start..template_end].to_string(),
    })
}
```

**Key Features**:
- **Position tracking**: Adjusts match positions to account for template offset
- **Script/style exclusion**: JavaScript and CSS remain untouched
- **Fallback support**: Works even with non-standard Vue files

#### Svelte Parser
**Strategy**: Extract markup sections, excluding `<script>` and `<style>`
- Finds all `<script>` and `<style>` tag ranges
- Extracts markup sections between excluded ranges
- Parses classes in markup sections only

**Implementation Details**:
```rust
fn extract_svelte_markup_sections(content: &str) -> Vec<ContentSection> {
    // Find all <script> and <style> tags
    let mut excluded_ranges = Vec::new();
    
    // Find <script> tags
    while let Some(script_start) = content[search_pos..].find("<script") {
        if let Some(script_end) = content[abs_start..].find("</script>") {
            excluded_ranges.push((abs_start, abs_end));
        }
    }
    
    // Find <style> tags (same process)
    // Extract sections between excluded ranges
}
```

**Key Features**:
- **Multiple section support**: Handles multiple script/style blocks
- **Reactive syntax**: Preserves Svelte's `{#if}`, `{#each}`, etc.
- **Position adjustment**: Each section's positions properly tracked

#### Astro Parser
**Strategy**: Skip frontmatter, parse markup
- Detects frontmatter section (`---...---`)
- Calculates markup start position after frontmatter
- Parses classes in markup section only

**Implementation Details**:
```rust
fn find_astro_frontmatter_end(content: &str) -> Option<usize> {
    // Check if file starts with ---
    if !content.trim_start().starts_with("---") {
        return None;
    }
    
    // Find the closing ---
    let start = content.find("---")? + 3;
    let remaining = &content[start..];
    let end = remaining.find("---")? + start + 3;
    
    // Return position after closing ---
    Some(end + newline + 1)
}
```

**Key Features**:
- **TypeScript frontmatter**: Ignores TypeScript/JavaScript logic
- **JSX-like markup**: Handles Astro's HTML-like syntax
- **No frontmatter fallback**: Works with Astro files without frontmatter

### 3. Integration with Main Plugin

Updated `format()` in `src/lib.rs`:

```rust
// Determine file format from path
let file_path = request.file_path.to_string_lossy();
let format = FileFormat::from_path(&file_path);

// Create extractor with configured function and attribute names
let extractor = ClassExtractor::new(
    request.config.tailwind_functions.clone(),
    request.config.tailwind_attributes.clone(),
);

// Extract all class strings using format-aware parsing
let matches = if let Some(format) = format {
    let parser = FormatParser::new(extractor);
    parser.parse(&file_text, format)
} else {
    // Fallback to basic extraction if format is unknown
    let mut matches = extractor.extract_from_attributes(&file_text);
    let function_matches = extractor.extract_from_functions(&file_text);
    matches.extend(function_matches);
    matches
};
```

**Benefits**:
- Automatic format detection from file path
- Format-specific parsing for better accuracy
- Fallback for unknown file types
- Clean separation of concerns

### 4. Comprehensive Test Suite

Added 10 new format-aware integration tests in `src/format_aware_tests.rs`:

#### Section Isolation Tests:
1. **`test_vue_only_parses_template_section`**
   - Verifies classes in `<script>` and `<style>` are ignored
   - Confirms only template section is parsed
   - Validates position tracking

2. **`test_svelte_excludes_script_and_style`**
   - Tests multiple markup sections
   - Ensures script/style sections are skipped
   - Verifies position tracking across sections

3. **`test_astro_excludes_frontmatter`**
   - Confirms TypeScript frontmatter is ignored
   - Validates markup parsing
   - Checks position adjustment

#### Content Preservation Tests:
4. **`test_html_preserves_comments_and_structure`**
   - Ensures HTML comments are not parsed
   - Verifies DOCTYPE and structure remain intact
   - Validates position tracking

5. **`test_jsx_parses_utility_functions`**
   - Tests clsx/cn function parsing
   - Validates className attribute extraction
   - Ensures nested expressions work

#### Multi-line and Complex Tests:
6. **`test_vue_with_multiline_template`**
   - Tests complex nested Vue templates
   - Validates multiple class attributes
   - Ensures all are within template section

7. **`test_svelte_with_reactive_statements`**
   - Tests Svelte reactive syntax (`$:`)
   - Validates control flow (`{#if}`, `{/if}`)
   - Ensures event handlers preserved

#### Edge Case Tests:
8. **`test_format_detection_from_file_paths`**
   - Tests format detection for all file types
   - Validates unknown formats return None
   - Ensures case-insensitive extension matching

9. **`test_position_tracking_across_formats`**
   - Validates position accuracy for all formats
   - Ensures extracted content matches positions
   - Tests format-specific quirks

10. **`test_unknown_format_fallback`**
    - Tests fallback extraction for unknown formats
    - Ensures plugin still works without format detection
    - Validates basic extraction functionality

## Test Results

### Full Test Suite: 89/89 Passing ✅

```
test result: ok. 89 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### Breakdown by Module:
- **Config Tests**: 8 tests
- **Sorter Tests**: 23 tests
- **Extractor Tests**: 14 tests
- **Plugin Tests**: 5 tests
- **Integration Tests**: 20 tests
- **Parser Tests**: 9 tests
- **Format-Aware Tests**: 10 tests (NEW)

### Test Coverage Summary:

| Category | Tests | Coverage |
|----------|-------|----------|
| Configuration | 8 | Schema, validation, defaults |
| Class Sorting | 23 | Parsing, priority, variants |
| Class Extraction | 14 | HTML, JSX, functions, positions |
| Plugin Interface | 5 | Info, license, config resolution |
| File Formats | 20 | HTML, JSX, TSX, Vue, Svelte, Astro |
| Parser Logic | 9 | Format detection, section extraction |
| Format-Aware | 10 | Section isolation, preservation, edge cases |

## Technical Achievements

### 1. Intelligent Section Parsing

**Vue Single-File Components**:
- ✅ Parses only `<template>` section
- ✅ Ignores `<script>` (JavaScript/TypeScript)
- ✅ Ignores `<style>` (CSS)
- ✅ Position tracking with section offset

**Svelte Components**:
- ✅ Extracts multiple markup sections
- ✅ Excludes all `<script>` and `<style>` blocks
- ✅ Preserves reactive syntax (`$:`, `{#if}`, etc.)
- ✅ Handles multiple script/style blocks

**Astro Components**:
- ✅ Detects and skips frontmatter (`---...---`)
- ✅ Parses JSX-like markup
- ✅ Works with and without frontmatter
- ✅ TypeScript/JavaScript in frontmatter ignored

### 2. Structure Preservation

**Comments**:
```html
<!-- This comment is preserved -->
<div class="z-10 p-4 mt-2">Content</div>
<!-- Another comment -->
```
Comments remain in their original positions and are never parsed as class content.

**Whitespace**:
```vue
<template>
  <div class="flex p-4">
    <span class="text-lg">Nested</span>
  </div>
</template>
```
Indentation, newlines, and spacing are completely preserved.

**Non-Class Code**:
```svelte
<script>
  let className = "this-is-javascript"; // Not parsed
</script>

<div class="this-is-html">Parsed!</div>
```
JavaScript, TypeScript, and CSS are never modified.

### 3. Robust Position Tracking

Each format parser maintains accurate position information:
- Vue: Adjusts positions for template section offset
- Svelte: Tracks positions across multiple sections
- Astro: Adjusts for frontmatter length
- All formats: Validates positions match extracted content

### 4. Fallback Strategy

For unknown file formats (`.php`, `.erb`, `.blade.php`, etc.):
- Falls back to basic extraction
- Still attempts to find class attributes
- Works with HTML-like syntax
- Provides degraded but functional behavior

## Architecture Improvements

### Before (Step 5):
```
format() → ClassExtractor → extract_all() → Sort and replace
```

### After (Step 6):
```
format() → FileFormat detection → FormatParser
           ↓                       ↓
       Unknown format?         parse()
           ↓                       ↓
    Fallback extraction     Format-specific parsing
           ↓                       ↓
                Sort and replace
```

**Benefits**:
- **Accuracy**: Format-specific parsing is more precise
- **Safety**: Section-aware parsing prevents false matches
- **Extensibility**: Easy to add new formats
- **Compatibility**: Fallback ensures broad support

## Example Transformations

### Vue Component (Template-Only):
```vue
<!-- BEFORE -->
<template>
  <div class="z-10 p-4 mt-2">Content</div>
</template>

<script>
export default {
  data() {
    return {
      className: "z-10 p-4 mt-2" // NOT touched
    }
  }
}
</script>

<!-- AFTER -->
<template>
  <div class="mt-2 p-4 z-10">Content</div>
</template>

<script>
export default {
  data() {
    return {
      className: "z-10 p-4 mt-2" // Still NOT touched
    }
  }
}
</script>
```

### Svelte Component (Markup-Only):
```svelte
<!-- BEFORE -->
<script>
  let count = 0;
  let className = "z-10 p-4"; // NOT touched
</script>

<div class="z-10 p-4 mt-2">
  <button class="text-white bg-blue-500">Click</button>
</div>

<style>
  div { /* NOT touched */ }
</style>

<!-- AFTER -->
<script>
  let count = 0;
  let className = "z-10 p-4"; // Still NOT touched
</script>

<div class="mt-2 p-4 z-10">
  <button class="bg-blue-500 text-white">Click</button>
</div>

<style>
  div { /* Still NOT touched */ }
</style>
```

### Astro Component (Markup-Only):
```astro
---
// BEFORE (frontmatter NOT touched)
const title = "Hello";
const className = "z-10 p-4 mt-2";
---

<div class="z-10 p-4 mt-2">{title}</div>

<!-- AFTER -->
---
// Frontmatter still NOT touched
const title = "Hello";
const className = "z-10 p-4 mt-2";
---

<div class="mt-2 p-4 z-10">{title}</div>
```

## Performance Considerations

### Section Extraction Optimization:
- **Vue**: Single pass to find template boundaries
- **Svelte**: Linear scan for script/style tags (O(n))
- **Astro**: Early exit if no frontmatter detected

### Position Tracking:
- **No re-parsing**: Positions calculated during extraction
- **Offset adjustment**: Simple arithmetic, no string operations
- **Single allocation**: ContentSection structs reused

### Regex Caching:
- Extractor regexes compiled once (via `once_cell`)
- No regex compilation during parsing
- Efficient pattern matching

## Next Steps

Step 6 is now complete. Remaining steps from the implementation plan:

- **Step 7**: Integration Points (partially done - format-aware integration complete)
- **Step 8**: Testing - Test with real-world TailwindCSS projects
- **Step 9**: Build and Distribution - Compile to Wasm, publish to registry
- **Step 10**: Documentation - User guides, examples, troubleshooting

## Files Modified/Created

### New Files:
- `src/parser.rs` (460 lines) - Format-aware parser module
- `src/format_aware_tests.rs` (345 lines) - Format-specific integration tests

### Modified Files:
- `src/lib.rs` - Integrated FormatParser into format() method
- `README.md` - Marked Step 6 as complete

## Documentation Files:
- `docs/STEP_6_SUMMARY.md` (this file)

---

**Total Lines of Code**: ~2,000 lines (excluding generated code and dependencies)
**Total Tests**: 89 tests (100% passing)
**Test Coverage**: Config, Sorter, Extractor, Plugin, Integration, Parser, Format-Aware
**Formats Supported**: HTML, HTM, JSX, TSX, Vue, Svelte, Astro + Unknown format fallback
