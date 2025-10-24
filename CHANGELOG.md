# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of dprint-plugin-tailwindcss
- Automatic TailwindCSS class sorting using official ordering
- Support for 6 file formats: HTML, JSX, TSX, Vue, Svelte, Astro
- Format-aware parsing (Vue templates, Svelte markup, Astro frontmatter)
- Configuration options for custom functions and attributes
- 240 comprehensive tests (100% passing)
- TailwindCSS v4 support (container queries, data attributes, ARIA)
- Arbitrary values and variants support
- Variant priority ordering (responsive, state, dark mode, etc.)
- 12-level property priority system
- Plugin ecosystem compatibility (works with TypeScript, JSON, Markdown plugins)
- Comprehensive documentation (user guide + developer docs)

### Features
- **Class Sorting**: Official TailwindCSS ordering with 12-level priority
- **File Formats**: HTML, JSX/TSX, Vue, Svelte, Astro
- **Modern Features**: Container queries, data/ARIA attributes, arbitrary values
- **Configurable**: Custom function names (clsx, cn, etc.) and attributes
- **Fast**: O(n) parsing, WebAssembly performance
- **Compatible**: Works alongside other dprint plugins

### Configuration
- `enabled`: Enable/disable plugin (default: true)
- `tailwindFunctions`: Function names containing classes (default: ["clsx", "cn", "cva", "tw", "classnames"])
- `tailwindAttributes`: HTML attributes to format (default: ["class", "className"])

### Tested
- 240 tests covering all functionality
- Edge cases, performance, real-world scenarios
- prettier-plugin-tailwindcss compatibility verified
- Multi-plugin ecosystem compatibility

## [0.1.0] - YYYY-MM-DD

### Added
- Initial alpha release
- Basic class sorting functionality
- Support for HTML, JSX, Vue, Svelte, Astro
- Configuration system
- Comprehensive test suite

---

**Note**: Versions will be updated as releases are published.
