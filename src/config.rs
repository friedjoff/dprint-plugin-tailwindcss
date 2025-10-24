use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_nullable_value, ConfigKeyMap,
    ConfigurationDiagnostic, GlobalConfiguration,
};
use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};
use serde::{Deserialize, Serialize};

/// Configuration for the TailwindCSS plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// Enable or disable the plugin
    pub enabled: bool,
    
    /// Path to tailwind.config.js (optional)
    pub tailwind_config: Option<String>,
    
    /// Custom function names containing class lists
    pub tailwind_functions: Vec<String>,
    
    /// HTML attributes to format
    pub tailwind_attributes: Vec<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec![
                "classnames".to_string(),
                "clsx".to_string(),
                "ctl".to_string(),
                "cva".to_string(),
                "tw".to_string(),
            ],
            tailwind_attributes: vec!["class".to_string(), "className".to_string()],
        }
    }
}

/// Resolve the configuration from the provided config map
pub fn resolve_config(
    mut config: ConfigKeyMap,
    _global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();
    let mut resolved_config = Configuration::default();

    // Parse enabled
    resolved_config.enabled = get_nullable_value(&mut config, "enabled", &mut diagnostics)
        .unwrap_or(resolved_config.enabled);

    // Parse tailwindConfig
    if let Some(tailwind_config) = get_nullable_value::<String>(&mut config, "tailwindConfig", &mut diagnostics) {
        resolved_config.tailwind_config = Some(tailwind_config);
    }

    // Parse tailwindFunctions
    if let Some(functions) = get_nullable_vec(&mut config, "tailwindFunctions", &mut diagnostics) {
        resolved_config.tailwind_functions = functions;
    }

    // Parse tailwindAttributes
    if let Some(attributes) = get_nullable_vec(&mut config, "tailwindAttributes", &mut diagnostics) {
        resolved_config.tailwind_attributes = attributes;
    }

    // Check for unknown properties
    diagnostics.extend(get_unknown_property_diagnostics(config));

    PluginResolveConfigurationResult {
        config: resolved_config,
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec![
                "html".to_string(),
                "htm".to_string(),
                "jsx".to_string(),
                "tsx".to_string(),
                "vue".to_string(),
                "svelte".to_string(),
                "astro".to_string(),
            ],
            file_names: vec![],
        },
    }
}

fn get_nullable_vec(
    config: &mut ConfigKeyMap,
    key: &str,
    diagnostics: &mut Vec<ConfigurationDiagnostic>,
) -> Option<Vec<String>> {
    use dprint_core::configuration::ConfigKeyValue;
    
    if let Some(value) = config.swap_remove(key) {
        match value {
            ConfigKeyValue::Array(arr) => {
                let result: Option<Vec<String>> = arr
                    .iter()
                    .map(|v| match v {
                        ConfigKeyValue::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect();
                if result.is_none() {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: key.to_string(),
                        message: format!("Expected array of strings for '{}'", key),
                    });
                }
                result
            }
            _ => {
                diagnostics.push(ConfigurationDiagnostic {
                    property_name: key.to_string(),
                    message: format!("Expected array for '{}'", key),
                });
                None
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default_config() {
        let config = Configuration::default();
        assert!(config.enabled);
        assert_eq!(config.tailwind_functions.len(), 5);
        assert_eq!(config.tailwind_attributes.len(), 2);
    }

    #[test]
    fn test_resolve_config_with_custom_values() {
        let mut config_map = ConfigKeyMap::new();
        config_map.insert("enabled".to_string(), json!(false));
        config_map.insert("tailwindConfig".to_string(), json!("./tailwind.config.js"));
        config_map.insert("tailwindFunctions".to_string(), json!(["cn"]));

        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        assert!(!result.config.enabled);
        assert_eq!(
            result.config.tailwind_config,
            Some("./tailwind.config.js".to_string())
        );
        assert_eq!(result.config.tailwind_functions, vec!["cn"]);
        assert!(result.diagnostics.is_empty());
    }
}
