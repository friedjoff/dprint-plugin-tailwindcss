# API Reference

Complete API documentation for dprint-plugin-tailwindcss developers and users.

## Configuration API

### Configuration Object

The plugin accepts the following configuration options via `dprint.json`:

```typescript
interface TailwindCssConfiguration {
  enabled?: boolean;
  tailwindFunctions?: string[];
  tailwindAttributes?: string[];
}
```

### Configuration Properties

#### `enabled`

- **Type**: `boolean`
- **Default**: `true`
- **Description**: Enable or disable the TailwindCSS plugin globally

**Example**:
```json
{
  "tailwindcss": {
    "enabled": false
  }
}
```

**Use Case**: Temporarily disable without removing plugin

---

#### `tailwindFunctions`

- **Type**: `string[]`
- **Default**: `["clsx", "cn", "cva", "tw", "classnames"]`
- **Description**: JavaScript/TypeScript function names that contain TailwindCSS classes

**Example**:
```json
{
  "tailwindcss": {
    "tailwindFunctions": ["clsx", "cn", "classNames", "twMerge"]
  }
}
```

**Use Cases**:
- Custom utility functions
- Third-party class composition libraries
- Project-specific class helpers

**Pattern Matching**:
```javascript
// Matches:
clsx("text-sm font-bold")
cn("bg-blue-500", active && "opacity-50")
classNames({ "p-4": true })

// Does not match:
className="text-sm"  // Use tailwindAttributes instead
```

---

#### `tailwindAttributes`

- **Type**: `string[]`
- **Default**: `["class", "className"]`
- **Description**: HTML/JSX attribute names that contain TailwindCSS classes

**Example**:
```json
{
  "tailwindcss": {
    "tailwindAttributes": ["class", "className", "ngClass"]
  }
}
```

**Use Cases**:
- Framework-specific attributes (Angular's `ngClass`)
- Custom component props
- Data attributes with classes

**Pattern Matching**:
```html
<!-- Matches: -->
<div class="flex items-center"></div>
<Button className="bg-blue-500 hover:bg-blue-600" />

<!-- Does not match: -->
<div data-class="text-sm"></div>  <!-- Must add "data-class" to config -->
```

---

## Plugin Interface

### Plugin Handler

The plugin implements the `SyncPluginHandler` trait from dprint-core:

```rust
pub struct TailwindCssPluginHandler;

impl SyncPluginHandler<Configuration> for TailwindCssPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo;
    fn license_text(&mut self) -> String;
    fn resolve_config(&mut self, config: ConfigKeyMap, global_config: &GlobalConfiguration) -> PluginResolveConfigurationResult<Configuration>;
    fn format(&mut self, file_path: &Path, file_text: &str, config: &Configuration, override_config: &ConfigKeyMap) -> FormatResult;
}
```

### Plugin Metadata

#### `plugin_info()`

Returns plugin identification and capabilities:

```rust
PluginInfo {
    name: "dprint-plugin-tailwindcss",
    version: env!("CARGO_PKG_VERSION"),
    config_key: "tailwindcss",
    file_extensions: vec!["html", "htm", "jsx", "tsx", "vue", "svelte", "astro"],
    file_names: vec![],
    help_url: "https://github.com/friedjoff/dprint-plugin-tailwindcss",
    config_schema_url: "",
    update_url: None,
}
```

**Fields**:
- `name`: Plugin identifier
- `version`: Semantic version from Cargo.toml
- `config_key`: Key in dprint.json ("tailwindcss")
- `file_extensions`: Supported file types
- `help_url`: Documentation link

---

#### `license_text()`

Returns MIT license text:

```rust
fn license_text(&mut self) -> String {
    include_str!("../LICENSE").to_string()
}
```

---

### Configuration Resolution

#### `resolve_config()`

Parses and validates configuration from dprint.json:

```rust
fn resolve_config(
    &mut self,
    config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration>
```

**Parameters**:
- `config`: Plugin-specific configuration
- `global_config`: Global dprint configuration

**Returns**:
- `Ok(Configuration)`: Valid configuration
- `Err(diagnostics)`: Configuration errors

**Validation**:
- Type checking (bool, string arrays)
- Default value injection
- Diagnostic messages for errors

**Example Error**:
```
Configuration Error: tailwindFunctions must be an array of strings
  Expected: ["clsx", "cn"]
  Got: "clsx"
```

---

### Formatting

#### `format()`

Main formatting function:

```rust
fn format(
    &mut self,
    file_path: &Path,
    file_text: &str,
    config: &Configuration,
    override_config: &ConfigKeyMap,
) -> FormatResult
```

**Parameters**:
- `file_path`: Path to file being formatted (for extension detection)
- `file_text`: Content to format
- `config`: Resolved configuration
- `override_config`: File-specific overrides

**Returns**:
- `Ok(Some(formatted))`: File was formatted
- `Ok(None)`: No changes needed OR file should be skipped
- `Err(error)`: Formatting error

**Behavior**:
1. Check if plugin is enabled
2. Check if file extension is supported
3. Parse file according to format
4. Extract class strings
5. Sort classes
6. Replace in original text
7. Verify no content was lost

**Example**:
```rust
// Input
let input = r#"<div class="p-4 bg-blue-500 text-white">"#;

// Output
let output = format(Path::new("test.html"), input, &config, &overrides)?;
// => Some("<div class=\"bg-blue-500 p-4 text-white\">")
```

---

## Core Modules

### Configuration Module (`config.rs`)

#### `Configuration`

Main configuration struct:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub enabled: bool,
    pub tailwind_functions: Vec<String>,
    pub tailwind_attributes: Vec<String>,
}
```

**Methods**:

##### `default()`

```rust
impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            enabled: true,
            tailwind_functions: vec![
                "clsx".to_string(),
                "cn".to_string(),
                "cva".to_string(),
                "tw".to_string(),
                "classnames".to_string(),
            ],
            tailwind_attributes: vec![
                "class".to_string(),
                "className".to_string(),
            ],
        }
    }
}
```

##### `validate()`

```rust
pub fn validate(&self) -> Result<(), Vec<ConfigurationDiagnostic>>
```

Validates configuration values and returns diagnostics for errors.

---

### File Format Module (`parser.rs`)

#### `FileFormat`

Enum for supported file formats:

```rust
pub enum FileFormat {
    Html,
    Jsx,
    Vue,
    Svelte,
    Astro,
}
```

**Methods**:

##### `from_path()`

```rust
pub fn from_path(path: &str) -> Option<FileFormat>
```

Detects file format from extension:

```rust
// Examples
FileFormat::from_path("component.vue")    // => Some(Vue)
FileFormat::from_path("index.html")       // => Some(Html)
FileFormat::from_path("Button.tsx")       // => Some(Jsx)
FileFormat::from_path("App.svelte")       // => Some(Svelte)
FileFormat::from_path("Layout.astro")     // => Some(Astro)
FileFormat::from_path("script.js")        // => None
```

---

#### `FormatParser`

Handles format-specific parsing:

```rust
pub struct FormatParser {
    format: FileFormat,
}
```

**Methods**:

##### `new()`

```rust
pub fn new(format: FileFormat) -> Self
```

##### `parse()`

```rust
pub fn parse(&self, content: &str) -> String
```

Extracts relevant content based on file format:

| Format | Behavior |
|--------|----------|
| Html | Returns full content |
| Jsx | Returns full content |
| Vue | Extracts `<template>` section |
| Svelte | Excludes `<script>` and `<style>` sections |
| Astro | Excludes frontmatter (between `---`) |

**Example**:
```rust
let parser = FormatParser::new(FileFormat::Vue);
let content = r#"
<script setup>
const name = "World";
</script>
<template>
  <div class="p-4 text-sm">Hello</div>
</template>
"#;

let parsed = parser.parse(content);
// => "<template>\n  <div class=\"p-4 text-sm\">Hello</div>\n</template>"
```

---

### Class Extraction Module (`extractor.rs`)

#### `ClassExtractor`

Extracts TailwindCSS class strings:

```rust
pub struct ClassExtractor {
    config: Configuration,
}
```

**Methods**:

##### `new()`

```rust
pub fn new(config: Configuration) -> Self
```

##### `extract()`

```rust
pub fn extract(&self, content: &str) -> Vec<ClassMatch>
```

Finds all class strings in content using regex patterns.

**Returns**: Vector of `ClassMatch` with positions and values

**Example**:
```rust
let extractor = ClassExtractor::new(config);
let html = r#"<div class="flex p-4"><span className="text-sm">Hi</span></div>"#;
let matches = extractor.extract(html);

// matches[0]: ClassMatch { start: 12, end: 20, value: "flex p-4", ... }
// matches[1]: ClassMatch { start: 36, end: 43, value: "text-sm", ... }
```

---

#### `ClassMatch`

Represents a matched class string:

```rust
pub struct ClassMatch {
    pub start: usize,
    pub end: usize,
    pub value: String,
    pub quote_char: char,
}
```

**Fields**:
- `start`: Byte offset of class string start
- `end`: Byte offset of class string end
- `value`: The class string content
- `quote_char`: Quote character used (`"` or `'`)

---

### Class Sorting Module (`sorter.rs`)

#### `sort_classes()`

Main sorting function:

```rust
pub fn sort_classes(classes: &str) -> String
```

Sorts TailwindCSS classes according to recommended order.

**Algorithm**:
1. Split classes by whitespace
2. Parse each class into components
3. Calculate priority (0-11)
4. Sort by: priority → variant order → property → value
5. Join with single space

**Example**:
```rust
let input = "hover:bg-blue-500 p-4 text-white flex items-center";
let output = sort_classes(input);
// => "flex items-center bg-blue-500 p-4 text-white hover:bg-blue-500"
```

---

#### `TailwindClass`

Parsed class representation:

```rust
pub struct TailwindClass {
    pub original: String,
    pub variants: Vec<String>,
    pub important: bool,
    pub negative: bool,
    pub property: String,
    pub value: Option<String>,
    pub arbitrary_value: Option<String>,
    pub priority: u8,
}
```

**Fields**:
- `original`: Original class string
- `variants`: Responsive/state modifiers (e.g., `["sm", "hover"]`)
- `important`: Has `!` prefix
- `negative`: Has `-` prefix
- `property`: Base property (e.g., `"bg"`, `"text"`)
- `value`: Property value (e.g., `"blue-500"`)
- `arbitrary_value`: Content of `[...]`
- `priority`: Sort priority (0-11)

---

#### `parse_class()`

```rust
pub fn parse_class(class: &str) -> TailwindClass
```

Parses a single class into components.

**Examples**:
```rust
parse_class("text-sm")
// => TailwindClass { property: "text", value: Some("sm"), priority: 6, ... }

parse_class("sm:hover:bg-blue-500")
// => TailwindClass { variants: ["sm", "hover"], property: "bg", value: Some("blue-500"), ... }

parse_class("!-mt-4")
// => TailwindClass { important: true, negative: true, property: "mt", value: Some("4"), ... }

parse_class("w-[100px]")
// => TailwindClass { property: "w", arbitrary_value: Some("100px"), ... }
```

---

#### `get_priority()`

```rust
pub fn get_priority(property: &str) -> u8
```

Returns sort priority for a property (0-11):

| Priority | Category | Examples |
|----------|----------|----------|
| 0 | Container | `container` |
| 1 | Layout | `box-border`, `block`, `inline` |
| 2 | Positioning | `static`, `fixed`, `absolute`, `top`, `left` |
| 3 | Display | `flex`, `grid`, `table`, `hidden` |
| 4 | Spacing | `m-*`, `p-*`, `space-*`, `gap-*` |
| 5 | Sizing | `w-*`, `h-*`, `min-*`, `max-*` |
| 6 | Typography | `text-*`, `font-*`, `leading-*` |
| 7 | Backgrounds & Borders | `bg-*`, `border-*`, `rounded-*` |
| 8 | Effects | `shadow-*`, `opacity-*` |
| 9 | Filters | `blur-*`, `brightness-*` |
| 10 | Transitions | `transition-*`, `duration-*`, `animate-*` |
| 11 | Other | Everything else |

---

#### `get_variant_priority()`

```rust
pub fn get_variant_priority(variant: &str) -> u8
```

Returns priority for variant modifiers:

| Priority | Variants |
|----------|----------|
| 0 | Responsive: `sm`, `md`, `lg`, `xl`, `2xl` |
| 1 | Dark mode: `dark` |
| 2 | State: `hover`, `focus`, `active`, `disabled` |
| 3 | Group/Peer: `group-*`, `peer-*` |
| 4 | Other: Custom variants |

---

## Utility Functions

### Position Tracking

#### `adjust_position()`

```rust
fn adjust_position(original_pos: usize, replacements: &[(usize, usize, String)]) -> usize
```

Adjusts byte positions after text replacements.

**Parameters**:
- `original_pos`: Original position in text
- `replacements`: List of (start, end, new_text) replacements

**Returns**: Adjusted position accounting for size changes

---

### Content Validation

#### `validate_content()`

```rust
fn validate_content(original: &str, formatted: &str) -> Result<(), String>
```

Ensures formatting didn't lose content:

**Checks**:
- Line count unchanged
- Comments preserved
- Non-whitespace characters preserved

**Returns**:
- `Ok(())`: Content valid
- `Err(message)`: Validation failed

---

## WASM Exports

The plugin exports the following functions via WASM:

```rust
#[wasm_bindgen]
pub fn plugin_info() -> String;

#[wasm_bindgen]
pub fn license_text() -> String;

#[wasm_bindgen]
pub fn resolve_config(config: String, global_config: String) -> String;

#[wasm_bindgen]
pub fn format(file_path: String, file_text: String, config: String, override_config: String) -> String;
```

These are generated automatically by the `generate_plugin_code!` macro from dprint-core.

---

## Error Handling

### Error Types

#### `FormatError`

```rust
pub enum FormatError {
    ParseError(String),
    ValidationError(String),
    ConfigError(String),
}
```

#### `ConfigurationDiagnostic`

```rust
pub struct ConfigurationDiagnostic {
    pub property_name: String,
    pub message: String,
}
```

---

## Constants

### Default Functions

```rust
pub const DEFAULT_TAILWIND_FUNCTIONS: &[&str] = &[
    "clsx",
    "cn",
    "cva",
    "tw",
    "classnames",
];
```

### Default Attributes

```rust
pub const DEFAULT_TAILWIND_ATTRIBUTES: &[&str] = &[
    "class",
    "className",
];
```

### Supported Extensions

```rust
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "html",
    "htm",
    "jsx",
    "tsx",
    "vue",
    "svelte",
    "astro",
];
```

---

## Usage Examples

### Basic Usage

```rust
use dprint_plugin_tailwindcss::{Configuration, sort_classes};

let config = Configuration::default();
let input = "hover:bg-blue-500 p-4 text-white flex";
let output = sort_classes(input);
// => "flex bg-blue-500 p-4 text-white hover:bg-blue-500"
```

### Custom Configuration

```rust
use dprint_plugin_tailwindcss::Configuration;

let config = Configuration {
    enabled: true,
    tailwind_functions: vec!["myHelper".to_string()],
    tailwind_attributes: vec!["myClass".to_string()],
};
```

### File Format Detection

```rust
use dprint_plugin_tailwindcss::FileFormat;

let format = FileFormat::from_path("Button.tsx");
assert_eq!(format, Some(FileFormat::Jsx));
```

### Parsing Vue Template

```rust
use dprint_plugin_tailwindcss::{FileFormat, FormatParser};

let parser = FormatParser::new(FileFormat::Vue);
let content = r#"
<script>export default { name: 'App' }</script>
<template><div class="flex"></div></template>
"#;
let parsed = parser.parse(content);
// Only template content extracted
```

---

## Type Definitions

### Rust Types

```rust
pub type FormatResult = Result<Option<String>, String>;
pub type ConfigResult = Result<Configuration, Vec<ConfigurationDiagnostic>>;
```

### TypeScript Types (for dprint.json)

```typescript
interface DprintConfig {
  tailwindcss?: {
    enabled?: boolean;
    tailwindFunctions?: string[];
    tailwindAttributes?: string[];
  };
}
```

---

## Version History

See [CHANGELOG.md](../CHANGELOG.md) for version history.

---

## See Also

- [Architecture Guide](ARCHITECTURE.md)
- [Configuration Reference](CONFIGURATION.md)
- [Plugin Compatibility](PLUGIN_COMPATIBILITY.md)
- [Performance Guide](PERFORMANCE.md)

---

**API Version**: 0.1.0  
**Last Updated**: 2025-10-24
