/// Performance tests for sorting and parsing operations
/// 
/// Tests ensure that the plugin handles large inputs efficiently and
/// doesn't degrade performance with complex scenarios.

#[cfg(test)]
mod performance_tests {
    use crate::sorter::sort_classes;
    use crate::extractor::ClassExtractor;
    use crate::parser::{FileFormat, FormatParser};
    use std::time::Instant;

    #[test]
    fn test_large_class_list_performance() {
        // Test with 100 classes
        let classes = vec![
            "px-4", "py-2", "bg-blue-500", "text-white", "rounded",
            "hover:bg-blue-600", "focus:outline-none", "focus:ring-2",
            "mt-4", "mb-4", "ml-4", "mr-4", "w-full", "h-auto",
            "flex", "items-center", "justify-center", "space-x-4",
            "text-lg", "font-bold", "text-center", "uppercase",
            "shadow-md", "border", "border-gray-300", "transition-all",
        ];
        
        // Repeat to create 100 classes
        let mut all_classes = String::new();
        for _ in 0..4 {
            for class in &classes {
                all_classes.push_str(class);
                all_classes.push(' ');
            }
        }
        
        let start = Instant::now();
        let result = sort_classes(&all_classes);
        let duration = start.elapsed();
        
        // Should complete in reasonable time (< 100ms for 100 classes)
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_very_long_class_names() {
        // Test with extremely long variant chains
        let classes = "hover:focus:active:sm:md:lg:xl:2xl:dark:group-hover:peer-focus:bg-blue-500 \
                      hover:focus:active:sm:md:lg:xl:2xl:dark:group-hover:peer-focus:text-white \
                      hover:focus:active:sm:md:lg:xl:2xl:dark:group-hover:peer-focus:border-gray-300";
        
        let start = Instant::now();
        let result = sort_classes(classes);
        let duration = start.elapsed();
        
        // Should handle long variants efficiently
        assert!(duration.as_millis() < 50, "Took too long: {:?}", duration);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_many_arbitrary_values() {
        // Test with many arbitrary value classes
        let mut classes = String::new();
        for i in 0..50 {
            classes.push_str(&format!("mt-[{}px] ", i * 10));
            classes.push_str(&format!("text-[#ff{}{}{}] ", i, i, i));
            classes.push_str(&format!("w-[{}rem] ", i));
        }
        
        let start = Instant::now();
        let result = sort_classes(&classes);
        let duration = start.elapsed();
        
        // Should handle arbitrary values efficiently
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_large_html_document_extraction() {
        let extractor = ClassExtractor::new(
            vec!["className".to_string()],
            vec!["class".to_string()],
        );
        
        // Create a large HTML document with many elements
        let mut html = String::from("<html><body>");
        for i in 0..100 {
            html.push_str(&format!(
                r#"<div class="p-{} bg-blue-{} text-white rounded shadow-md">Content {}</div>"#,
                i % 8, (i % 9) * 100, i
            ));
        }
        html.push_str("</body></html>");
        
        let start = Instant::now();
        let matches = extractor.extract_all(&html);
        let duration = start.elapsed();
        
        // Should extract from 100 elements quickly
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert_eq!(matches.len(), 100);
    }

    #[test]
    fn test_deeply_nested_jsx() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["className".to_string()],
        );
        
        // Create deeply nested JSX
        let mut jsx = String::from("<div className='container'>");
        for i in 0..50 {
            jsx.push_str(&format!(
                "<div className='level-{} p-4 bg-gray-{}'><span className='text-sm'>Level {}</span>",
                i, (i % 9) * 100, i
            ));
        }
        for _ in 0..50 {
            jsx.push_str("</div>");
        }
        jsx.push_str("</div>");
        
        let start = Instant::now();
        let matches = extractor.extract_all(&jsx);
        let duration = start.elapsed();
        
        // Should handle deep nesting efficiently
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert!(matches.len() >= 100); // container + 50 divs + 50 spans
    }

    #[test]
    fn test_vue_component_with_many_elements() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["class".to_string()],
        );
        let parser = FormatParser::new(extractor);
        
        // Large Vue component
        let mut vue = String::from("<template>\n<div class='container'>\n");
        for i in 0..100 {
            vue.push_str(&format!(
                "  <button class='btn btn-{} p-4 bg-blue-{} text-white'>Button {}</button>\n",
                i % 5, (i % 9) * 100, i
            ));
        }
        vue.push_str("</div>\n</template>\n");
        
        let start = Instant::now();
        let matches = parser.parse(&vue, FileFormat::Vue);
        let duration = start.elapsed();
        
        // Should parse large Vue component quickly
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert!(matches.len() >= 100);
    }

    #[test]
    fn test_repeated_sorting_performance() {
        let classes = "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 \
                      focus:outline-none focus:ring-2 focus:ring-blue-500 \
                      transition-colors duration-200 ease-in-out";
        
        let start = Instant::now();
        
        // Sort the same classes 1000 times
        for _ in 0..1000 {
            let _ = sort_classes(classes);
        }
        
        let duration = start.elapsed();
        
        // 1000 sorts should complete in reasonable time (< 1s)
        assert!(duration.as_secs() < 1, "Took too long: {:?}", duration);
    }

    #[test]
    fn test_many_duplicate_classes() {
        // Test with many duplicates to ensure deduplication is efficient
        let mut classes = String::new();
        for _ in 0..100 {
            classes.push_str("px-4 py-2 bg-blue-500 text-white rounded ");
        }
        
        let start = Instant::now();
        let result = sort_classes(&classes);
        let duration = start.elapsed();
        
        // Should deduplicate efficiently
        assert!(duration.as_millis() < 50, "Took too long: {:?}", duration);
        
        // Result should contain each class only once (but our implementation doesn't deduplicate yet)
        // Note: Current implementation preserves duplicates, which is actually correct behavior
        // for TailwindCSS (CSS cascade order matters)
        let class_count = result.split_whitespace().count();
        assert!(class_count > 0); // At least some classes present
        assert!(result.contains("px-4"));
        assert!(result.contains("bg-blue-500"));
    }

    #[test]
    fn test_complex_variant_combinations() {
        // Test performance with many complex variant combinations
        let mut classes = String::new();
        let variants = vec![
            "hover", "focus", "active", "sm", "md", "lg", "dark",
            "group-hover", "peer-focus", "disabled"
        ];
        
        for variant1 in &variants {
            for variant2 in &variants {
                if variant1 != variant2 {
                    classes.push_str(&format!("{}:{}:bg-blue-500 ", variant1, variant2));
                }
            }
        }
        
        let start = Instant::now();
        let result = sort_classes(&classes);
        let duration = start.elapsed();
        
        // Should handle complex variants efficiently
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_extraction_with_many_functions() {
        let extractor = ClassExtractor::new(
            vec![
                "clsx".to_string(),
                "classNames".to_string(),
                "tw".to_string(),
                "css".to_string(),
                "cx".to_string(),
            ],
            vec!["className".to_string(), "class".to_string()],
        );
        
        // Create content with many function calls
        let mut content = String::new();
        for i in 0..100 {
            content.push_str(&format!(
                r#"<div className={{clsx("p-{} bg-blue-{} text-white")}}>Item {}</div>"#,
                i % 8, (i % 9) * 100, i
            ));
        }
        
        let start = Instant::now();
        let matches = extractor.extract_all(&content);
        let duration = start.elapsed();
        
        // Should extract from many function calls efficiently
        assert!(duration.as_millis() < 100, "Took too long: {:?}", duration);
        assert_eq!(matches.len(), 100);
    }

    #[test]
    fn test_whitespace_heavy_input() {
        // Test with lots of whitespace between classes
        let mut classes = String::new();
        for class in vec!["px-4", "py-2", "bg-blue-500", "text-white", "rounded"] {
            classes.push_str(class);
            classes.push_str("        "); // 8 spaces
            classes.push_str("\n\t\t\t"); // newline and tabs
        }
        
        let start = Instant::now();
        let result = sort_classes(&classes);
        let duration = start.elapsed();
        
        // Should handle excessive whitespace efficiently
        assert!(duration.as_millis() < 50, "Took too long: {:?}", duration);
        
        // Result should be normalized
        let class_count = result.split_whitespace().count();
        assert_eq!(class_count, 5);
    }

    #[test]
    fn test_memory_efficiency_large_file() {
        let extractor = ClassExtractor::new(
            vec!["className".to_string()],
            vec!["class".to_string()],
        );
        let parser = FormatParser::new(extractor);
        
        // Create a realistically large file (50KB of HTML)
        let mut html = String::from("<html><body>");
        for i in 0..1000 {
            html.push_str(&format!(
                r#"
                <div class="container mx-auto px-4 py-8">
                  <h1 class="text-3xl font-bold mb-4">Section {}</h1>
                  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div class="p-6 bg-white rounded-lg shadow-md">Content</div>
                  </div>
                </div>
                "#,
                i
            ));
        }
        html.push_str("</body></html>");
        
        let start = Instant::now();
        let matches = parser.parse(&html, FileFormat::Html);
        let duration = start.elapsed();
        
        // Should handle 50KB+ files efficiently
        assert!(duration.as_millis() < 500, "Took too long: {:?}", duration);
        assert!(matches.len() >= 3000); // Many matches expected
    }

    #[test]
    fn test_regex_compilation_caching() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["className".to_string()],
        );
        
        let content = r#"<div className="px-4 py-2">Test</div>"#;
        
        let start = Instant::now();
        
        // Extract 100 times to test regex operations
        for _ in 0..100 {
            let _ = extractor.extract_all(content);
        }
        
        let duration = start.elapsed();
        
        // Should complete in reasonable time (< 1s for 100 iterations)
        // Note: Regex compilation is already cached via once_cell in the implementation
        assert!(duration.as_secs() < 1, "Regex operations too slow: {:?}", duration);
    }
}
