/// Custom configuration tests
/// 
/// Tests for various configuration scenarios including custom functions,
/// attributes, and configuration validation.

#[cfg(test)]
mod custom_config_tests {
    use crate::config::Configuration;
    use crate::extractor::ClassExtractor;
    use dprint_core::configuration::{ConfigKeyMap, ConfigKeyValue, GlobalConfiguration};
    use crate::TailwindCssPluginHandler;
    use dprint_core::plugins::SyncPluginHandler;

    #[test]
    fn test_custom_function_names() {
        let config = Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec![
                "myCustomFunction".to_string(),
                "tw".to_string(),
                "css".to_string(),
            ],
            tailwind_attributes: vec!["class".to_string()],
        };

        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );

        let content = r#"const classes = myCustomFunction("flex p-4");"#;
        let matches = extractor.extract_from_functions(content);

        assert!(matches.len() > 0);
    }

    #[test]
    fn test_custom_attribute_names() {
        let config = Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec![],
            tailwind_attributes: vec![
                "class".to_string(),
                "className".to_string(),
                "styleName".to_string(),
                "css".to_string(),
            ],
        };

        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );

        let content = r#"<div styleName="flex p-4" css="text-lg">Test</div>"#;
        let matches = extractor.extract_from_attributes(content);

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_config_with_single_function() {
        let config = Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec!["tw".to_string()],
            tailwind_attributes: vec!["class".to_string()],
        };

        assert_eq!(config.tailwind_functions.len(), 1);
        assert_eq!(config.tailwind_functions[0], "tw");
    }

    #[test]
    fn test_config_with_many_functions() {
        let functions: Vec<String> = (0..20)
            .map(|i| format!("func{}", i))
            .collect();

        let config = Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: functions.clone(),
            tailwind_attributes: vec!["class".to_string()],
        };

        assert_eq!(config.tailwind_functions.len(), 20);
    }

    #[test]
    fn test_disabled_config() {
        let config = Configuration {
            enabled: false,
            tailwind_config: None,
            tailwind_functions: vec!["clsx".to_string()],
            tailwind_attributes: vec!["class".to_string()],
        };

        assert!(!config.enabled);
    }

    #[test]
    fn test_config_with_tailwind_config_path() {
        let config = Configuration {
            enabled: true,
            tailwind_config: Some("./custom-tailwind.config.js".to_string()),
            tailwind_functions: vec!["clsx".to_string()],
            tailwind_attributes: vec!["class".to_string()],
        };

        assert!(config.tailwind_config.is_some());
        assert_eq!(
            config.tailwind_config.unwrap(),
            "./custom-tailwind.config.js"
        );
    }

    #[test]
    fn test_resolve_config_with_custom_functions_array() {
        let mut handler = TailwindCssPluginHandler::new();
        let mut config_map = ConfigKeyMap::new();
        
        // Create array of custom functions
        let functions = vec![
            ConfigKeyValue::String("myFunc1".to_string()),
            ConfigKeyValue::String("myFunc2".to_string()),
            ConfigKeyValue::String("myFunc3".to_string()),
        ];
        config_map.insert(
            "tailwindFunctions".to_string(),
            ConfigKeyValue::Array(functions),
        );

        let global_config = GlobalConfiguration::default();
        let result = handler.resolve_config(config_map, &global_config);

        assert_eq!(result.config.tailwind_functions.len(), 3);
        assert!(result.config.tailwind_functions.contains(&"myFunc1".to_string()));
        assert!(result.config.tailwind_functions.contains(&"myFunc2".to_string()));
        assert!(result.config.tailwind_functions.contains(&"myFunc3".to_string()));
    }

    #[test]
    fn test_resolve_config_with_custom_attributes_array() {
        let mut handler = TailwindCssPluginHandler::new();
        let mut config_map = ConfigKeyMap::new();
        
        let attributes = vec![
            ConfigKeyValue::String("class".to_string()),
            ConfigKeyValue::String("styleName".to_string()),
        ];
        config_map.insert(
            "tailwindAttributes".to_string(),
            ConfigKeyValue::Array(attributes),
        );

        let global_config = GlobalConfiguration::default();
        let result = handler.resolve_config(config_map, &global_config);

        assert_eq!(result.config.tailwind_attributes.len(), 2);
        assert!(result.config.tailwind_attributes.contains(&"class".to_string()));
        assert!(result.config.tailwind_attributes.contains(&"styleName".to_string()));
    }

    #[test]
    fn test_resolve_config_with_tailwind_config_string() {
        let mut handler = TailwindCssPluginHandler::new();
        let mut config_map = ConfigKeyMap::new();
        
        config_map.insert(
            "tailwindConfig".to_string(),
            ConfigKeyValue::String("./my-config.js".to_string()),
        );

        let global_config = GlobalConfiguration::default();
        let result = handler.resolve_config(config_map, &global_config);

        assert!(result.config.tailwind_config.is_some());
        assert_eq!(result.config.tailwind_config.unwrap(), "./my-config.js");
    }

    #[test]
    fn test_resolve_config_with_enabled_false() {
        let mut handler = TailwindCssPluginHandler::new();
        let mut config_map = ConfigKeyMap::new();
        
        config_map.insert(
            "enabled".to_string(),
            ConfigKeyValue::Bool(false),
        );

        let global_config = GlobalConfiguration::default();
        let result = handler.resolve_config(config_map, &global_config);

        assert!(!result.config.enabled);
    }

    #[test]
    fn test_resolve_config_with_all_custom_values() {
        let mut handler = TailwindCssPluginHandler::new();
        let mut config_map = ConfigKeyMap::new();
        
        config_map.insert("enabled".to_string(), ConfigKeyValue::Bool(true));
        config_map.insert(
            "tailwindConfig".to_string(),
            ConfigKeyValue::String("./tailwind.config.js".to_string()),
        );
        
        let functions = vec![
            ConfigKeyValue::String("tw".to_string()),
            ConfigKeyValue::String("css".to_string()),
        ];
        config_map.insert(
            "tailwindFunctions".to_string(),
            ConfigKeyValue::Array(functions),
        );
        
        let attributes = vec![
            ConfigKeyValue::String("class".to_string()),
            ConfigKeyValue::String("styleName".to_string()),
        ];
        config_map.insert(
            "tailwindAttributes".to_string(),
            ConfigKeyValue::Array(attributes),
        );

        let global_config = GlobalConfiguration::default();
        let result = handler.resolve_config(config_map, &global_config);

        assert!(result.config.enabled);
        assert_eq!(result.config.tailwind_config.unwrap(), "./tailwind.config.js");
        assert_eq!(result.config.tailwind_functions.len(), 2);
        assert_eq!(result.config.tailwind_attributes.len(), 2);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_extractor_with_no_functions_configured() {
        let extractor = ClassExtractor::new(
            vec![], // No functions
            vec!["class".to_string()],
        );

        let content = r#"const x = clsx("flex p-4");"#;
        let matches = extractor.extract_from_functions(content);

        // Should find nothing since clsx is not in the configured functions
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_extractor_with_no_attributes_configured() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec![], // No attributes
        );

        let content = r#"<div class="flex p-4">Test</div>"#;
        let matches = extractor.extract_from_attributes(content);

        // Should find nothing since class is not in the configured attributes
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_case_sensitive_function_names() {
        let extractor = ClassExtractor::new(
            vec!["MyFunc".to_string()],
            vec!["class".to_string()],
        );

        let content = r#"
            const a = MyFunc("flex p-4");
            const b = myFunc("text-lg");
            const c = MYFUNC("font-bold");
        "#;
        
        let matches = extractor.extract_from_functions(content);

        // Should only match exact case
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_case_sensitive_attribute_names() {
        let extractor = ClassExtractor::new(
            vec![],
            vec!["Class".to_string()], // Capital C
        );

        let content = r#"<div Class="flex" class="p-4">Test</div>"#;
        let matches = extractor.extract_from_attributes(content);

        // Should only match "Class" (capital C), not "class"
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex");
    }

    #[test]
    fn test_unicode_in_function_names() {
        // While unlikely, test that unicode function names work
        let extractor = ClassExtractor::new(
            vec!["函数".to_string()], // Chinese characters
            vec!["class".to_string()],
        );

        let content = r#"const x = 函数("flex p-4");"#;
        let matches = extractor.extract_from_functions(content);

        assert!(matches.len() > 0);
    }

    #[test]
    fn test_function_names_with_numbers() {
        let extractor = ClassExtractor::new(
            vec!["func123".to_string(), "tw2".to_string()],
            vec!["class".to_string()],
        );

        let content = r#"
            const a = func123("flex");
            const b = tw2("p-4");
        "#;
        
        let matches = extractor.extract_from_functions(content);
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_attribute_names_with_dashes() {
        let extractor = ClassExtractor::new(
            vec![],
            vec!["data-class".to_string(), "aria-label".to_string()],
        );

        let content = r#"<div data-class="flex" aria-label="button">Test</div>"#;
        let matches = extractor.extract_from_attributes(content);

        // Note: aria-label is not a class attribute, but if configured, should work
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_empty_function_call() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["class".to_string()],
        );

        let content = r#"const x = clsx();"#;
        let matches = extractor.extract_from_functions(content);

        // Empty function call should not cause errors
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_function_with_only_whitespace() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["class".to_string()],
        );

        let content = r#"const x = clsx("   ");"#;
        let matches = extractor.extract_from_functions(content);

        // Whitespace-only should be handled gracefully
        // Implementation may choose to include or exclude
        assert!(matches.is_empty() || matches[0].content.trim().is_empty());
    }
}
