/// Parser module for different file formats
/// 
/// This module provides format-aware parsing to extract TailwindCSS classes
/// from various file types while preserving their original structure.

use crate::extractor::{ClassExtractor, ClassMatch};

/// File format types supported by the plugin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Html,
    Jsx,
    Tsx,
    Vue,
    Svelte,
    Astro,
}

impl FileFormat {
    /// Determine file format from file path
    pub fn from_path(path: &str) -> Option<Self> {
        let extension = path.split('.').last()?.to_lowercase();
        match extension.as_str() {
            "html" | "htm" => Some(FileFormat::Html),
            "jsx" => Some(FileFormat::Jsx),
            "tsx" => Some(FileFormat::Tsx),
            "vue" => Some(FileFormat::Vue),
            "svelte" => Some(FileFormat::Svelte),
            "astro" => Some(FileFormat::Astro),
            _ => None,
        }
    }
}

/// Format-aware parser that extracts classes based on file type
pub struct FormatParser {
    extractor: ClassExtractor,
}

impl FormatParser {
    pub fn new(extractor: ClassExtractor) -> Self {
        Self { extractor }
    }

    /// Parse content based on file format
    pub fn parse(&self, content: &str, format: FileFormat) -> Vec<ClassMatch> {
        match format {
            FileFormat::Html => self.parse_html(content),
            FileFormat::Jsx | FileFormat::Tsx => self.parse_jsx(content),
            FileFormat::Vue => self.parse_vue(content),
            FileFormat::Svelte => self.parse_svelte(content),
            FileFormat::Astro => self.parse_astro(content),
        }
    }

    /// Parse HTML files
    /// 
    /// HTML files contain standard class attributes in tags.
    /// We preserve all HTML structure, comments, and whitespace.
    fn parse_html(&self, content: &str) -> Vec<ClassMatch> {
        // Use extractor to find class attributes
        let mut matches = self.extractor.extract_from_attributes(content);
        
        // HTML doesn't typically have function calls like clsx()
        // but we check anyway in case of inline scripts
        let function_matches = self.extractor.extract_from_functions(content);
        matches.extend(function_matches);
        
        matches
    }

    /// Parse JSX/TSX files
    /// 
    /// JSX files use className instead of class and support:
    /// - String literals: className="..."
    /// - Template literals: className={`...`}
    /// - Utility functions: className={clsx(...)}
    fn parse_jsx(&self, content: &str) -> Vec<ClassMatch> {
        // Extract from className and class attributes
        let mut matches = self.extractor.extract_from_attributes(content);
        
        // Extract from utility functions (clsx, classnames, etc.)
        let function_matches = self.extractor.extract_from_functions(content);
        matches.extend(function_matches);
        
        matches
    }

    /// Parse Vue single-file components
    /// 
    /// Vue files have three sections:
    /// - <template>: Contains HTML-like markup with class attributes
    /// - <script>: Contains JavaScript/TypeScript logic
    /// - <style>: Contains CSS (we ignore this)
    /// 
    /// We only parse classes in the template section.
    fn parse_vue(&self, content: &str) -> Vec<ClassMatch> {
        // Find the template section
        if let Some(template_section) = extract_vue_template(content) {
            // Parse classes within the template section
            let mut matches = self.extractor.extract_from_attributes(&template_section.content);
            
            // Adjust match positions to account for template offset
            for m in &mut matches {
                m.start += template_section.start;
                m.end += template_section.start;
            }
            
            // Also check for function calls in template (rare but possible)
            let mut function_matches = self.extractor.extract_from_functions(&template_section.content);
            for m in &mut function_matches {
                m.start += template_section.start;
                m.end += template_section.start;
            }
            matches.extend(function_matches);
            
            matches
        } else {
            // No template section found, parse entire file
            // This handles edge cases where template syntax is non-standard
            let mut matches = self.extractor.extract_from_attributes(content);
            let function_matches = self.extractor.extract_from_functions(content);
            matches.extend(function_matches);
            matches
        }
    }

    /// Parse Svelte components
    /// 
    /// Svelte files are similar to Vue but with different syntax:
    /// - HTML-like markup at the top level
    /// - <script> sections for logic
    /// - <style> sections for CSS
    /// 
    /// Svelte also supports reactive expressions like {#if}, {#each}, etc.
    fn parse_svelte(&self, content: &str) -> Vec<ClassMatch> {
        // Svelte markup is at the top level, but we need to avoid
        // parsing inside <script> and <style> tags
        let sections = extract_svelte_markup_sections(content);
        
        let mut all_matches = Vec::new();
        
        for section in sections {
            // Extract classes from this markup section
            let mut matches = self.extractor.extract_from_attributes(&section.content);
            
            // Adjust positions
            for m in &mut matches {
                m.start += section.start;
                m.end += section.start;
            }
            
            all_matches.extend(matches);
        }
        
        all_matches
    }

    /// Parse Astro components
    /// 
    /// Astro files have:
    /// - Frontmatter section (---...---) with TypeScript/JavaScript
    /// - HTML-like markup (JSX-like syntax)
    /// 
    /// We parse classes in the markup section only.
    fn parse_astro(&self, content: &str) -> Vec<ClassMatch> {
        // Find the frontmatter section (---...---)
        let markup_start = if let Some(frontmatter_end) = find_astro_frontmatter_end(content) {
            frontmatter_end
        } else {
            0
        };
        
        // Parse the markup section
        let markup = &content[markup_start..];
        let mut matches = self.extractor.extract_from_attributes(markup);
        
        // Adjust positions to account for frontmatter
        for m in &mut matches {
            m.start += markup_start;
            m.end += markup_start;
        }
        
        // Also check for utility functions
        let mut function_matches = self.extractor.extract_from_functions(markup);
        for m in &mut function_matches {
            m.start += markup_start;
            m.end += markup_start;
        }
        matches.extend(function_matches);
        
        matches
    }
}

/// Section of content with its position
#[derive(Debug)]
struct ContentSection {
    start: usize,
    content: String,
}

/// Extract the template section from a Vue file
fn extract_vue_template(content: &str) -> Option<ContentSection> {
    // Find <template> opening tag
    let template_start_tag = content.find("<template")?;
    let template_content_start = content[template_start_tag..].find('>')? + template_start_tag + 1;
    
    // Find </template> closing tag
    let template_end = content.find("</template>")?;
    
    Some(ContentSection {
        start: template_content_start,
        content: content[template_content_start..template_end].to_string(),
    })
}

/// Extract markup sections from Svelte file (excluding <script> and <style>)
fn extract_svelte_markup_sections(content: &str) -> Vec<ContentSection> {
    let mut sections = Vec::new();
    let mut current_pos = 0;
    
    // Find all <script> and <style> tags
    let mut excluded_ranges = Vec::new();
    
    // Find <script> tags
    let mut search_pos = 0;
    while let Some(script_start) = content[search_pos..].find("<script") {
        let abs_start = search_pos + script_start;
        if let Some(script_end) = content[abs_start..].find("</script>") {
            let abs_end = abs_start + script_end + "</script>".len();
            excluded_ranges.push((abs_start, abs_end));
            search_pos = abs_end;
        } else {
            break;
        }
    }
    
    // Find <style> tags
    search_pos = 0;
    while let Some(style_start) = content[search_pos..].find("<style") {
        let abs_start = search_pos + style_start;
        if let Some(style_end) = content[abs_start..].find("</style>") {
            let abs_end = abs_start + style_end + "</style>".len();
            excluded_ranges.push((abs_start, abs_end));
            search_pos = abs_end;
        } else {
            break;
        }
    }
    
    // Sort excluded ranges by start position
    excluded_ranges.sort_by_key(|r| r.0);
    
    // Extract sections between excluded ranges
    for (start, end) in excluded_ranges {
        if current_pos < start {
            sections.push(ContentSection {
                start: current_pos,
                content: content[current_pos..start].to_string(),
            });
        }
        current_pos = end;
    }
    
    // Add remaining content after last excluded range
    if current_pos < content.len() {
        sections.push(ContentSection {
            start: current_pos,
            content: content[current_pos..].to_string(),
        });
    }
    
    // If no excluded ranges found, return entire content
    if sections.is_empty() {
        sections.push(ContentSection {
            start: 0,
            content: content.to_string(),
        });
    }
    
    sections
}

/// Find the end position of Astro frontmatter section
fn find_astro_frontmatter_end(content: &str) -> Option<usize> {
    // Check if file starts with ---
    if !content.trim_start().starts_with("---") {
        return None;
    }
    
    // Find the closing ---
    let start = content.find("---")? + 3;
    let remaining = &content[start..];
    let end = remaining.find("---")? + start + 3;
    
    // Find the next newline after closing ---
    if let Some(newline) = content[end..].find('\n') {
        Some(end + newline + 1)
    } else {
        Some(end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_parser() -> FormatParser {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string(), "classnames".to_string()],
            vec!["class".to_string(), "className".to_string()],
        );
        FormatParser::new(extractor)
    }

    #[test]
    fn test_file_format_from_path() {
        assert_eq!(FileFormat::from_path("index.html"), Some(FileFormat::Html));
        assert_eq!(FileFormat::from_path("index.htm"), Some(FileFormat::Html));
        assert_eq!(FileFormat::from_path("App.jsx"), Some(FileFormat::Jsx));
        assert_eq!(FileFormat::from_path("App.tsx"), Some(FileFormat::Tsx));
        assert_eq!(FileFormat::from_path("App.vue"), Some(FileFormat::Vue));
        assert_eq!(FileFormat::from_path("App.svelte"), Some(FileFormat::Svelte));
        assert_eq!(FileFormat::from_path("page.astro"), Some(FileFormat::Astro));
        assert_eq!(FileFormat::from_path("styles.css"), None);
    }

    #[test]
    fn test_parse_html() {
        let parser = create_test_parser();
        let content = r#"<div class="flex p-4">Content</div>"#;
        
        let matches = parser.parse_html(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }

    #[test]
    fn test_parse_jsx() {
        let parser = create_test_parser();
        let content = r#"<div className="flex p-4">Content</div>"#;
        
        let matches = parser.parse_jsx(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }

    #[test]
    fn test_parse_vue_with_template() {
        let parser = create_test_parser();
        let content = r#"
<template>
  <div class="flex p-4">Content</div>
</template>

<script>
export default {
  name: 'App'
}
</script>
"#;
        
        let matches = parser.parse_vue(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }

    #[test]
    fn test_parse_svelte() {
        let parser = create_test_parser();
        let content = r#"
<script>
  let count = 0;
</script>

<div class="flex p-4">
  <button class="bg-blue-500">Click</button>
</div>

<style>
  div { color: red; }
</style>
"#;
        
        let matches = parser.parse_svelte(content);
        assert_eq!(matches.len(), 2);
        assert!(matches.iter().any(|m| m.content == "flex p-4"));
        assert!(matches.iter().any(|m| m.content == "bg-blue-500"));
    }

    #[test]
    fn test_parse_astro() {
        let parser = create_test_parser();
        let content = r#"---
const title = "Hello";
---

<div class="flex p-4">{title}</div>
"#;
        
        let matches = parser.parse_astro(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }

    #[test]
    fn test_extract_vue_template() {
        let content = r#"
<template>
  <div>Hello</div>
</template>
"#;
        
        let section = extract_vue_template(content);
        assert!(section.is_some());
        let section = section.unwrap();
        assert!(section.content.contains("<div>Hello</div>"));
    }

    #[test]
    fn test_find_astro_frontmatter_end() {
        let content = "---\nconst x = 1;\n---\n<div>Hi</div>";
        let end = find_astro_frontmatter_end(content);
        assert!(end.is_some());
        assert!(content[end.unwrap()..].starts_with("<div>"));
    }

    #[test]
    fn test_astro_without_frontmatter() {
        let content = "<div class=\"flex\">No frontmatter</div>";
        let end = find_astro_frontmatter_end(content);
        assert_eq!(end, None);
        
        // Parser should still work
        let parser = create_test_parser();
        let matches = parser.parse_astro(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex");
    }

    #[test]
    fn test_svelte_markup_sections() {
        let content = r#"
<div class="a">Before</div>

<script>
  const x = 1;
</script>

<div class="b">Middle</div>

<style>
  .a { color: red; }
</style>

<div class="c">After</div>
"#;
        
        let sections = extract_svelte_markup_sections(content);
        assert_eq!(sections.len(), 3);
        
        // Check that sections contain the expected content
        assert!(sections[0].content.contains("class=\"a\""));
        assert!(sections[1].content.contains("class=\"b\""));
        assert!(sections[2].content.contains("class=\"c\""));
    }

    #[test]
    fn test_parse_preserves_positions() {
        let parser = create_test_parser();
        let content = r#"<div class="flex p-4"><span class="text-lg">Hi</span></div>"#;
        
        let matches = parser.parse_html(content);
        assert_eq!(matches.len(), 2);
        
        // Verify positions are correct
        for m in &matches {
            let extracted = &content[m.start..m.end];
            assert_eq!(extracted, m.content);
        }
    }

    #[test]
    fn test_parse_vue_without_template() {
        let parser = create_test_parser();
        let content = r#"<div class="flex p-4">No template tags</div>"#;
        
        // Should still parse as fallback
        let matches = parser.parse_vue(content);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }
}
