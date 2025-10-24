use once_cell::sync::Lazy;
use regex::Regex;

/// Patterns for detecting TailwindCSS classes in different contexts
pub struct ClassExtractor {
    /// Function names to look for
    pub function_names: Vec<String>,
    /// Attribute names to look for
    pub attribute_names: Vec<String>,
}

impl ClassExtractor {
    pub fn new(function_names: Vec<String>, attribute_names: Vec<String>) -> Self {
        Self {
            function_names,
            attribute_names,
        }
    }

    /// Extract all class strings from HTML/JSX attributes
    pub fn extract_from_attributes(&self, content: &str) -> Vec<ClassMatch> {
        let mut matches = Vec::new();

        for attr_name in &self.attribute_names {
            // Match class="..." or className="..." or class='...'
            let pattern = format!(r#"{}=["']([^"']*)["']"#, regex::escape(attr_name));
            if let Ok(re) = Regex::new(&pattern) {
                for cap in re.captures_iter(content) {
                    if let Some(classes) = cap.get(1) {
                        let class_content = classes.as_str();
                        if !class_content.trim().is_empty() {
                            matches.push(ClassMatch {
                                start: classes.start(),
                                end: classes.end(),
                                content: class_content.to_string(),
                            });
                        }
                    }
                }
            }

            // Match class={...} or className={...} (JSX)
            let jsx_pattern = format!(r#"{}\s*=\s*\{{([^}}]+)\}}"#, regex::escape(attr_name));
            if let Ok(re) = Regex::new(&jsx_pattern) {
                for cap in re.captures_iter(content) {
                    if let Some(expr) = cap.get(1) {
                        // Extract string literals from JSX expressions
                        let jsx_matches =
                            self.extract_from_jsx_expression(expr.as_str(), expr.start());
                        matches.extend(jsx_matches);
                    }
                }
            }
        }

        matches
    }

    /// Extract class strings from utility function calls
    pub fn extract_from_functions(&self, content: &str) -> Vec<ClassMatch> {
        let mut matches = Vec::new();

        for func_name in &self.function_names {
            // Match function calls: clsx("...", "...")
            let pattern = format!(r#"{}\s*\(([^)]+)\)"#, regex::escape(func_name));
            if let Ok(re) = Regex::new(&pattern) {
                for cap in re.captures_iter(content) {
                    if let Some(args) = cap.get(1) {
                        // Extract string literals from function arguments
                        let func_matches =
                            self.extract_strings_from_args(args.as_str(), args.start());
                        matches.extend(func_matches);
                    }
                }
            }
        }

        matches
    }

    /// Extract string literals from JSX expression
    fn extract_from_jsx_expression(&self, expr: &str, base_offset: usize) -> Vec<ClassMatch> {
        self.extract_strings_from_args(expr, base_offset)
    }

    /// Extract string literals from function arguments or JSX expressions
    fn extract_strings_from_args(&self, args: &str, base_offset: usize) -> Vec<ClassMatch> {
        static STRING_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"["'`]([^"'`]*)["'`]"#).unwrap());

        let mut matches = Vec::new();

        for cap in STRING_REGEX.captures_iter(args) {
            if let Some(string_content) = cap.get(1) {
                let content = string_content.as_str();
                // Skip if it looks like a variable or expression
                if !content.contains('$') && !content.is_empty() {
                    matches.push(ClassMatch {
                        start: base_offset + string_content.start(),
                        end: base_offset + string_content.end(),
                        content: content.to_string(),
                    });
                }
            }
        }

        matches
    }

    /// Extract all class strings from content
    #[allow(dead_code)]
    pub fn extract_all(&self, content: &str) -> Vec<ClassMatch> {
        let mut matches = Vec::new();
        matches.extend(self.extract_from_attributes(content));
        matches.extend(self.extract_from_functions(content));

        // Sort by position and remove duplicates
        matches.sort_by_key(|m| m.start);
        matches.dedup_by(|a, b| a.start == b.start && a.end == b.end);

        matches
    }
}

/// Represents a matched class string in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassMatch {
    /// Start position in the source
    pub start: usize,
    /// End position in the source
    pub end: usize,
    /// The matched class content
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_extractor() -> ClassExtractor {
        ClassExtractor::new(
            vec![
                "clsx".to_string(),
                "classnames".to_string(),
                "cn".to_string(),
            ],
            vec!["class".to_string(), "className".to_string()],
        )
    }

    #[test]
    fn test_extract_from_html_class_double_quotes() {
        let extractor = create_extractor();
        let html = r#"<div class="text-red-500 bg-blue-500">Test</div>"#;
        let matches = extractor.extract_from_attributes(html);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "text-red-500 bg-blue-500");
    }

    #[test]
    fn test_extract_from_html_class_single_quotes() {
        let extractor = create_extractor();
        let html = r#"<div class='text-red-500 bg-blue-500'>Test</div>"#;
        let matches = extractor.extract_from_attributes(html);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "text-red-500 bg-blue-500");
    }

    #[test]
    fn test_extract_from_jsx_classname() {
        let extractor = create_extractor();
        let jsx = r#"<div className="text-red-500 bg-blue-500">Test</div>"#;
        let matches = extractor.extract_from_attributes(jsx);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "text-red-500 bg-blue-500");
    }

    #[test]
    fn test_extract_from_clsx_function() {
        let extractor = create_extractor();
        let code = r#"const classes = clsx("text-red-500", "bg-blue-500");"#;
        let matches = extractor.extract_from_functions(code);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].content, "text-red-500");
        assert_eq!(matches[1].content, "bg-blue-500");
    }

    #[test]
    fn test_extract_from_classnames_function() {
        let extractor = create_extractor();
        let code = r#"const classes = classnames("text-red-500", "bg-blue-500");"#;
        let matches = extractor.extract_from_functions(code);

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_extract_from_multiple_elements() {
        let extractor = create_extractor();
        let html = r#"
            <div class="text-red-500">First</div>
            <div class="bg-blue-500">Second</div>
        "#;
        let matches = extractor.extract_from_attributes(html);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].content, "text-red-500");
        assert_eq!(matches[1].content, "bg-blue-500");
    }

    #[test]
    fn test_extract_mixed_content() {
        let extractor = create_extractor();
        let code = r#"
            <div class="text-red-500">
                <span className="bg-blue-500">Test</span>
            </div>
            const classes = clsx("p-4", "m-2");
        "#;
        let matches = extractor.extract_all(code);

        assert_eq!(matches.len(), 4);
    }

    #[test]
    fn test_extract_jsx_expression() {
        let extractor = create_extractor();
        let jsx = r#"<div className={"text-red-500 bg-blue-500"}>Test</div>"#;
        let matches = extractor.extract_from_attributes(jsx);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "text-red-500 bg-blue-500");
    }

    #[test]
    fn test_extract_empty_class() {
        let extractor = create_extractor();
        let html = r#"<div class="">Test</div>"#;
        let matches = extractor.extract_from_attributes(html);

        assert_eq!(matches.len(), 0); // Empty strings are filtered
    }

    #[test]
    fn test_extract_position_tracking() {
        let extractor = create_extractor();
        let html = r#"<div class="text-red-500">Test</div>"#;
        let matches = extractor.extract_from_attributes(html);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].start < matches[0].end);
        assert_eq!(&html[matches[0].start..matches[0].end], "text-red-500");
    }

    #[test]
    fn test_extract_real_world_react() {
        let extractor = create_extractor();
        let jsx = r#"
            export default function Button({ variant }) {
                return (
                    <button className="px-4 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600">
                        Click me
                    </button>
                );
            }
        "#;
        let matches = extractor.extract_all(jsx);

        assert_eq!(matches.len(), 1);
        assert!(matches[0].content.contains("px-4"));
        assert!(matches[0].content.contains("hover:bg-blue-600"));
    }

    #[test]
    fn test_extract_real_world_vue() {
        let extractor = create_extractor();
        let vue = r#"
            <template>
                <div class="flex items-center justify-center min-h-screen bg-gray-100">
                    <div class="p-6 bg-white rounded-lg shadow-lg">
                        <h1 class="text-2xl font-bold text-gray-900">Hello Vue</h1>
                    </div>
                </div>
            </template>
        "#;
        let matches = extractor.extract_all(vue);

        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_no_false_positives() {
        let extractor = create_extractor();
        let code = r#"
            // This should not match
            const notAClass = "text-red-500";
            const someUrl = "https://example.com/class?param=value";
        "#;
        let matches = extractor.extract_all(code);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_custom_function_names() {
        let extractor = ClassExtractor::new(
            vec!["cn".to_string(), "makeClass".to_string()],
            vec!["class".to_string()],
        );

        let code = r#"const classes = cn("text-red-500"); const other = makeClass("bg-blue-500");"#;
        let matches = extractor.extract_from_functions(code);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].content, "text-red-500");
        assert_eq!(matches[1].content, "bg-blue-500");
    }
}
