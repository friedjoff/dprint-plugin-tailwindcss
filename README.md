# dprint-plugin-tailwindcss

A [dprint](https://dprint.dev/) plugin for sorting TailwindCSS classes, providing the same automatic class ordering as [prettier-plugin-tailwindcss](https://github.com/tailwindlabs/prettier-plugin-tailwindcss).

[![Tests](https://img.shields.io/badge/tests-240%20passing-brightgreen)]()
[![Coverage](https://img.shields.io/badge/coverage-100%25-brightgreen)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- 🎯 **Automatic TailwindCSS class sorting** using official ordering
- 🚀 **Fast and lightweight** - Compiled to WebAssembly
- 📦 **Multi-framework support** - HTML, JSX, TSX, Vue, Svelte, Astro
- 🔧 **Highly configurable** - Custom functions, attributes, and options
- 🎨 **Format-aware** - Understands different file structures
- 🔗 **Plugin-compatible** - Works alongside other dprint plugins

## Installation

Add the plugin to your `dprint.json` configuration:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ]
}
```

Or use the dprint CLI:

```bash
dprint config add tailwindcss
```

## Quick Start

Once installed, the plugin will automatically format TailwindCSS classes in supported file types:

**Before:**
```html
<div class="text-center font-bold p-4 bg-blue-500 mt-2">
  Hello World
</div>
```

**After:**
```html
<div class="mt-2 bg-blue-500 p-4 font-bold text-center">
  Hello World
</div>
```

## Configuration

Add a `"tailwindcss"` section to your `dprint.json`:

```json
{
  "tailwindcss": {
    "enabled": true,
    "tailwindFunctions": ["clsx", "cn", "cva", "tw"],
    "tailwindAttributes": ["class", "className"]
  },
  "plugins": [
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ]
}
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable or disable the plugin |
| `tailwindFunctions` | string[] | `["clsx", "cn", "cva", "tw", "classnames"]` | Function names that contain class lists |
| `tailwindAttributes` | string[] | `["class", "className"]` | HTML/JSX attributes to format |

### Example Configurations

#### React/Next.js Project

```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn", "clsx", "cva"],
    "tailwindAttributes": ["className"]
  }
}
```

#### Vue Project

```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn"],
    "tailwindAttributes": ["class"]
  }
}
```

#### Multi-Framework Project

```json
{
  "tailwindcss": {
    "tailwindFunctions": ["clsx", "cn", "tw"],
    "tailwindAttributes": ["class", "className", "data-class"]
  }
}
```

## Supported File Formats

The plugin automatically detects and formats TailwindCSS classes in:

- **HTML** (`.html`, `.htm`)
- **React** (`.jsx`, `.tsx`)
- **Vue** (`.vue`) - Template section only
- **Svelte** (`.svelte`) - Markup section only
- **Astro** (`.astro`) - Post-frontmatter only

## Examples

### HTML

```html
<!-- Input -->
<button class="hover:bg-blue-700 text-white font-bold py-2 px-4 rounded bg-blue-500">
  Click me
</button>

<!-- Output -->
<button class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700">
  Click me
</button>
```

### React with clsx

```jsx
// Input
import clsx from 'clsx';

<div className={clsx(
  'text-center',
  'font-bold',
  isActive && 'bg-blue-500 text-white',
  'p-4 mt-2'
)}>
  Content
</div>

// Output
<div className={clsx(
  'font-bold text-center',
  isActive && 'bg-blue-500 text-white',
  'mt-2 p-4'
)}>
  Content
</div>
```

### Vue Template

```vue
<!-- Input -->
<template>
  <div class="hover:shadow-lg transition-shadow p-6 bg-white rounded-lg shadow">
    {{ content }}
  </div>
</template>

<!-- Output -->
<template>
  <div class="rounded-lg bg-white p-6 shadow transition-shadow hover:shadow-lg">
    {{ content }}
  </div>
</template>
```

### Svelte

```svelte
<!-- Input -->
<div class="text-lg font-semibold {active ? 'bg-blue-500' : 'bg-gray-200'} p-4">
  {count}
</div>

<!-- Output -->
<div class="p-4 text-lg font-semibold {active ? 'bg-blue-500' : 'bg-gray-200'}">
  {count}
</div>
```

## Advanced Features

### Arbitrary Values

Supports TailwindCSS arbitrary values:

```html
<div class="mt-[117px] bg-[#bada55] text-[22px]">
  Custom values
</div>
```

### Variants and Modifiers

Handles all TailwindCSS variants:

```html
<!-- Responsive -->
<div class="text-sm md:text-base lg:text-lg">

<!-- State -->
<div class="hover:bg-blue-500 focus:ring-2 active:scale-95">

<!-- Dark mode -->
<div class="bg-white dark:bg-gray-900">

<!-- Stacked variants -->
<div class="md:hover:bg-blue-500 lg:focus:text-white">
```

### Modern TailwindCSS v4 Features

```html
<!-- Container queries -->
<div class="@container/main @lg/main:grid">

<!-- Data attributes -->
<div class="data-[state=open]:block data-[state=closed]:hidden">

<!-- ARIA attributes -->
<div class="aria-[expanded=true]:font-bold aria-disabled:opacity-50">
```

### Important Modifier

```html
<div class="!text-red-500 !font-bold">
  Always red and bold
</div>
```

### Negative Values

```html
<div class="-mt-4 -ml-2">
  Negative margins
</div>
```

## Integration with Other Plugins

This plugin works seamlessly with other dprint plugins:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/typescript-0.91.0.wasm",
    "https://plugins.dprint.dev/json-0.19.0.wasm",
    "https://plugins.dprint.dev/markdown-0.16.0.wasm",
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-0.1.0.wasm"
  ]
}
```

## Usage

### Command Line

Format your entire project:

```bash
dprint fmt
```

Format specific files:

```bash
dprint fmt src/**/*.{html,jsx,vue}
```

Check formatting without changes:

```bash
dprint check
```

### Editor Integration

The plugin works automatically with dprint editor extensions:

- **VS Code**: [Dprint Formatter](https://marketplace.visualstudio.com/items?itemName=dprint.dprint)
- **Vim/Neovim**: [dprint.nvim](https://github.com/kbario/dprint.nvim)
- **Other editors**: See [dprint editor setup](https://dprint.dev/setup/)

## Comparison with prettier-plugin-tailwindcss

This plugin aims to provide the same class sorting behavior as `prettier-plugin-tailwindcss`, but for dprint users:

| Feature | dprint-plugin-tailwindcss | prettier-plugin-tailwindcss |
|---------|---------------------------|----------------------------|
| Class sorting | ✅ Official TailwindCSS order | ✅ Official TailwindCSS order |
| Speed | ✅ WebAssembly (faster) | ⚠️ JavaScript |
| File formats | ✅ 6 formats | ✅ Many formats |
| TailwindCSS v4 | ✅ Full support | ✅ Full support |
| TailwindCSS v3 | ❌ Not supported | ✅ Supported |
| Configuration | ✅ Simple JSON | ✅ JavaScript |

**Note**: This plugin focuses on TailwindCSS v4 and does not maintain compatibility with v3 or earlier.

## Troubleshooting

### Classes Not Sorting

1. Check that the file extension is supported
2. Verify the plugin is enabled in `dprint.json`
3. Ensure class attributes match your `tailwindAttributes` configuration
4. For utility functions, verify they're listed in `tailwindFunctions`

### Conflicts with Other Plugins

If you experience issues with other dprint plugins:

1. Ensure this plugin is listed last in the `plugins` array
2. Check that other plugins don't also format the same file types
3. Review the [plugin compatibility guide](docs/PLUGIN_COMPATIBILITY.md)

### Performance Issues

For large projects:

1. Use `.dprintignore` to exclude `node_modules` and build directories
2. Consider formatting only changed files in CI/CD
3. See [performance tips](docs/PERFORMANCE.md)

## Contributing

Contributions are welcome! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

## Development

For plugin development, architecture details, and implementation guides, see the [Developer Documentation](docs/README.md).

## License

MIT © [friedjoff](https://github.com/friedjoff)

## Acknowledgments

- Inspired by [prettier-plugin-tailwindcss](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)
- Built with [dprint](https://dprint.dev/)
- Powered by [TailwindCSS](https://tailwindcss.com/)

