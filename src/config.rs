use dprint_core::configuration::{
    get_nullable_value, get_unknown_property_diagnostics, ConfigKeyMap, ConfigurationDiagnostic,
    GlobalConfiguration,
};
use dprint_core::plugins::{FileMatchingInfo, PluginResolveConfigurationResult};
use serde::{Deserialize, Serialize};

/// Configuration for the TailwindCSS plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    if let Some(tailwind_config) =
        get_nullable_value::<String>(&mut config, "tailwindConfig", &mut diagnostics)
    {
        resolved_config.tailwind_config = Some(tailwind_config);
    }

    // Parse tailwindFunctions
    if let Some(functions) = get_nullable_vec(&mut config, "tailwindFunctions", &mut diagnostics) {
        resolved_config.tailwind_functions = functions;
    }

    // Parse tailwindAttributes
    if let Some(attributes) = get_nullable_vec(&mut config, "tailwindAttributes", &mut diagnostics)
    {
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

#[allow(dead_code)]
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

    #[test]
    fn test_default_config() {
        let config = Configuration::default();
        assert!(config.enabled);
        assert_eq!(config.tailwind_functions.len(), 5);
        assert_eq!(config.tailwind_attributes.len(), 2);
    }

    #[test]
    fn test_resolve_config_with_custom_values() {
        use dprint_core::configuration::ConfigKeyValue;

        let mut config_map = ConfigKeyMap::new();
        config_map.insert("enabled".to_string(), ConfigKeyValue::Bool(false));
        config_map.insert(
            "tailwindConfig".to_string(),
            ConfigKeyValue::String("./tailwind.config.js".to_string()),
        );
        config_map.insert(
            "tailwindFunctions".to_string(),
            ConfigKeyValue::Array(vec![ConfigKeyValue::String("cn".to_string())]),
        );

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

    #[test]
    fn test_resolve_config_with_all_custom_values() {
        use dprint_core::configuration::ConfigKeyValue;

        let mut config_map = ConfigKeyMap::new();
        config_map.insert("enabled".to_string(), ConfigKeyValue::Bool(true));
        config_map.insert(
            "tailwindConfig".to_string(),
            ConfigKeyValue::String("./custom/tailwind.config.js".to_string()),
        );
        config_map.insert(
            "tailwindFunctions".to_string(),
            ConfigKeyValue::Array(vec![
                ConfigKeyValue::String("cn".to_string()),
                ConfigKeyValue::String("classNames".to_string()),
                ConfigKeyValue::String("tw".to_string()),
            ]),
        );
        config_map.insert(
            "tailwindAttributes".to_string(),
            ConfigKeyValue::Array(vec![
                ConfigKeyValue::String("class".to_string()),
                ConfigKeyValue::String("className".to_string()),
                ConfigKeyValue::String("classList".to_string()),
            ]),
        );

        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        assert!(result.config.enabled);
        assert_eq!(
            result.config.tailwind_config,
            Some("./custom/tailwind.config.js".to_string())
        );
        assert_eq!(
            result.config.tailwind_functions,
            vec!["cn", "classNames", "tw"]
        );
        assert_eq!(
            result.config.tailwind_attributes,
            vec!["class", "className", "classList"]
        );
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_resolve_config_with_invalid_type() {
        use dprint_core::configuration::ConfigKeyValue;

        let mut config_map = ConfigKeyMap::new();
        config_map.insert(
            "enabled".to_string(),
            ConfigKeyValue::String("true".to_string()),
        );
        config_map.insert(
            "tailwindFunctions".to_string(),
            ConfigKeyValue::String("not an array".to_string()),
        );

        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        // enabled should use default value when type is wrong
        assert!(result.config.enabled);
        // tailwindFunctions should use default value and produce diagnostic
        assert_eq!(result.config.tailwind_functions.len(), 5);
        assert!(!result.diagnostics.is_empty());
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.property_name == "tailwindFunctions"));
    }

    #[test]
    fn test_resolve_config_with_unknown_properties() {
        use dprint_core::configuration::ConfigKeyValue;

        let mut config_map = ConfigKeyMap::new();
        config_map.insert(
            "unknownProperty".to_string(),
            ConfigKeyValue::String("value".to_string()),
        );
        config_map.insert("anotherUnknown".to_string(), ConfigKeyValue::Number(123));

        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        // Should use default config
        assert!(result.config.enabled);
        // Should report unknown properties
        assert_eq!(result.diagnostics.len(), 2);
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.property_name == "unknownProperty"));
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.property_name == "anotherUnknown"));
    }

    #[test]
    fn test_resolve_config_with_invalid_array_elements() {
        use dprint_core::configuration::ConfigKeyValue;

        let mut config_map = ConfigKeyMap::new();
        config_map.insert(
            "tailwindFunctions".to_string(),
            ConfigKeyValue::Array(vec![
                ConfigKeyValue::String("valid".to_string()),
                ConfigKeyValue::Number(123), // Invalid type
                ConfigKeyValue::String("also_valid".to_string()),
            ]),
        );

        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        // Should use default value and produce diagnostic
        assert_eq!(result.config.tailwind_functions.len(), 5);
        assert!(!result.diagnostics.is_empty());
        assert!(result.diagnostics.iter().any(|d| {
            d.property_name == "tailwindFunctions" && d.message.contains("array of strings")
        }));
    }

    #[test]
    fn test_resolve_config_empty() {
        let config_map = ConfigKeyMap::new();
        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        // Should use all default values
        assert!(result.config.enabled);
        assert_eq!(result.config.tailwind_config, None);
        assert_eq!(
            result.config.tailwind_functions,
            vec![
                "classnames".to_string(),
                "clsx".to_string(),
                "ctl".to_string(),
                "cva".to_string(),
                "tw".to_string(),
            ]
        );
        assert_eq!(
            result.config.tailwind_attributes,
            vec!["class".to_string(), "className".to_string()]
        );
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_file_matching_extensions() {
        let config_map = ConfigKeyMap::new();
        let global_config = GlobalConfiguration::default();
        let result = resolve_config(config_map, &global_config);

        let extensions = &result.file_matching.file_extensions;
        assert!(extensions.contains(&"html".to_string()));
        assert!(extensions.contains(&"htm".to_string()));
        assert!(extensions.contains(&"jsx".to_string()));
        assert!(extensions.contains(&"tsx".to_string()));
        assert!(extensions.contains(&"vue".to_string()));
        assert!(extensions.contains(&"svelte".to_string()));
        assert!(extensions.contains(&"astro".to_string()));
    }
}
