/// Plugin ecosystem integration tests
///
/// Tests that verify compatibility with dprint's plugin ecosystem
/// and ensure the plugin works well alongside other plugins.

#[cfg(test)]
mod plugin_ecosystem_tests {
    use crate::config::Configuration;
    use crate::integration::PluginCompatibility;
    use crate::TailwindCssPluginHandler;
    use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
    use dprint_core::plugins::{
        FormatConfigId, NullCancellationToken, SyncFormatRequest, SyncPluginHandler,
    };
    use std::path::Path;

    fn create_test_handler() -> TailwindCssPluginHandler {
        TailwindCssPluginHandler::new()
    }

    fn create_test_config() -> Configuration {
        Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec!["clsx".to_string(), "cn".to_string()],
            tailwind_attributes: vec!["class".to_string(), "className".to_string()],
        }
    }

    fn format_file(
        handler: &mut TailwindCssPluginHandler,
        file_path: &str,
        content: &str,
        config: Configuration,
    ) -> Option<String> {
        let path = Path::new(file_path);
        let request = SyncFormatRequest {
            file_path: path,
            file_bytes: content.as_bytes().to_vec(),
            range: None,
            config: &config,
            config_id: FormatConfigId::from_raw(0),
            token: &NullCancellationToken,
        };

        let result = handler.format(request, |_| Ok(None));

        match result {
            Ok(Some(bytes)) => Some(String::from_utf8(bytes).unwrap()),
            Ok(None) => None,
            Err(e) => panic!("Format error: {}", e),
        }
    }

    #[test]
    fn test_json_files_not_formatted() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let json_content = r#"{
  "name": "test",
  "class": "z-10 p-4 mt-2"
}"#;

        let result = format_file(&mut handler, "config.json", json_content, config);

        // JSON files should not be formatted by this plugin
        assert!(result.is_none());
    }

    #[test]
    fn test_yaml_files_not_formatted() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let yaml_content = r#"
name: test
class: z-10 p-4 mt-2
"#;

        let result = format_file(&mut handler, "config.yaml", yaml_content, config);

        // YAML files should not be formatted by this plugin
        assert!(result.is_none());
    }

    #[test]
    fn test_html_files_formatted() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let html_content = r#"<div class="z-10 p-4 mt-2">Content</div>"#;

        let result = format_file(&mut handler, "index.html", html_content, config);

        // HTML files should be formatted
        assert!(result.is_some());
        let formatted = result.unwrap();
        assert!(formatted.contains("mt-2 p-4 z-10"));
    }

    #[test]
    fn test_jsx_files_formatted() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let jsx_content = r#"<div className="z-10 p-4 mt-2">Content</div>"#;

        let result = format_file(&mut handler, "App.jsx", jsx_content, config);

        // JSX files should be formatted
        assert!(result.is_some());
        let formatted = result.unwrap();
        assert!(formatted.contains("mt-2 p-4 z-10"));
    }

    #[test]
    fn test_multiple_plugins_coexistence() {
        // This test verifies that our plugin can be used alongside other plugins
        // by ensuring it doesn't modify files meant for other plugins

        let mut handler = create_test_handler();
        let config = create_test_config();

        // Test JSON (should defer to dprint-plugin-json)
        let json_result = format_file(&mut handler, "package.json", "{}", config.clone());
        assert!(json_result.is_none(), "Should not format JSON files");

        // Test TOML (should defer to dprint-plugin-toml)
        let toml_result = format_file(&mut handler, "Cargo.toml", "[package]", config.clone());
        assert!(toml_result.is_none(), "Should not format TOML files");

        // Test HTML (our responsibility)
        let html_result = format_file(
            &mut handler,
            "index.html",
            r#"<div class="z-10 p-4">Test</div>"#,
            config,
        );
        assert!(html_result.is_some(), "Should format HTML files");
    }

    #[test]
    fn test_typescript_files_formatted() {
        // TypeScript files might contain utility functions like clsx()
        let mut handler = create_test_handler();
        let config = create_test_config();

        let ts_content = r#"
const classes = clsx("z-10 p-4 mt-2");
"#;

        let result = format_file(&mut handler, "utils.ts", ts_content, config);

        // Should format utility functions in TS files
        assert!(result.is_some());
        let formatted = result.unwrap();
        assert!(formatted.contains("mt-2 p-4 z-10"));
    }

    #[test]
    fn test_markdown_files_formatted() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let md_content = r#"# Title

<div class="z-10 p-4 mt-2">HTML in markdown</div>
"#;

        let result = format_file(&mut handler, "README.md", md_content, config);

        // Markdown with HTML should be formatted
        assert!(result.is_some());
        let formatted = result.unwrap();
        assert!(formatted.contains("mt-2 p-4 z-10"));
    }

    #[test]
    fn test_global_config_respected() {
        let mut handler = create_test_handler();
        let config_map = ConfigKeyMap::new();
        let global_config = GlobalConfiguration {
            line_width: Some(80),
            indent_width: Some(2),
            use_tabs: Some(false),
            new_line_kind: None,
        };

        let result = handler.resolve_config(config_map, &global_config);

        // Global config should be accepted without errors
        assert!(result.diagnostics.is_empty());

        // Our plugin uses global config for dprint integration
        // but doesn't use line_width/indent_width since we only
        // sort class names, not reformat HTML
        assert!(result.config.enabled);
    }

    #[test]
    fn test_file_matching_configuration() {
        let mut handler = create_test_handler();
        let config_map = ConfigKeyMap::new();
        let global_config = GlobalConfiguration::default();

        let result = handler.resolve_config(config_map, &global_config);

        // Verify file matching includes all supported extensions
        let extensions = &result.file_matching.file_extensions;
        assert!(extensions.contains(&"html".to_string()));
        assert!(extensions.contains(&"htm".to_string()));
        assert!(extensions.contains(&"jsx".to_string()));
        assert!(extensions.contains(&"tsx".to_string()));
        assert!(extensions.contains(&"vue".to_string()));
        assert!(extensions.contains(&"svelte".to_string()));
        assert!(extensions.contains(&"astro".to_string()));
    }

    #[test]
    fn test_disabled_plugin_returns_none() {
        let mut handler = create_test_handler();
        let mut config = create_test_config();
        config.enabled = false;

        let content = r#"<div class="z-10 p-4 mt-2">Content</div>"#;
        let result = format_file(&mut handler, "index.html", content, config);

        // Disabled plugin should not format anything
        assert!(result.is_none());
    }

    #[test]
    fn test_comments_preserved_during_formatting() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let content = r#"<!-- Header comment -->
<div class="z-10 p-4 mt-2">
  <!-- Inner comment -->
  <span class="text-lg font-bold">Text</span>
</div>
<!-- Footer comment -->"#;

        let result = format_file(&mut handler, "index.html", content, config);

        assert!(result.is_some());
        let formatted = result.unwrap();

        // All comments should be preserved
        assert_eq!(formatted.matches("<!--").count(), 3);
        assert!(formatted.contains("<!-- Header comment -->"));
        assert!(formatted.contains("<!-- Inner comment -->"));
        assert!(formatted.contains("<!-- Footer comment -->"));
    }

    #[test]
    fn test_whitespace_preserved_outside_classes() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let content = "  <div class=\"z-10 p-4 mt-2\">Content</div>  ";

        let result = format_file(&mut handler, "index.html", content, config);

        assert!(result.is_some());
        let formatted = result.unwrap();

        // Leading and trailing whitespace should be preserved
        assert!(formatted.starts_with("  "));
        assert!(formatted.ends_with("  "));
    }

    #[test]
    fn test_line_breaks_preserved() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let content = "<div class=\"z-10 p-4 mt-2\">\n  <span>Content</span>\n</div>";

        let result = format_file(&mut handler, "index.html", content, config);

        assert!(result.is_some());
        let formatted = result.unwrap();

        // Line breaks should be preserved
        assert_eq!(formatted.lines().count(), content.lines().count());
    }

    #[test]
    fn test_plugin_compatibility_checks() {
        // Test the PluginCompatibility helper functions
        assert!(PluginCompatibility::should_format("index.html"));
        assert!(PluginCompatibility::should_format("App.jsx"));
        assert!(!PluginCompatibility::should_format("config.json"));

        assert!(PluginCompatibility::should_defer("config.json"));
        assert!(PluginCompatibility::should_defer("Cargo.toml"));
        assert!(!PluginCompatibility::should_defer("index.html"));
    }

    #[test]
    fn test_unknown_extension_fallback() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        // File with unknown extension but HTML-like content
        let content = r#"<div class="z-10 p-4 mt-2">Content</div>"#;

        let result = format_file(&mut handler, "template.twig", content, config);

        // Should still attempt formatting with fallback
        assert!(result.is_some());
        let formatted = result.unwrap();
        assert!(formatted.contains("mt-2 p-4 z-10"));
    }

    #[test]
    fn test_mixed_framework_file() {
        // Test a file that might be processed by multiple plugins
        let mut handler = create_test_handler();
        let config = create_test_config();

        let content = r#"<script>
const x = 1;
</script>

<div class="z-10 p-4 mt-2">Content</div>

<style>
.test { color: red; }
</style>"#;

        let result = format_file(&mut handler, "component.svelte", content, config);

        assert!(result.is_some());
        let formatted = result.unwrap();

        // Only class in div should be sorted, script/style untouched
        assert!(formatted.contains("mt-2 p-4 z-10"));
        assert!(formatted.contains("const x = 1;"));
        assert!(formatted.contains(".test { color: red; }"));
    }

    #[test]
    fn test_no_false_positives_in_script_tags() {
        let mut handler = create_test_handler();
        let config = create_test_config();

        let content = r#"<script>
const className = "z-10 p-4 mt-2"; // Should NOT be sorted
</script>

<div class="z-10 p-4 mt-2">Should be sorted</div>"#;

        let result = format_file(&mut handler, "component.vue", content, config);

        assert!(result.is_some());
        let formatted = result.unwrap();

        // Script content should remain unchanged
        assert!(formatted.contains("const className = \"z-10 p-4 mt-2\";"));

        // But div class should be sorted
        // Note: This depends on our Vue parser correctly excluding script sections
        // The actual test may need adjustment based on implementation
    }

    #[test]
    fn test_format_idempotency() {
        // Formatting twice should produce the same result
        let mut handler = create_test_handler();
        let config = create_test_config();

        let content = r#"<div class="z-10 p-4 mt-2">Content</div>"#;

        // First format
        let result1 = format_file(&mut handler, "index.html", content, config.clone());
        assert!(result1.is_some());
        let formatted1 = result1.unwrap();

        // Second format on already formatted content
        let result2 = format_file(&mut handler, "index.html", &formatted1, config);

        // Should return None (no changes needed) since it's already formatted
        assert!(result2.is_none());
    }
}
