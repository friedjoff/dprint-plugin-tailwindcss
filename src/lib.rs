mod config;
mod sorter;
mod extractor;
mod parser;
mod integration;

use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::{
    FormatResult, PluginInfo, PluginResolveConfigurationResult, 
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

use config::Configuration;
use extractor::ClassExtractor;
use integration::PluginCompatibility;
use parser::{FileFormat, FormatParser};
use sorter::sort_classes;

struct TailwindCssPluginHandler;

impl TailwindCssPluginHandler {
    const fn new() -> Self {
        TailwindCssPluginHandler
    }
}

impl SyncPluginHandler<Configuration> for TailwindCssPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "tailwindcss".to_string(),
            help_url: "https://github.com/friedjoff/dprint-plugin-tailwindcss".to_string(),
            config_schema_url: String::new(),
            update_url: Some("https://plugins.dprint.dev/friedjoff/dprint-plugin-tailwindcss/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        config::resolve_config(config, global_config)
    }

    fn check_config_updates(&self, _message: dprint_core::plugins::CheckConfigUpdatesMessage) -> anyhow::Result<Vec<dprint_core::plugins::ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        // Check if plugin is enabled
        if !request.config.enabled {
            return Ok(None);
        }

        // Check plugin compatibility - should we format this file?
        let file_path = request.file_path.to_string_lossy();
        if !PluginCompatibility::should_format(&file_path) {
            return Ok(None);
        }

        // If we should defer to another plugin, return None
        if PluginCompatibility::should_defer(&file_path) {
            return Ok(None);
        }

        // Convert file bytes to string
        let file_text = String::from_utf8(request.file_bytes.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to parse file as UTF-8: {}", e))?;

        // Determine file format from path
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

        // If no matches found, return unchanged
        if matches.is_empty() {
            return Ok(None);
        }

        // Sort and replace each class string
        let mut result = file_text.clone();
        let mut offset: i32 = 0;

        for class_match in matches {
            let sorted = sort_classes(&class_match.content);
            
            // Only replace if sorting changed the content
            if sorted != class_match.content {
                let start = (class_match.start as i32 + offset) as usize;
                let end = (class_match.end as i32 + offset) as usize;
                
                result.replace_range(start..end, &sorted);
                
                // Update offset for next replacements
                offset += sorted.len() as i32 - class_match.content.len() as i32;
            }
        }

        // Return the formatted text if changes were made
        if result != file_text {
            Ok(Some(result.into_bytes()))
        } else {
            Ok(None)
        }
    }
}

// Generate the WASM plugin code
#[cfg(target_arch = "wasm32")]
generate_plugin_code!(TailwindCssPluginHandler, TailwindCssPluginHandler::new());

#[cfg(test)]
mod tests {
    use super::*;
    use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};

    #[test]
    fn test_plugin_info() {
        let mut handler = TailwindCssPluginHandler::new();
        let info = handler.plugin_info();
        
        assert_eq!(info.name, "dprint-plugin-tailwindcss");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.config_key, "tailwindcss");
        assert!(info.help_url.contains("github.com"));
        assert!(info.update_url.is_some());
    }

    #[test]
    fn test_license_text() {
        let mut handler = TailwindCssPluginHandler::new();
        let license = handler.license_text();
        
        assert!(license.contains("MIT"));
        assert!(!license.is_empty());
    }

    #[test]
    fn test_resolve_config_default() {
        let mut handler = TailwindCssPluginHandler::new();
        let config_map = ConfigKeyMap::new();
        let global_config = GlobalConfiguration::default();
        
        let result = handler.resolve_config(config_map, &global_config);
        
        assert!(result.config.enabled);
        assert_eq!(result.config.tailwind_functions.len(), 5);
        assert_eq!(result.config.tailwind_attributes.len(), 2);
        assert!(result.diagnostics.is_empty());
        
        // Check file matching
        assert!(result.file_matching.file_extensions.contains(&"html".to_string()));
        assert!(result.file_matching.file_extensions.contains(&"jsx".to_string()));
        assert!(result.file_matching.file_extensions.contains(&"tsx".to_string()));
        assert!(result.file_matching.file_extensions.contains(&"vue".to_string()));
        assert!(result.file_matching.file_extensions.contains(&"svelte".to_string()));
    }

    #[test]
    fn test_resolve_config_custom() {
        use dprint_core::configuration::ConfigKeyValue;
        
        let mut handler = TailwindCssPluginHandler::new();
        let mut config_map = ConfigKeyMap::new();
        config_map.insert("enabled".to_string(), ConfigKeyValue::Bool(false));
        
        let global_config = GlobalConfiguration::default();
        let result = handler.resolve_config(config_map, &global_config);
        
        assert!(!result.config.enabled);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_check_config_updates() {
        use dprint_core::plugins::CheckConfigUpdatesMessage;
        
        let handler = TailwindCssPluginHandler::new();
        let message = CheckConfigUpdatesMessage {
            config: ConfigKeyMap::new(),
            old_version: Some("0.0.0".to_string()),
        };
        
        let result = handler.check_config_updates(message);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod format_aware_tests;

#[cfg(test)]
mod plugin_ecosystem_tests;

#[cfg(test)]
mod edge_case_tests;

#[cfg(test)]
mod custom_config_tests;

#[cfg(test)]
mod real_world_tests;

#[cfg(test)]
mod performance_tests;

#[cfg(test)]
mod prettier_compat_tests;
