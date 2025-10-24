/// Advanced edge case tests
///
/// Tests for malformed input, extreme cases, and unusual scenarios
/// to ensure robust error handling and graceful degradation.

#[cfg(test)]
mod edge_case_tests {
    use crate::config::Configuration;
    use crate::extractor::ClassExtractor;
    use crate::parser::{FileFormat, FormatParser};
    use crate::sorter::{sort_classes, TailwindClass};

    #[test]
    fn test_empty_string() {
        let result = sort_classes("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_only_whitespace() {
        let result = sort_classes("   ");
        // Whitespace-only should be normalized to empty or trimmed
        assert!(result.trim().is_empty());
    }

    #[test]
    fn test_single_class() {
        let result = sort_classes("flex");
        assert_eq!(result, "flex");
    }

    #[test]
    fn test_duplicate_classes() {
        let result = sort_classes("flex p-4 flex p-4");
        // Should preserve all classes even if duplicated
        assert!(result.contains("flex"));
        assert!(result.contains("p-4"));
    }

    #[test]
    fn test_very_long_class_string() {
        let classes = (0..100)
            .map(|i| format!("class-{}", i))
            .collect::<Vec<_>>()
            .join(" ");

        let result = sort_classes(&classes);
        // Should handle long strings without panic
        assert!(!result.is_empty());
    }

    #[test]
    fn test_malformed_arbitrary_value() {
        // Unclosed bracket
        let result = sort_classes("w-[100px p-4");
        // Should still parse what it can
        assert!(result.contains("p-4"));
    }

    #[test]
    fn test_special_characters_in_classes() {
        let result = sort_classes("flex p-4 text-[#ff0000]");
        // Should handle hex colors in arbitrary values
        assert!(result.contains("text-[#ff0000]"));
    }

    #[test]
    fn test_unicode_in_content() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);

        let content = r#"<div class="flex p-4">😀 Unicode content 中文</div>"#;
        let matches = extractor.extract_from_attributes(content);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }

    #[test]
    fn test_nested_quotes() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);

        // Single quotes inside double quotes
        let content = r#"<div class="flex p-4" data-test='{"class": "ignored"}'>Test</div>"#;
        let matches = extractor.extract_from_attributes(content);

        // Should only find the actual class attribute
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }

    #[test]
    fn test_multiple_spaces_between_classes() {
        let result = sort_classes("flex    p-4     mt-2");
        // Should normalize spaces
        let space_count = result.matches("  ").count();
        // Should have single spaces between classes
        assert_eq!(space_count, 0);
    }

    #[test]
    fn test_classes_with_tabs() {
        let result = sort_classes("flex\tp-4\tmt-2");
        // Should handle tabs as separators
        assert!(result.contains("flex"));
        assert!(result.contains("p-4"));
        assert!(result.contains("mt-2"));
    }

    #[test]
    fn test_classes_with_newlines() {
        let result = sort_classes("flex\np-4\nmt-2");
        // Should handle newlines as separators
        assert!(result.contains("flex"));
        assert!(result.contains("p-4"));
        assert!(result.contains("mt-2"));
    }

    #[test]
    fn test_invalid_class_names() {
        // Classes that don't match TailwindCSS patterns
        let result = sort_classes("not-a-real-class custom-class-123");
        // Should still sort them (as custom classes)
        assert!(result.contains("not-a-real-class"));
        assert!(result.contains("custom-class-123"));
    }

    #[test]
    fn test_extremely_long_variant_chain() {
        let class_with_variants =
            "sm:md:lg:xl:2xl:dark:hover:focus:active:disabled:first:last:odd:even:flex";
        let result = sort_classes(class_with_variants);
        // Should handle long variant chains
        assert!(result.contains("flex"));
    }

    #[test]
    fn test_arbitrary_value_with_spaces() {
        let result = sort_classes("w-[calc(100%-2rem)] p-4");
        // Should preserve arbitrary values with spaces
        assert!(result.contains("w-[calc(100%-2rem)]"));
        assert!(result.contains("p-4"));
    }

    #[test]
    fn test_class_with_slash() {
        let result = sort_classes("w-1/2 w-1/3 w-2/3");
        // Should handle fractional widths
        assert!(result.contains("w-1/2"));
        assert!(result.contains("w-1/3"));
        assert!(result.contains("w-2/3"));
    }

    #[test]
    fn test_peer_and_group_variants() {
        let result = sort_classes("peer-hover:bg-blue-500 group-hover:text-white");
        // Should handle peer and group variants
        assert!(result.contains("peer-hover:"));
        assert!(result.contains("group-hover:"));
    }

    #[test]
    fn test_tailwind_class_parsing_edge_cases() {
        // Test various edge cases in TailwindClass parsing
        let test_cases = vec![
            "!important-class",
            "-negative-margin",
            "hover:!font-bold",
            "dark:!-mt-4",
            "[&>*]:flex",
            "aria-[disabled]:opacity-50",
        ];

        for class in test_cases {
            let parsed = TailwindClass::parse(class);
            assert_eq!(parsed.original, class);
        }
    }

    #[test]
    fn test_empty_file_format() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);
        let parser = FormatParser::new(extractor);

        let content = "";
        let matches = parser.parse(content, FileFormat::Html);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_file_with_only_comments() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);
        let parser = FormatParser::new(extractor);

        let content = "<!-- This is just a comment -->\n<!-- Another comment -->";
        let matches = parser.parse(content, FileFormat::Html);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_malformed_html() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);

        // Unclosed tags
        let content = r#"<div class="flex p-4"><span class="text-lg">"#;
        let matches = extractor.extract_from_attributes(content);

        // Should still extract classes from well-formed attributes
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_mixed_quote_styles() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);

        let content = r#"<div class="flex p-4"><span class='text-lg font-bold'>Test</span></div>"#;
        let matches = extractor.extract_from_attributes(content);

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_class_in_attribute_value() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);

        // "class" appears in data attribute value, should not be extracted
        let content = r#"<div class="flex" data-info="class should be ignored">Test</div>"#;
        let matches = extractor.extract_from_attributes(content);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex");
    }

    #[test]
    fn test_extremely_nested_structure() {
        let extractor = ClassExtractor::new(vec!["clsx".to_string()], vec!["class".to_string()]);

        let content = r#"
            <div class="a">
                <div class="b">
                    <div class="c">
                        <div class="d">
                            <div class="e">
                                <div class="f">
                                    Deep
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        "#;

        let matches = extractor.extract_from_attributes(content);
        assert_eq!(matches.len(), 6);
    }

    #[test]
    fn test_config_with_empty_arrays() {
        let config = Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec![],
            tailwind_attributes: vec![],
        };

        // Should not panic with empty configuration
        assert!(config.tailwind_functions.is_empty());
        assert!(config.tailwind_attributes.is_empty());
    }

    #[test]
    fn test_sort_with_all_variants() {
        let classes = "hover:bg-blue-500 focus:bg-green-500 active:bg-red-500 \
                      disabled:opacity-50 first:mt-0 last:mb-0 odd:bg-gray-100 \
                      even:bg-gray-200 visited:text-purple-500";

        let result = sort_classes(classes);

        // Should handle all common variants
        assert!(result.contains("hover:"));
        assert!(result.contains("focus:"));
        assert!(result.contains("active:"));
    }

    #[test]
    fn test_responsive_with_variants() {
        let classes = "sm:hover:bg-blue-500 md:focus:text-white lg:active:p-4";
        let result = sort_classes(classes);

        // Should handle responsive breakpoints with pseudo-class variants
        assert!(result.contains("sm:hover:"));
        assert!(result.contains("md:focus:"));
        assert!(result.contains("lg:active:"));
    }

    #[test]
    fn test_dark_mode_variants() {
        let classes = "dark:bg-gray-900 dark:text-white dark:hover:bg-gray-800";
        let result = sort_classes(classes);

        // Should handle dark mode variants
        assert!(result.contains("dark:"));
    }

    #[test]
    fn test_container_queries() {
        let classes = "@container/main:flex @container/sidebar:hidden";
        let result = sort_classes(classes);

        // Should handle container query variants (new in Tailwind v3.2+)
        assert!(result.contains("@container"));
    }

    #[test]
    fn test_arbitrary_variants() {
        let classes = "[&:nth-child(3)]:flex [&>*]:p-4";
        let result = sort_classes(classes);

        // Should handle arbitrary variants
        assert!(result.contains("[&:nth-child(3)]:"));
        assert!(result.contains("[&>*]:"));
    }
}
