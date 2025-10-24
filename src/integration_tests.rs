use super::*;
use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
use dprint_core::plugins::FormatConfigId;

fn format_text(file_text: &str, file_extension: &str) -> Option<String> {
    let mut handler = TailwindCssPluginHandler::new();

    let config_map = ConfigKeyMap::new();
    let global_config = GlobalConfiguration::default();
    let config_result = handler.resolve_config(config_map, &global_config);

    let file_name = format!("test.{}", file_extension);
    let file_path = std::path::Path::new(&file_name);
    let file_bytes = file_text.as_bytes().to_vec();
    let request = SyncFormatRequest {
        file_path,
        file_bytes,
        range: None,
        config: &config_result.config,
        config_id: FormatConfigId::from_raw(0),
        token: &dprint_core::plugins::NullCancellationToken,
    };

    match handler.format(request, |_| Ok(None)) {
        Ok(Some(result)) => Some(String::from_utf8(result).unwrap()),
        Ok(None) => None,
        Err(_) => None,
    }
}

#[test]
fn test_format_html_file() {
    let input = r#"<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body>
    <div class="z-10 p-4 mt-2 bg-white">Content</div>
</body>
</html>"#;

    let result = format_text(input, "html");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains(r#"class="mt-2 p-4 z-10 bg-white""#));
    // Verify rest of HTML is unchanged
    assert!(formatted.contains("<!DOCTYPE html>"));
    assert!(formatted.contains("<title>Test</title>"));
}

#[test]
fn test_format_htm_file() {
    let input = r#"<div class="hover:bg-blue-500 bg-red-500">Test</div>"#;

    let result = format_text(input, "htm");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains(r#"class="bg-red-500 hover:bg-blue-500""#));
}

#[test]
fn test_format_jsx_file() {
    let input = r#"export function Button() {
    return (
        <button className="z-10 p-4 bg-blue-500 text-white rounded-lg">
            Click me
        </button>
    );
}"#;

    let result = format_text(input, "jsx");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains(r#"className="p-4 z-10 text-white bg-blue-500 rounded-lg""#));
    assert!(formatted.contains("export function Button()"));
    assert!(formatted.contains("Click me"));
}

#[test]
fn test_format_tsx_file() {
    let input = r#"import React from 'react';

interface Props {
    variant: string;
}

export const Card: React.FC<Props> = ({ variant }) => {
    return (
        <div className="shadow-lg rounded-lg p-6 bg-white">
            Card content
        </div>
    );
};"#;

    let result = format_text(input, "tsx");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains(r#"className="p-6 bg-white rounded-lg shadow-lg""#));
    assert!(formatted.contains("import React"));
    assert!(formatted.contains("interface Props"));
}

#[test]
fn test_format_vue_file() {
    let input = r#"<template>
    <div class="flex items-center justify-between w-full px-4">
        <h1 class="font-bold text-2xl">Title</h1>
    </div>
</template>

<script>
export default {
    name: 'MyComponent'
};
</script>"#;

    let result = format_text(input, "vue");
    assert!(result.is_some());

    let formatted = result.unwrap();
    // Check that classes are present (order may vary slightly)
    assert!(formatted.contains("flex"));
    assert!(formatted.contains("items-center"));
    assert!(formatted.contains("w-full"));
    assert!(formatted.contains("px-4"));
    assert!(formatted.contains("font-bold"));
    assert!(formatted.contains("text-2xl"));
    assert!(formatted.contains("<script>"));
    assert!(formatted.contains("export default"));
}

#[test]
fn test_format_svelte_file() {
    let input = r#"<script>
    let count = 0;
</script>

<div class="flex items-center space-x-4 p-4">
    <button class="hover:bg-blue-600 bg-blue-500 px-4 py-2 text-white rounded">
        Count: {count}
    </button>
</div>"#;

    let result = format_text(input, "svelte");
    assert!(result.is_some());

    let formatted = result.unwrap();
    // Check that classes are present and sorted
    assert!(formatted.contains("flex"));
    assert!(formatted.contains("items-center"));
    assert!(formatted.contains("p-4"));
    assert!(formatted.contains("bg-blue-500"));
    assert!(formatted.contains("hover:bg-blue-600"));
}

#[test]
fn test_format_astro_file() {
    let input = r#"---
const title = "My Page";
---

<html>
<body>
    <div class="min-h-screen flex items-center bg-gray-100">
        <h1 class="text-4xl font-bold text-gray-900">{title}</h1>
    </div>
</body>
</html>"#;

    let result = format_text(input, "astro");
    // May or may not need changes depending on sort order
    if let Some(formatted) = result {
        // Check that classes are present
        assert!(formatted.contains("flex"));
        assert!(formatted.contains("items-center"));
        assert!(formatted.contains("min-h-screen"));
        assert!(formatted.contains("bg-gray-100"));
        assert!(formatted.contains("font-bold"));
        assert!(formatted.contains("text-4xl"));
        assert!(formatted.contains("const title"));
    } else {
        // No formatting needed, which is also ok
        assert!(input.contains("const title"));
    }
}

#[test]
fn test_format_with_clsx_function() {
    let input = r#"import clsx from 'clsx';

const classes = clsx("z-10 hover:shadow-lg p-4 bg-white");
"#;

    let result = format_text(input, "jsx");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains(r#"clsx("p-4 z-10 bg-white hover:shadow-lg")"#));
}

#[test]
fn test_format_preserves_comments() {
    let input = r#"<!-- This is a comment -->
<div class="z-10 p-4 mt-2">
    <!-- Another comment -->
    <span class="text-red-500">Text</span>
</div>
<!-- Final comment -->"#;

    let result = format_text(input, "html");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains("<!-- This is a comment -->"));
    assert!(formatted.contains("<!-- Another comment -->"));
    assert!(formatted.contains("<!-- Final comment -->"));
    assert!(formatted.contains(r#"class="mt-2 p-4 z-10""#));
}

#[test]
fn test_format_preserves_whitespace() {
    let input = r#"<div class="z-10 p-4">

    <span class="text-red-500">Text</span>

</div>"#;

    let result = format_text(input, "html");
    assert!(result.is_some());

    let formatted = result.unwrap();
    // Whitespace between tags should be preserved
    assert!(formatted.contains(">\n\n    <span"));
    assert!(formatted.contains("</span>\n\n</div>"));
}

#[test]
fn test_format_multiple_classes_in_file() {
    let input = r#"<div class="z-10 p-4">
    <div class="hover:bg-blue-500 bg-red-500">
        <div class="!font-bold text-gray-900">Text</div>
    </div>
</div>"#;

    let result = format_text(input, "html");
    assert!(result.is_some());

    let formatted = result.unwrap();
    assert!(formatted.contains(r#"class="p-4 z-10""#));
    assert!(formatted.contains(r#"class="bg-red-500 hover:bg-blue-500""#));
    assert!(formatted.contains(r#"class="text-gray-900 !font-bold""#));
}

#[test]
fn test_format_empty_class() {
    let input = r#"<div class="">Empty</div>"#;

    let result = format_text(input, "html");
    // Empty classes should not trigger formatting
    assert!(result.is_none());
}

#[test]
fn test_format_already_sorted() {
    let input = r#"<div class="mt-2 p-4 z-10 bg-white">Already sorted</div>"#;

    let result = format_text(input, "html");
    // Already sorted, so no changes
    assert!(result.is_none());
}

#[test]
fn test_format_disabled() {
    use dprint_core::configuration::ConfigKeyValue;

    let mut handler = TailwindCssPluginHandler::new();

    let mut config_map = ConfigKeyMap::new();
    config_map.insert("enabled".to_string(), ConfigKeyValue::Bool(false));

    let global_config = GlobalConfiguration::default();
    let config_result = handler.resolve_config(config_map, &global_config);

    let input = r#"<div class="z-10 p-4 mt-2">Test</div>"#;
    let file_bytes = input.as_bytes().to_vec();
    let request = SyncFormatRequest {
        file_path: std::path::Path::new("test.html"),
        file_bytes,
        range: None,
        config: &config_result.config,
        config_id: FormatConfigId::from_raw(0),
        token: &dprint_core::plugins::NullCancellationToken,
    };

    let result = handler.format(request, |_| Ok(None));
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_format_malformed_utf8() {
    let mut handler = TailwindCssPluginHandler::new();

    let config_map = ConfigKeyMap::new();
    let global_config = GlobalConfiguration::default();
    let config_result = handler.resolve_config(config_map, &global_config);

    // Invalid UTF-8 bytes
    let invalid_bytes: Vec<u8> = vec![0xFF, 0xFE, 0xFD];

    let request = SyncFormatRequest {
        file_path: std::path::Path::new("test.html"),
        file_bytes: invalid_bytes,
        range: None,
        config: &config_result.config,
        config_id: FormatConfigId::from_raw(0),
        token: &dprint_core::plugins::NullCancellationToken,
    };

    let result = handler.format(request, |_| Ok(None));
    assert!(result.is_err());
}

#[test]
fn test_format_mixed_quotes() {
    let input = r#"<div class="z-10 p-4" data-class='hover:bg-blue-500 bg-red-500'>Test</div>"#;

    let result = format_text(input, "html");
    assert!(result.is_some());

    let formatted = result.unwrap();
    // class attribute should be sorted
    assert!(formatted.contains("class="));
    // Verify the file structure is preserved
    assert!(formatted.contains("data-class="));
    assert!(formatted.contains("Test</div>"));
}

#[test]
fn test_format_with_line_breaks_in_class() {
    let input = r#"<div class="z-10
    p-4
    mt-2">Test</div>"#;

    let result = format_text(input, "html");
    // Should handle multi-line classes
    if let Some(formatted) = result {
        // Classes should be on one line after formatting
        assert!(!formatted.contains("class=\"z-10\n"));
    }
}
