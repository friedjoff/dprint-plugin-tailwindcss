# Architecture

## Overview

This document provides an in-depth look at the architecture of dprint-plugin-tailwindcss, including design decisions, data structures, and implementation details.

## Core Principles

1. **Format-Aware**: Understand different file structures (Vue templates, Svelte markup, etc.)
2. **Structure Preservation**: Never modify non-class code, comments, or whitespace
3. **Performance**: O(n) complexity for parsing and extraction
4. **Extensibility**: Easy to add new file formats and utilities
5. **Compatibility**: Works alongside other dprint plugins without conflicts

## Module Architecture

### 1. Plugin Handler (`lib.rs`)

**Purpose**: Main entry point implementing the dprint plugin interface

**Key Components**:
```rust
struct TailwindCssPluginHandler;

impl SyncPluginHandler<Configuration> for TailwindCssPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo { }
    fn license_text(&mut self) -> String { }
    fn resolve_config(...) -> PluginResolveConfigurationResult<Configuration> { }
    fn format(...) -> FormatResult { }
}
```

**Responsibilities**:
- Implement dprint's `SyncPluginHandler` trait
- Coordinate between parser, extractor, and sorter
- Handle format result generation
- Manage file matching and plugin enablement

**Data Flow**:
```
Input bytes → UTF-8 conversion → Format detection →
Parser → Extractor → Sorter → String replacement → Output bytes
```

### 2. Configuration (`config.rs`)

**Purpose**: Schema definition and validation for plugin configuration

**Data Structure**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub enabled: bool,
    pub tailwind_functions: Vec<String>,
    pub tailwind_attributes: Vec<String>,
}
```

**Key Features**:
- Serde-based JSON deserialization
- Default value provision
- Validation with diagnostics
- File extension matching configuration

**Configuration Resolution**:
```
ConfigKeyMap → parse values → validate → Configuration + diagnostics
```

### 3. Class Sorter (`sorter.rs`)

**Purpose**: Parse and sort TailwindCSS class names

**Core Algorithm**:

```
Input: "sm:hover:bg-blue-500 p-4 !mt-2"
                 ↓
        Parse each class
                 ↓
   ┌──────────────────────────┐
   │ ParsedClass {            │
   │   variants: ["sm","hover"]│
   │   is_important: false    │
   │   is_negative: false     │
   │   property: "bg"         │
   │   value: "blue-500"      │
   │   arbitrary: None        │
   │ }                        │
   └──────────────────────────┘
                 ↓
    Calculate priority scores
                 ↓
    ┌─────────────────────────┐
    │ Score Calculation:      │
    │ 1. Variant priority     │
    │ 2. Property category    │
    │ 3. Stable sort          │
    └─────────────────────────┘
                 ↓
         Sort by score
                 ↓
Output: "!mt-2 p-4 sm:hover:bg-blue-500"
```

**Priority System**:

```rust
// 12-level property priority
Priority 0:  container, @container
Priority 1:  block, inline, flex, grid
Priority 2:  static, fixed, absolute, relative
Priority 3:  hidden, visible
Priority 4:  margin, padding (m-*, p-*)
Priority 5:  width, height (w-*, h-*)
Priority 6:  font-*, text-*, leading-*
Priority 7:  bg-*, border-*
Priority 8:  shadow, opacity
Priority 9:  filter, blur, brightness
Priority 10: transition, duration, animate
Priority 11: other utilities

// Variant priority (ordered)
1. Responsive: sm, md, lg, xl, 2xl
2. State: hover, focus, active, disabled
3. Dark mode: dark
4. Group/peer: group-*, peer-*
5. Container queries: @container, @lg
6. Data/ARIA: data-*, aria-*
7. Print: print
```

**ParsedClass Structure**:
```rust
struct ParsedClass {
    original: String,          // Original class text
    variants: Vec<String>,     // E.g., ["sm", "hover"]
    is_important: bool,        // Starts with !
    is_negative: bool,         // Starts with -
    property: String,          // E.g., "bg", "text", "mt"
    value: Option<String>,     // E.g., "blue-500", "center"
    arbitrary: Option<String>, // E.g., "[#bada55]"
}
```

### 4. Class Extractor (`extractor.rs`)

**Purpose**: Extract class strings from various contexts

**Extraction Types**:

1. **HTML/JSX Attributes**:
   ```html
   <div class="..." className="...">
   ```
   Regex: `(\w+)=["']([^"']*?)["']`

2. **Utility Functions**:
   ```javascript
   clsx("...", "...")
   cn("...")
   ```
   Regex: `(function_name)\s*\(\s*["']([^"']*?)["']`

3. **Position Tracking**:
   ```rust
   struct ClassMatch {
       start: usize,    // Start byte position
       end: usize,      // End byte position
       content: String, // Extracted class string
   }
   ```

**Extraction Flow**:
```
Input text
    │
    ├─ Scan for attributes (class="...")
    │   └─ Extract position + content
    │
    ├─ Scan for functions (clsx("..."))
    │   └─ Extract position + content
    │
    └─ Return Vec<ClassMatch>
```

### 5. Format Parser (`parser.rs`)

**Purpose**: Format-aware parsing for different file types

**Format Detection**:
```rust
pub enum FileFormat {
    Html,    // .html, .htm
    Jsx,     // .jsx, .tsx
    Vue,     // .vue
    Svelte,  // .svelte
    Astro,   // .astro
}

impl FileFormat {
    pub fn from_path(path: &str) -> Option<Self> {
        // Extension-based detection
    }
}
```

**Format-Specific Parsing**:

**Vue**:
```
Input:
<template>...</template>
<script>...</script>
<style>...</style>
    ↓
Extract: <template>...</template> only
    ↓
Parse classes within template
```

**Svelte**:
```
Input:
<script>...</script>
<div class="...">...</div>
<style>...</style>
    ↓
Extract: Everything except <script> and <style>
    ↓
Parse classes in markup
```

**Astro**:
```
Input:
---
const x = 1;
---
<div class="...">...</div>
    ↓
Find frontmatter end (---)
    ↓
Parse classes after frontmatter
```

**Parser Architecture**:
```rust
pub struct FormatParser {
    extractor: ClassExtractor,
}

impl FormatParser {
    pub fn parse(&self, content: &str, format: FileFormat) -> Vec<ClassMatch> {
        match format {
            FileFormat::Html => self.extractor.extract_all(content),
            FileFormat::Jsx => self.extractor.extract_all(content),
            FileFormat::Vue => self.parse_vue_template(content),
            FileFormat::Svelte => self.parse_svelte_markup(content),
            FileFormat::Astro => self.parse_astro_content(content),
        }
    }
}
```

### 6. Plugin Integration (`integration.rs`)

**Purpose**: Ensure compatibility with dprint plugin ecosystem

**Key Functions**:

1. **Format Decision**:
   ```rust
   fn should_format(path: &str) -> bool {
       // Check if file should be formatted
   }
   
   fn should_defer(path: &str) -> bool {
       // Check if another plugin should handle this
   }
   ```

2. **Preservation Guards**:
   ```rust
   fn preserve_structure(original: &str, formatted: &str) -> bool {
       // Verify line count unchanged
       // Verify comments preserved
       // Verify non-class content identical
   }
   ```

3. **Host Communication**:
   ```rust
   struct HostFormatter;
   // Placeholder for future host format requests
   ```

## Data Structures

### ClassMatch

```rust
pub struct ClassMatch {
    pub start: usize,     // Byte position in file
    pub end: usize,       // End byte position
    pub content: String,  // Extracted class string
}
```

**Usage**:
- Tracks exact position of class strings
- Enables precise string replacement
- Preserves surrounding code

### ParsedClass

```rust
struct ParsedClass {
    original: String,          // "sm:hover:!bg-blue-500"
    variants: Vec<String>,     // ["sm", "hover"]
    is_important: bool,        // true
    is_negative: bool,         // false
    property: String,          // "bg"
    value: Option<String>,     // Some("blue-500")
    arbitrary: Option<String>, // None
}
```

**Priority Calculation**:
```rust
fn calculate_priority(class: &ParsedClass) -> (u16, u8, usize) {
    let variant_priority = calculate_variant_priority(&class.variants);
    let category_priority = get_category_priority(&class.property);
    let original_index = /* stable sort index */;
    
    (variant_priority, category_priority, original_index)
}
```

## Algorithm Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| File parsing | O(n) | Single pass through file |
| Class extraction | O(m) | m = number of attributes/functions |
| Class parsing | O(k) | k = class string length |
| Priority calculation | O(1) | Hash map lookups |
| Sorting | O(c log c) | c = number of classes |
| String replacement | O(n) | Single pass with offset tracking |
| **Total** | **O(n + m + c log c)** | Linear in file size |

## Memory Layout

```
Input File (bytes)
    ↓
String (UTF-8)
    ↓
Vec<ClassMatch>  // Typically small (< 100 items)
    ↓
Vec<ParsedClass> // Per class match
    ↓
Sorted Vec<String> // Output classes
    ↓
Modified String
    ↓
Output bytes
```

**Memory Efficiency**:
- Lazy regex compilation (`once_cell`)
- Minimal allocations (string reuse)
- No AST generation (regex-based)

## Error Handling

```rust
// UTF-8 validation
String::from_utf8(bytes)
    .map_err(|e| anyhow::anyhow!("Failed to parse file as UTF-8: {}", e))?

// Format detection fallback
let format = FileFormat::from_path(path)
    .unwrap_or_else(|| /* fallback to basic extraction */);

// Graceful degradation
if matches.is_empty() {
    return Ok(None); // No changes needed
}
```

## Plugin Interface

**dprint Integration Points**:

```rust
// Plugin metadata
fn plugin_info() -> PluginInfo {
    PluginInfo {
        name: "dprint-plugin-tailwindcss",
        version: env!("CARGO_PKG_VERSION"),
        config_key: "tailwindcss",
        // ...
    }
}

// Configuration resolution
fn resolve_config(
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration>

// Main formatting
fn format(
    request: SyncFormatRequest<Configuration>,
    _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
) -> FormatResult
```

## Design Decisions

### 1. Regex-Based vs AST-Based Parsing

**Decision**: Regex-based extraction  
**Rationale**:
- ✅ Simpler implementation
- ✅ Better performance (no AST construction)
- ✅ Format-agnostic (works with templates)
- ❌ Less precise (can have false positives)
- ❌ Limited to string literals

### 2. Section-Aware Parsing

**Decision**: Format-specific parsers  
**Rationale**:
- ✅ Avoids formatting script sections
- ✅ Handles component frameworks correctly
- ✅ Prevents false positives in code
- ✅ Preserves framework semantics

### 3. No Duplicate Removal

**Decision**: Preserve duplicates  
**Rationale**:
- ✅ Predictable behavior
- ✅ Preserves intentional patterns
- ✅ Avoids complex deduplication logic
- ⚠️ User may need to clean up manually

### 4. Priority-Based Sorting

**Decision**: 12-level category priority + variant priority  
**Rationale**:
- ✅ Matches TailwindCSS official order
- ✅ Deterministic output
- ✅ Extensible for new utilities
- ✅ Handles complex variant chains

### 5. cdylib Crate Type

**Decision**: Pure cdylib for WASM  
**Rationale**:
- ✅ Smallest WASM binary
- ✅ Fastest compilation
- ❌ No integration tests possible
- ⚠️ Requires unit tests only

## Future Enhancements

### Potential Additions

1. **AST-Based Parsing**
   - More precise extraction
   - Support for template literals
   - Better JSX expression handling

2. **Custom Order Configuration**
   - User-defined priority rules
   - Project-specific sorting

3. **tailwind.config.js Integration**
   - Read custom utilities
   - Support plugin-generated classes
   - Dynamic priority assignment

4. **Performance Optimizations**
   - Parallel class sorting
   - Cached regex compilation
   - Incremental parsing

5. **Additional Formats**
   - PHP templates (Blade)
   - Ruby templates (ERB)
   - Twig templates
   - Handlebars

---

**Last Updated**: Step 8 completion  
**Architecture Status**: Stable and production-ready
