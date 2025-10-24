/// Additional integration tests for format-aware parsing
///
/// These tests verify that the FormatParser correctly handles
/// different file formats and preserves their structure.

#[cfg(test)]
mod format_aware_tests {
    use crate::config::Configuration;
    use crate::extractor::ClassExtractor;
    use crate::parser::{FileFormat, FormatParser};

    fn create_test_config() -> Configuration {
        Configuration {
            enabled: true,
            tailwind_config: None,
            tailwind_functions: vec!["clsx".to_string(), "cn".to_string()],
            tailwind_attributes: vec!["class".to_string(), "className".to_string()],
        }
    }

    #[test]
    fn test_vue_only_parses_template_section() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
<template>
  <div class="z-10 p-4 mt-2">In template - should be sorted</div>
</template>

<script>
export default {
  data() {
    return {
      // This class="z-10 p-4 mt-2" in a comment should NOT be parsed
      className: "z-10 p-4 mt-2" // This is JavaScript, not HTML
    }
  }
}
</script>

<style>
.my-class {
  /* class="z-10 p-4 mt-2" - CSS comment should NOT be parsed */
}
</style>
"#;

        let matches = parser.parse(content, FileFormat::Vue);

        // Should only find the one in the template section
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "z-10 p-4 mt-2");

        // Verify the match is in the template section
        assert!(matches[0].start > content.find("<template>").unwrap());
        assert!(matches[0].end < content.find("</template>").unwrap());
    }

    #[test]
    fn test_svelte_excludes_script_and_style() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
<div class="z-10 p-4 mt-2">Before script</div>

<script>
  let className = "z-10 p-4 mt-2"; // Should NOT be parsed
</script>

<button class="text-white bg-blue-500">Middle section</button>

<style>
  .button {
    /* class="z-10 p-4 mt-2" should NOT be parsed */
  }
</style>

<span class="flex items-center">After style</span>
"#;

        let matches = parser.parse(content, FileFormat::Svelte);

        // Should find exactly 3 matches (excluding script and style sections)
        assert_eq!(matches.len(), 3);

        let class_strings: Vec<&str> = matches.iter().map(|m| m.content.as_str()).collect();
        assert!(class_strings.contains(&"z-10 p-4 mt-2"));
        assert!(class_strings.contains(&"text-white bg-blue-500"));
        assert!(class_strings.contains(&"flex items-center"));

        // Verify none of the matches are in script or style sections
        for m in &matches {
            let before_match = &content[..m.start];
            let after_match = &content[m.end..];

            // Count script tags before and after
            let scripts_before = before_match.matches("<script").count();
            let scripts_after = after_match.matches("</script>").count();
            let total_scripts = content.matches("<script").count();

            // Match should not be inside a script tag
            assert_eq!(scripts_before, total_scripts - scripts_after);

            // Count style tags before and after
            let styles_before = before_match.matches("<style").count();
            let styles_after = after_match.matches("</style>").count();
            let total_styles = content.matches("<style").count();

            // Match should not be inside a style tag
            assert_eq!(styles_before, total_styles - styles_after);
        }
    }

    #[test]
    fn test_astro_excludes_frontmatter() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"---
const title = "Hello";
const className = "z-10 p-4 mt-2"; // TypeScript - should NOT be parsed
---

<div class="z-10 p-4 mt-2">{title}</div>
<button class="bg-blue-500 text-white">Click</button>
"#;

        let matches = parser.parse(content, FileFormat::Astro);

        // Should find 2 matches (both in markup, not in frontmatter)
        assert_eq!(matches.len(), 2);

        let class_strings: Vec<&str> = matches.iter().map(|m| m.content.as_str()).collect();
        assert!(class_strings.contains(&"z-10 p-4 mt-2"));
        assert!(class_strings.contains(&"bg-blue-500 text-white"));

        // Verify all matches are after the frontmatter section
        let frontmatter_end = content.find("---\n\n").unwrap() + 4;
        for m in &matches {
            assert!(m.start >= frontmatter_end);
        }
    }

    #[test]
    fn test_jsx_parses_utility_functions() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
export function Button({ active }) {
  return (
    <button 
      className={clsx("z-10 p-4 mt-2", active && "bg-blue-500")}
    >
      <span className="text-white font-bold">Click me</span>
    </button>
  );
}
"#;

        let matches = parser.parse(content, FileFormat::Jsx);

        // Should find both className attributes and clsx function call
        assert!(matches.len() >= 2);

        let class_strings: Vec<&str> = matches.iter().map(|m| m.content.as_str()).collect();
        assert!(class_strings.contains(&"z-10 p-4 mt-2"));
        assert!(class_strings.contains(&"text-white font-bold"));
    }

    #[test]
    fn test_html_preserves_comments_and_structure() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"<!DOCTYPE html>
<html>
<head>
  <!-- This is a comment -->
  <title>Test</title>
</head>
<body>
  <!-- Main container -->
  <div class="z-10 p-4 mt-2">
    <h1 class="text-2xl font-bold">Title</h1>
    <!-- Nested comment -->
    <p class="text-gray-500">Paragraph</p>
  </div>
</body>
</html>"#;

        let matches = parser.parse(content, FileFormat::Html);

        // Should find 3 class attributes
        assert_eq!(matches.len(), 3);

        // Verify all matches point to actual class content
        for m in &matches {
            let extracted = &content[m.start..m.end];
            assert_eq!(extracted, m.content);

            // Verify it's not inside a comment
            let before = &content[..m.start];
            let _after = &content[m.end..];

            let comment_opens_before = before.matches("<!--").count();
            let comment_closes_before = before.matches("-->").count();

            // If there are more opens than closes before, we're inside a comment
            assert_eq!(
                comment_opens_before, comment_closes_before,
                "Match '{}' appears to be inside a comment",
                m.content
            );
        }
    }

    #[test]
    fn test_format_detection_from_file_paths() {
        assert_eq!(FileFormat::from_path("src/App.jsx"), Some(FileFormat::Jsx));
        assert_eq!(FileFormat::from_path("src/App.tsx"), Some(FileFormat::Tsx));
        assert_eq!(FileFormat::from_path("src/App.vue"), Some(FileFormat::Vue));
        assert_eq!(
            FileFormat::from_path("src/App.svelte"),
            Some(FileFormat::Svelte)
        );
        assert_eq!(
            FileFormat::from_path("src/pages/index.astro"),
            Some(FileFormat::Astro)
        );
        assert_eq!(
            FileFormat::from_path("public/index.html"),
            Some(FileFormat::Html)
        );
        assert_eq!(
            FileFormat::from_path("public/page.htm"),
            Some(FileFormat::Html)
        );

        // Unknown formats should return None
        assert_eq!(FileFormat::from_path("styles.css"), None);
        assert_eq!(FileFormat::from_path("script.js"), None);
        assert_eq!(FileFormat::from_path("data.json"), None);
    }

    #[test]
    fn test_position_tracking_across_formats() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        // Test each format to ensure positions are correct
        let test_cases = vec![
            (FileFormat::Html, r#"<div class="flex p-4">Content</div>"#),
            (
                FileFormat::Jsx,
                r#"<div className="flex p-4">Content</div>"#,
            ),
            (
                FileFormat::Vue,
                r#"<template><div class="flex p-4">Content</div></template>"#,
            ),
            (FileFormat::Svelte, r#"<div class="flex p-4">Content</div>"#),
            (FileFormat::Astro, r#"<div class="flex p-4">Content</div>"#),
        ];

        for (format, content) in test_cases {
            let matches = parser.parse(content, format);
            assert!(!matches.is_empty(), "No matches found for {:?}", format);

            // Verify each match has correct positions
            for m in matches {
                let extracted = &content[m.start..m.end];
                assert_eq!(
                    extracted, m.content,
                    "Position mismatch for {:?}: expected '{}', got '{}'",
                    format, m.content, extracted
                );
            }
        }
    }

    #[test]
    fn test_vue_with_multiline_template() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
<template>
  <div class="container mx-auto">
    <header class="z-10 p-4 mt-2">
      <h1 class="text-3xl font-bold">Title</h1>
    </header>
    <main class="py-8">
      <section class="bg-white rounded shadow">
        <p>Content</p>
      </section>
    </main>
  </div>
</template>

<script setup>
import { ref } from 'vue';
const count = ref(0);
</script>
"#;

        let matches = parser.parse(content, FileFormat::Vue);

        // Should find multiple class attributes in the template
        assert!(matches.len() >= 5);

        // Verify all are in template section
        let template_start = content.find("<template>").unwrap();
        let template_end = content.find("</template>").unwrap();

        for m in &matches {
            assert!(
                m.start > template_start && m.end < template_end,
                "Match at {}..{} is outside template section",
                m.start,
                m.end
            );
        }
    }

    #[test]
    fn test_svelte_with_reactive_statements() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
<script>
  let count = 0;
  $: doubled = count * 2;
</script>

<div class="container mx-auto">
  <button 
    class="z-10 p-4 mt-2"
    on:click={() => count++}
  >
    Count: {count}
  </button>
  {#if count > 5}
    <div class="bg-green-500 text-white">High!</div>
  {/if}
</div>
"#;

        let matches = parser.parse(content, FileFormat::Svelte);

        // Should find class attributes (not the script section)
        assert_eq!(matches.len(), 3);

        let class_strings: Vec<&str> = matches.iter().map(|m| m.content.as_str()).collect();
        assert!(class_strings.contains(&"container mx-auto"));
        assert!(class_strings.contains(&"z-10 p-4 mt-2"));
        assert!(class_strings.contains(&"bg-green-500 text-white"));
    }

    #[test]
    fn test_unknown_format_fallback() {
        let config = create_test_config();
        let extractor = ClassExtractor::new(
            config.tailwind_functions.clone(),
            config.tailwind_attributes.clone(),
        );

        // For unknown formats, the plugin should still try to extract classes
        let content = r#"<div class="flex p-4">Content</div>"#;

        // Directly use extractor (simulating unknown format fallback)
        let mut matches = extractor.extract_from_attributes(content);
        let function_matches = extractor.extract_from_functions(content);
        matches.extend(function_matches);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].content, "flex p-4");
    }
}
