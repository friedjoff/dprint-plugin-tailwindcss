# Configuration Options

This document describes all configuration options available for the dprint-plugin-tailwindcss.

## Configuration Schema

Add the plugin configuration under the `tailwindcss` key in your `dprint.json` or `.dprint.json` file:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss-v0.1.0.wasm"
  ],
  "tailwindcss": {
    "enabled": true,
    "tailwindConfig": "./tailwind.config.js",
    "tailwindFunctions": ["classnames", "clsx", "ctl", "cva", "tw"],
    "tailwindAttributes": ["class", "className"]
  }
}
```

## Options

### `enabled`

**Type:** `boolean`  
**Default:** `true`

Enable or disable the plugin. When set to `false`, the plugin will not format any files.

**Example:**
```json
{
  "tailwindcss": {
    "enabled": false
  }
}
```

### `tailwindConfig`

**Type:** `string | null`  
**Default:** `null`

Optional path to your `tailwind.config.js` file. This allows the plugin to understand your custom utilities and sort them correctly.

**Example:**
```json
{
  "tailwindcss": {
    "tailwindConfig": "./config/tailwind.config.js"
  }
}
```

**Note:** This feature is planned for future implementation. Currently, the plugin uses built-in TailwindCSS class ordering rules.

### `tailwindFunctions`

**Type:** `string[]`  
**Default:** `["classnames", "clsx", "ctl", "cva", "tw"]`

Array of function names that contain TailwindCSS class lists. The plugin will format class strings passed to these functions.

**Example:**
```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn", "classNames", "tw", "makeClass"]
  }
}
```

**Common function libraries:**
- `classnames` / `classNames` - [classnames package](https://www.npmjs.com/package/classnames)
- `clsx` - [clsx package](https://www.npmjs.com/package/clsx)
- `cva` - [class-variance-authority](https://www.npmjs.com/package/class-variance-authority)
- `tw` - [twind](https://twind.dev/) or custom utility
- `ctl` - [ctl](https://www.npmjs.com/package/@netlify/classnames-template-literals)
- `cn` - Common custom utility function name

### `tailwindAttributes`

**Type:** `string[]`  
**Default:** `["class", "className"]`

Array of HTML/JSX attribute names that contain TailwindCSS classes. The plugin will format class strings in these attributes.

**Example:**
```json
{
  "tailwindcss": {
    "tailwindAttributes": ["class", "className", "classList", "ngClass"]
  }
}
```

**Common attributes:**
- `class` - Standard HTML class attribute
- `className` - React/JSX class attribute
- `classList` - Solid.js class list
- `ngClass` - Angular class binding
- `:class` - Vue class binding

## File Support

The plugin automatically processes files with the following extensions:

- `.html` - HTML files
- `.htm` - HTML files
- `.jsx` - React JSX files
- `.tsx` - TypeScript React files
- `.vue` - Vue.js single-file components
- `.svelte` - Svelte components
- `.astro` - Astro components

## Validation

The plugin validates configuration at startup and will report errors for:

- **Type mismatches:** e.g., passing a string when an array is expected
- **Unknown properties:** Properties not defined in the schema
- **Invalid array elements:** Array elements that are not strings

When validation errors occur, the plugin will:
1. Use default values for invalid configuration options
2. Report diagnostic messages listing all configuration issues
3. Continue formatting with the valid portions of the configuration

## Examples

### Minimal Configuration (Use Defaults)

```json
{
  "tailwindcss": {}
}
```

### Disable Plugin

```json
{
  "tailwindcss": {
    "enabled": false
  }
}
```

### Custom Function Names Only

```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn", "styles"]
  }
}
```

### Full Custom Configuration

```json
{
  "tailwindcss": {
    "enabled": true,
    "tailwindConfig": "./tailwind.config.js",
    "tailwindFunctions": ["cn", "classNames", "tw", "styles"],
    "tailwindAttributes": ["class", "className", "classList"]
  }
}
```

### Framework-Specific Configurations

#### React/Next.js
```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn", "clsx", "cva"],
    "tailwindAttributes": ["className"]
  }
}
```

#### Vue.js
```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn"],
    "tailwindAttributes": ["class", ":class"]
  }
}
```

#### Svelte
```json
{
  "tailwindcss": {
    "tailwindFunctions": ["cn"],
    "tailwindAttributes": ["class", "class:"]
  }
}
```

#### Angular
```json
{
  "tailwindcss": {
    "tailwindAttributes": ["class", "ngClass", "[ngClass]"]
  }
}
```

## Testing Your Configuration

You can test your configuration by:

1. Adding the plugin to your `dprint.json`
2. Running `dprint fmt --check` to see if files would be formatted
3. Running `dprint fmt` to format files
4. Checking the output for any configuration warnings

## Next Steps

After configuring the plugin, you can:

- Run `dprint fmt` to format all supported files
- Set up editor integration for format-on-save
- Add dprint to your CI/CD pipeline
- Configure pre-commit hooks with [husky](https://typicode.github.io/husky/) or similar tools
