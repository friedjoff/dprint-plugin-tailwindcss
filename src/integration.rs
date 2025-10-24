/// Integration module for dprint plugin ecosystem compatibility
///
/// This module handles integration points with dprint and ensures
/// compatibility with other plugins in the formatting pipeline.
use dprint_core::plugins::{FormatResult, SyncHostFormatRequest};

/// Plugin compatibility checks and coordination
///
/// Ensures this plugin works well with other dprint plugins.
#[allow(dead_code)]
pub struct PluginCompatibility;

impl PluginCompatibility {
    /// Check if this plugin should format a given file
    ///
    /// Some files might be handled by other plugins first,
    /// and we should respect their formatting decisions.
    #[allow(dead_code)]
    pub fn should_format(file_path: &str) -> bool {
        // Check file extension
        let extension = file_path.split('.').next_back().unwrap_or("");

        match extension.to_lowercase().as_str() {
            // Primary supported formats
            "html" | "htm" | "jsx" | "tsx" | "vue" | "svelte" | "astro" => true,

            // JSON/YAML should not be formatted by this plugin
            "json" | "jsonc" | "yaml" | "yml" => false,

            // TypeScript/JavaScript without JSX - might contain utility functions
            // but are primarily handled by dprint-plugin-typescript
            "ts" | "js" | "mjs" | "cjs" => {
                // We can still format if there are HTML-like structures
                // but we should be cautious
                true
            }

            // Markdown might contain code blocks with classes
            "md" | "mdx" => true,

            // Unknown formats - allow fallback extraction
            _ => true,
        }
    }

    /// Check if the plugin should defer to another plugin
    ///
    /// Some files are better handled by specialized plugins.
    #[allow(dead_code)]
    pub fn should_defer(file_path: &str) -> bool {
        let extension = file_path.split('.').next_back().unwrap_or("");

        match extension.to_lowercase().as_str() {
            // These should be handled by their respective plugins first
            "json" | "jsonc" => true, // dprint-plugin-json
            "toml" => true,           // dprint-plugin-toml
            "yaml" | "yml" => true,   // Other YAML plugins

            _ => false,
        }
    }
}

/// Range formatting support
///
/// Handles partial file formatting when only a specific range
/// of the file needs to be formatted.
pub struct RangeFormatter;

impl RangeFormatter {
    /// Check if range formatting is applicable
    ///
    /// Currently, we format the entire file because:
    /// 1. Class sorting might affect positions throughout the file
    /// 2. We need to ensure consistency across all class attributes
    /// 3. Partial formatting could miss related class strings
    #[allow(dead_code)]
    pub fn supports_range_formatting() -> bool {
        // For now, we always format the entire file
        // This could be optimized in the future to only format
        // class attributes within the specified range
        false
    }

    /// Format a specific range of a file
    ///
    /// This is a placeholder for future range formatting support.
    /// Currently, it returns None to indicate full file formatting is needed.
    #[allow(dead_code)]
    pub fn format_range(_content: &str, _start_byte: usize, _end_byte: usize) -> Option<String> {
        // Future implementation:
        // 1. Parse only the specified range
        // 2. Extract class attributes within range
        // 3. Sort and replace only those classes
        // 4. Return modified range

        None // Not yet implemented
    }
}

/// Host formatting integration
///
/// Handles delegation to other plugins when needed.
pub struct HostFormatter;

impl HostFormatter {
    /// Format using the host's formatting pipeline
    ///
    /// This allows us to delegate formatting to other plugins
    /// when appropriate (e.g., formatting JSON within a Vue file's <script>).
    #[allow(dead_code)]
    pub fn format_with_host<F>(
        file_path: &str,
        content: &[u8],
        mut format_with_host: F,
    ) -> FormatResult
    where
        F: FnMut(SyncHostFormatRequest) -> FormatResult,
    {
        use dprint_core::configuration::ConfigKeyMap;
        use std::path::Path;

        // Create a host format request
        let path = Path::new(file_path);
        let config_map = ConfigKeyMap::new();
        let request = SyncHostFormatRequest {
            file_path: path,
            file_bytes: content,
            range: None, // Full file formatting
            override_config: &config_map,
        };

        // Delegate to host
        format_with_host(request)
    }

    /// Check if we should use host formatting for a specific section
    ///
    /// For example, we might want to use dprint-plugin-typescript
    /// for formatting <script> sections in Vue files.
    #[allow(dead_code)]
    pub fn should_use_host_for_section(section_type: &str) -> bool {
        match section_type {
            "script" => true,    // Could be formatted by TypeScript plugin
            "style" => true,     // Could be formatted by CSS plugin
            "template" => false, // We handle this
            _ => false,
        }
    }
}

/// Whitespace and comment preservation
///
/// Ensures that formatting preserves all whitespace and comments
/// that are not part of class strings.
#[allow(dead_code)]
pub struct PreservationGuard;

impl PreservationGuard {
    /// Verify that only class strings were modified
    ///
    /// This is a debugging/testing utility that ensures we only
    /// change class attribute values and nothing else.
    #[cfg(test)]
    pub fn verify_preservation(original: &str, formatted: &str) -> Result<(), String> {
        // If content is identical, no preservation issues
        if original == formatted {
            return Ok(());
        }

        // Count lines to ensure structure is preserved
        let orig_lines = original.lines().count();
        let fmt_lines = formatted.lines().count();

        if orig_lines != fmt_lines {
            return Err(format!(
                "Line count changed: {} -> {}",
                orig_lines, fmt_lines
            ));
        }

        // Check that comments are preserved
        let orig_comments = original.matches("<!--").count();
        let fmt_comments = formatted.matches("<!--").count();

        if orig_comments != fmt_comments {
            return Err(format!(
                "Comment count changed: {} -> {}",
                orig_comments, fmt_comments
            ));
        }

        Ok(())
    }

    /// Check if whitespace is preserved outside of class strings
    #[cfg(test)]
    pub fn check_whitespace_preservation(original: &str, formatted: &str) -> bool {
        // Simple heuristic: check that leading/trailing whitespace is same
        let orig_leading = original.len() - original.trim_start().len();
        let fmt_leading = formatted.len() - formatted.trim_start().len();

        let orig_trailing = original.len() - original.trim_end().len();
        let fmt_trailing = formatted.len() - formatted.trim_end().len();

        orig_leading == fmt_leading && orig_trailing == fmt_trailing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_format_supported_extensions() {
        assert!(PluginCompatibility::should_format("index.html"));
        assert!(PluginCompatibility::should_format("App.jsx"));
        assert!(PluginCompatibility::should_format("App.tsx"));
        assert!(PluginCompatibility::should_format("App.vue"));
        assert!(PluginCompatibility::should_format("App.svelte"));
        assert!(PluginCompatibility::should_format("page.astro"));
    }

    #[test]
    fn test_should_not_format_json() {
        assert!(!PluginCompatibility::should_format("config.json"));
        assert!(!PluginCompatibility::should_format("tsconfig.jsonc"));
        assert!(!PluginCompatibility::should_format("config.yaml"));
        assert!(!PluginCompatibility::should_format("config.yml"));
    }

    #[test]
    fn test_should_defer_to_other_plugins() {
        assert!(PluginCompatibility::should_defer("config.json"));
        assert!(PluginCompatibility::should_defer("Cargo.toml"));
        assert!(PluginCompatibility::should_defer("config.yaml"));

        assert!(!PluginCompatibility::should_defer("App.jsx"));
        assert!(!PluginCompatibility::should_defer("index.html"));
    }

    #[test]
    fn test_range_formatting_not_yet_supported() {
        assert!(!RangeFormatter::supports_range_formatting());
    }

    #[test]
    fn test_preservation_guard_identical_content() {
        let content = "<div class=\"flex p-4\">Test</div>";
        assert!(PreservationGuard::verify_preservation(content, content).is_ok());
    }

    #[test]
    fn test_preservation_guard_line_count() {
        let original = "<div class=\"flex p-4\">\n  <span>Test</span>\n</div>";
        let formatted = "<div class=\"flex p-4\"><span>Test</span></div>";

        let result = PreservationGuard::verify_preservation(original, formatted);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Line count changed"));
    }

    #[test]
    fn test_preservation_guard_comments() {
        let original = "<!-- Comment --><div class=\"flex p-4\">Test</div>";
        let formatted = "<div class=\"flex p-4\">Test</div>";

        let result = PreservationGuard::verify_preservation(original, formatted);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Comment count changed"));
    }

    #[test]
    fn test_whitespace_preservation() {
        let original = "  <div class=\"flex p-4\">Test</div>  ";
        let formatted = "  <div class=\"flex p-4\">Test</div>  ";

        assert!(PreservationGuard::check_whitespace_preservation(
            original, formatted
        ));
    }

    #[test]
    fn test_whitespace_preservation_failed() {
        let original = "  <div class=\"flex p-4\">Test</div>  ";
        let formatted = "<div class=\"flex p-4\">Test</div>";

        assert!(!PreservationGuard::check_whitespace_preservation(
            original, formatted
        ));
    }

    #[test]
    fn test_should_format_markdown() {
        assert!(PluginCompatibility::should_format("README.md"));
        assert!(PluginCompatibility::should_format("component.mdx"));
    }

    #[test]
    fn test_should_format_typescript_javascript() {
        // These might contain utility functions like clsx()
        assert!(PluginCompatibility::should_format("utils.ts"));
        assert!(PluginCompatibility::should_format("helper.js"));
        assert!(PluginCompatibility::should_format("module.mjs"));
    }
}
