# Parsing Strategy Architecture

## Overview

The dprint-plugin-tailwindcss uses a sophisticated **format-aware parsing strategy** to extract TailwindCSS classes from various file formats while preserving their original structure.

## Design Philosophy

### 1. Format-Specific Intelligence
Different file formats have different structures. A Vue component has `<template>`, `<script>`, and `<style>` sections. An Astro file has frontmatter. The parser understands these differences and handles each format appropriately.

### 2. Structure Preservation
The parser **never modifies** code outside of class strings:
- Comments remain unchanged
- Whitespace and indentation preserved
- JavaScript/TypeScript code untouched
- CSS/SCSS code untouched

### 3. Position Accuracy
Every extracted class string includes its exact position in the original file, enabling precise replacements without affecting surrounding code.

### 4. Fallback Safety
For unknown or unsupported formats, the parser falls back to basic extraction, ensuring the plugin still works (with reduced accuracy).

## Architecture Diagram

```
                          ┌──────────────────┐
                          │   format()       │
                          │  (Main Handler)  │
                          └────────┬─────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │   FileFormat::from_path()   │
                    │   (Detect file type)        │
                    └──────────────┬──────────────┘
                                   │
                ┌──────────────────┴──────────────────┐
                │                                     │
         Format detected?                      No format?
                │                                     │
                ▼                                     ▼
      ┌─────────────────────┐            ┌──────────────────────┐
      │   FormatParser      │            │  Basic Extraction    │
      │   parse(format)     │            │  (Fallback)          │
      └─────────┬───────────┘            └──────────┬───────────┘
                │                                    │
    ┌───────────┴───────────┐                       │
    │  Format-Specific      │                       │
    │  Parsing Logic        │                       │
    └───────────┬───────────┘                       │
                │                                    │
    ┌───────────┴────────────────────────┬──────────┴──────┐
    │                                    │                  │
    ▼                                    ▼                  ▼
┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────────┐
│ HTML/JSX   │  │ Vue        │  │ Svelte     │  │ Astro        │
│ Parser     │  │ Parser     │  │ Parser     │  │ Parser       │
│            │  │            │  │            │  │              │
│ Full file  │  │ Template   │  │ Markup     │  │ After        │
│ parsing    │  │ section    │  │ sections   │  │ frontmatter  │
│            │  │ only       │  │ only       │  │              │
└────────────┘  └────────────┘  └────────────┘  └──────────────┘
                                    │
                                    ▼
                          ┌──────────────────────┐
                          │  ClassExtractor      │
                          │  - extract_from_     │
                          │    attributes()      │
                          │  - extract_from_     │
                          │    functions()       │
                          └──────────┬───────────┘
                                     │
                                     ▼
                          ┌──────────────────────┐
                          │  Vec<ClassMatch>     │
                          │  (Position + Content)│
                          └──────────┬───────────┘
                                     │
                                     ▼
                          ┌──────────────────────┐
                          │  sort_classes()      │
                          │  (TailwindCSS order) │
                          └──────────┬───────────┘
                                     │
                                     ▼
                          ┌──────────────────────┐
                          │  Replace in place    │
                          │  (Offset tracking)   │
                          └──────────────────────┘
```

## Format-Specific Strategies

### HTML/HTM Format
**File Extensions**: `.html`, `.htm`

**Strategy**: Direct full-file parsing
- Parse entire file content
- Extract from `class="..."` attributes
- Check for inline scripts with utility functions

**Why**: HTML files are straightforward with no special sections to exclude.

**Example**:
```html
<!DOCTYPE html>
<html>
<body>
  <!-- Comment preserved -->
  <div class="z-10 p-4 mt-2">Content</div>
</body>
</html>
```

### JSX Format
**File Extensions**: `.jsx`

**Strategy**: Parse className attributes and utility functions
- Extract from `className="..."` and `class="..."`
- Parse template literals: `className={\`...\`}`
- Parse utility functions: `clsx()`, `cn()`, etc.

**Why**: JSX uses `className` instead of `class` and supports dynamic expressions.

**Example**:
```jsx
export function Button({ active }) {
  return (
    <button className={clsx("z-10 p-4 mt-2", active && "bg-blue-500")}>
      Click me
    </button>
  );
}
```

### TSX Format
**File Extensions**: `.tsx`

**Strategy**: Same as JSX
- TypeScript types preserved
- JSX syntax handled identically to `.jsx`

**Why**: TSX is JSX with TypeScript types. The class extraction logic is identical.

**Example**:
```tsx
interface Props {
  active: boolean;
}

export function Button({ active }: Props) {
  return (
    <button className="z-10 p-4 mt-2">
      Click me
    </button>
  );
}
```

### Vue Format
**File Extensions**: `.vue`

**Strategy**: Template-section-only parsing with fallback

**Process**:
1. Locate `<template>` opening tag
2. Find `</template>` closing tag
3. Extract classes only from template content
4. Adjust positions for template offset
5. If no template found, parse entire file (fallback)

**Why**: Vue Single-File Components have three distinct sections:
- `<template>`: HTML-like markup (PARSE THIS)
- `<script>`: JavaScript/TypeScript (IGNORE)
- `<style>`: CSS (IGNORE)

We only want to sort classes in the template section.

**Example**:
```vue
<template>
  <div class="z-10 p-4 mt-2">
    <h1 class="text-2xl font-bold">{{ title }}</h1>
  </div>
</template>

<script>
export default {
  data() {
    return {
      title: "Hello",
      // This className is JavaScript, NOT HTML
      className: "z-10 p-4 mt-2" // IGNORED
    }
  }
}
</script>

<style scoped>
.container {
  /* CSS classes are IGNORED */
}
</style>
```

**Implementation**:
```rust
fn extract_vue_template(content: &str) -> Option<ContentSection> {
    let template_start_tag = content.find("<template")?;
    let template_content_start = content[template_start_tag..].find('>')? 
        + template_start_tag + 1;
    let template_end = content.find("</template>")?;
    
    Some(ContentSection {
        start: template_content_start,
        content: content[template_content_start..template_end].to_string(),
    })
}
```

### Svelte Format
**File Extensions**: `.svelte`

**Strategy**: Extract markup sections, excluding `<script>` and `<style>`

**Process**:
1. Scan for all `<script>` tags and their closing tags
2. Scan for all `<style>` tags and their closing tags
3. Mark these as "excluded ranges"
4. Extract markup sections between excluded ranges
5. Parse classes in each markup section
6. Adjust positions for each section's offset

**Why**: Svelte components mix markup, script, and style at the top level. Unlike Vue, there's no wrapper `<template>` tag. We need to extract the markup while excluding script/style blocks.

**Example**:
```svelte
<script>
  let count = 0;
  $: doubled = count * 2; // Reactive statement
  let className = "ignored"; // JavaScript IGNORED
</script>

<div class="z-10 p-4 mt-2">
  <button class="bg-blue-500" on:click={() => count++}>
    Count: {count}
  </button>
  
  {#if count > 5}
    <div class="text-green-500">High!</div>
  {/if}
</div>

<style>
  div {
    /* CSS IGNORED */
  }
</style>
```

**Implementation**:
```rust
fn extract_svelte_markup_sections(content: &str) -> Vec<ContentSection> {
    let mut excluded_ranges = Vec::new();
    
    // Find all <script>...</script> ranges
    while let Some(script_start) = find_next_script(content, search_pos) {
        if let Some(script_end) = find_script_end(content, script_start) {
            excluded_ranges.push((script_start, script_end));
        }
    }
    
    // Find all <style>...</style> ranges
    // ... similar process ...
    
    // Sort excluded ranges
    excluded_ranges.sort_by_key(|r| r.0);
    
    // Extract sections between excluded ranges
    let mut sections = Vec::new();
    let mut current_pos = 0;
    
    for (start, end) in excluded_ranges {
        if current_pos < start {
            sections.push(ContentSection {
                start: current_pos,
                content: content[current_pos..start].to_string(),
            });
        }
        current_pos = end;
    }
    
    // Add remaining content
    if current_pos < content.len() {
        sections.push(ContentSection {
            start: current_pos,
            content: content[current_pos..].to_string(),
        });
    }
    
    sections
}
```

### Astro Format
**File Extensions**: `.astro`

**Strategy**: Skip frontmatter, parse markup

**Process**:
1. Check if file starts with `---` (frontmatter marker)
2. If yes, find closing `---`
3. Calculate markup start position (after frontmatter)
4. Parse classes in markup section only
5. Adjust positions for frontmatter offset
6. If no frontmatter, parse entire file

**Why**: Astro components have optional frontmatter (TypeScript/JavaScript between `---` markers) followed by JSX-like markup. We only want to sort classes in the markup.

**Example**:
```astro
---
// Frontmatter: TypeScript/JavaScript
const title = "Hello World";
const className = "ignored"; // JavaScript IGNORED

interface Props {
  active: boolean;
}

const { active } = Astro.props;
---

<div class="z-10 p-4 mt-2">
  <h1 class="text-2xl font-bold">{title}</h1>
  <button class={clsx("bg-blue-500", active && "opacity-50")}>
    Click me
  </button>
</div>
```

**Implementation**:
```rust
fn find_astro_frontmatter_end(content: &str) -> Option<usize> {
    // Check if file starts with ---
    if !content.trim_start().starts_with("---") {
        return None;
    }
    
    // Find opening ---
    let start = content.find("---")? + 3;
    
    // Find closing ---
    let remaining = &content[start..];
    let end = remaining.find("---")? + start + 3;
    
    // Return position after closing --- and newline
    if let Some(newline) = content[end..].find('\n') {
        Some(end + newline + 1)
    } else {
        Some(end)
    }
}
```

## Position Tracking

Every format parser maintains accurate position information to enable precise replacements.

### Position Calculation Example (Vue):

```vue
<template>
  <div class="flex p-4">Content</div>
</template>
```

**Process**:
1. Find template section: start at position 11 (after `<template>`)
2. Extract classes: "flex p-4" found at position 19-27 (relative to template)
3. Adjust position: 19 + 11 = 30 (absolute position in file)
4. Store: `ClassMatch { start: 30, end: 38, content: "flex p-4" }`
5. Replace: `file[30..38]` with sorted classes

**Why**: Without position adjustment, replacements would be in the wrong location.

### Position Calculation Example (Svelte with multiple sections):

```svelte
<div class="a">Section 1</div>

<script>
  let x = 1;
</script>

<div class="b">Section 2</div>
```

**Process**:
1. Section 1: starts at 0, contains "a" at position 12-13 (relative)
   - Absolute: 0 + 12 = 12
2. Section 2: starts at 59 (after `</script>`), contains "b" at position 12-13 (relative)
   - Absolute: 59 + 12 = 71

**Result**:
- ClassMatch { start: 12, end: 13, content: "a" }
- ClassMatch { start: 71, end: 72, content: "b" }

## Class Extraction Process

After format-specific parsing determines which content to analyze, the `ClassExtractor` performs the actual extraction:

### 1. Attribute Extraction
**Pattern**: `class="..."` or `className="..."`

```rust
pub fn extract_from_attributes(&self, content: &str) -> Vec<ClassMatch> {
    let mut matches = Vec::new();
    
    for attr_name in &self.attribute_names {
        // Match class="..." or className="..."
        let pattern = format!(r#"{}=["']([^"']*)["']"#, regex::escape(attr_name));
        let re = Regex::new(&pattern)?;
        
        for cap in re.captures_iter(content) {
            if let Some(classes) = cap.get(1) {
                matches.push(ClassMatch {
                    start: classes.start(),
                    end: classes.end(),
                    content: classes.as_str().to_string(),
                });
            }
        }
    }
    
    matches
}
```

### 2. Function Extraction
**Pattern**: `clsx("...")` or `cn("...")`

```rust
pub fn extract_from_functions(&self, content: &str) -> Vec<ClassMatch> {
    let mut matches = Vec::new();
    
    for func_name in &self.function_names {
        // Match clsx("...") or cn("...")
        let pattern = format!(r#"{}s*\([^)]+\)"#, regex::escape(func_name));
        let re = Regex::new(&pattern)?;
        
        for cap in re.captures_iter(content) {
            // Extract string literals from function arguments
            let strings = extract_strings_from_args(cap.as_str());
            matches.extend(strings);
        }
    }
    
    matches
}
```

## Error Handling

### Invalid UTF-8
```rust
let file_text = String::from_utf8(request.file_bytes.to_vec())
    .map_err(|e| anyhow::anyhow!("Failed to parse file as UTF-8: {}", e))?;
```

**Behavior**: Returns error to dprint, file is not modified.

### Unknown File Format
```rust
let matches = if let Some(format) = format {
    let parser = FormatParser::new(extractor);
    parser.parse(&file_text, format)
} else {
    // Fallback to basic extraction
    let mut matches = extractor.extract_from_attributes(&file_text);
    let function_matches = extractor.extract_from_functions(&file_text);
    matches.extend(function_matches);
    matches
};
```

**Behavior**: Falls back to basic extraction (less accurate but still functional).

### No Template Section (Vue)
```rust
if let Some(template_section) = extract_vue_template(content) {
    // Parse template section
} else {
    // Fallback: parse entire file
    let matches = extractor.extract_from_attributes(content);
}
```

**Behavior**: Falls back to full-file parsing if template section not found.

## Performance Characteristics

### Time Complexity:
- **Format detection**: O(1) - simple string suffix check
- **HTML/JSX parsing**: O(n) - single pass through content
- **Vue parsing**: O(n) - single scan for template boundaries
- **Svelte parsing**: O(n) - linear scan for script/style tags
- **Astro parsing**: O(n) - linear scan for frontmatter markers
- **Class extraction**: O(n) - regex matching (compiled once)

### Space Complexity:
- **ContentSection**: O(k) where k = section count (typically 1-3)
- **ClassMatch**: O(m) where m = number of class attributes
- **Position tracking**: O(1) per match

### Optimization Techniques:
1. **Lazy regex compilation**: Regex patterns compiled once via `once_cell`
2. **Early exit**: Astro parser exits early if no frontmatter detected
3. **Single allocation**: ContentSection structs created once, reused
4. **No re-parsing**: Positions calculated during extraction, not recalculated

## Testing Strategy

### Unit Tests (Parser Module):
- Format detection from file paths
- Section extraction for each format
- Position tracking accuracy
- Edge cases (missing sections, multiple blocks)

### Integration Tests (Format-Aware):
- End-to-end parsing for each format
- Section isolation (Vue templates, Svelte markup, Astro frontmatter)
- Structure preservation (comments, whitespace, non-class code)
- Multi-line and complex structures

### Coverage:
- ✅ All file formats (HTML, JSX, TSX, Vue, Svelte, Astro)
- ✅ Section-specific parsing
- ✅ Position accuracy
- ✅ Fallback behavior
- ✅ Error handling

## Future Enhancements

### Potential Additions:
1. **PHP Templates**: `.php` files with HTML
2. **ERB Templates**: `.erb` files (Ruby on Rails)
3. **Blade Templates**: `.blade.php` files (Laravel)
4. **Twig Templates**: `.twig` files (Symfony)
5. **Handlebars/Mustache**: `.hbs`, `.mustache` files

### Implementation Strategy:
Each new format would follow the same pattern:
1. Add enum variant to `FileFormat`
2. Implement format-specific parser method
3. Add format detection to `from_path()`
4. Write comprehensive tests
5. Document behavior in this file

## Conclusion

The format-aware parsing strategy provides:
- ✅ **Accuracy**: Format-specific logic prevents false matches
- ✅ **Safety**: Non-class code never modified
- ✅ **Robustness**: Fallback for unknown formats
- ✅ **Performance**: Linear time complexity
- ✅ **Extensibility**: Easy to add new formats

This architecture ensures the plugin works correctly across all supported file types while maintaining the integrity of the original code structure.
