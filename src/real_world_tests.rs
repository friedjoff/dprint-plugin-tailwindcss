/// Real-world scenario tests
/// 
/// Tests based on common patterns found in actual TailwindCSS projects
/// to ensure the plugin works correctly with real codebases.

#[cfg(test)]
mod real_world_tests {
    use crate::sorter::sort_classes;
    use crate::extractor::ClassExtractor;
    use crate::parser::{FileFormat, FormatParser};

    #[test]
    fn test_typical_button_component() {
        let classes = "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 \
                      focus:outline-none focus:ring-2 focus:ring-blue-500 \
                      focus:ring-offset-2 disabled:opacity-50 transition-colors";
        
        let result = sort_classes(classes);
        
        // All classes should be present
        assert!(result.contains("px-4"));
        assert!(result.contains("py-2"));
        assert!(result.contains("bg-blue-500"));
        assert!(result.contains("text-white"));
        assert!(result.contains("rounded"));
        assert!(result.contains("hover:bg-blue-600"));
        assert!(result.contains("focus:outline-none"));
        assert!(result.contains("transition-colors"));
    }

    #[test]
    fn test_responsive_grid_layout() {
        let classes = "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4";
        let result = sort_classes(classes);
        
        // Should maintain responsive breakpoints
        assert!(result.contains("grid"));
        assert!(result.contains("grid-cols-1"));
        assert!(result.contains("sm:grid-cols-2"));
        assert!(result.contains("md:grid-cols-3"));
        assert!(result.contains("lg:grid-cols-4"));
        assert!(result.contains("gap-4"));
    }

    #[test]
    fn test_card_component() {
        let classes = "p-6 bg-white rounded-lg shadow-md hover:shadow-lg \
                      transition-shadow border border-gray-200";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("p-6"));
        assert!(result.contains("bg-white"));
        assert!(result.contains("rounded-lg"));
        assert!(result.contains("shadow-md"));
        assert!(result.contains("border"));
    }

    #[test]
    fn test_form_input() {
        let classes = "w-full px-3 py-2 border border-gray-300 rounded-md \
                      focus:outline-none focus:ring-2 focus:ring-blue-500 \
                      focus:border-transparent placeholder-gray-400 \
                      disabled:bg-gray-100 disabled:cursor-not-allowed";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("w-full"));
        assert!(result.contains("px-3"));
        assert!(result.contains("focus:ring-2"));
        assert!(result.contains("disabled:bg-gray-100"));
    }

    #[test]
    fn test_navigation_menu() {
        let classes = "flex items-center space-x-4 text-sm font-medium \
                      text-gray-700 hover:text-gray-900";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("flex"));
        assert!(result.contains("items-center"));
        assert!(result.contains("space-x-4"));
        assert!(result.contains("text-sm"));
    }

    #[test]
    fn test_modal_overlay() {
        let classes = "fixed inset-0 bg-black bg-opacity-50 z-50 \
                      flex items-center justify-center backdrop-blur-sm";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("fixed"));
        assert!(result.contains("inset-0"));
        assert!(result.contains("z-50"));
        assert!(result.contains("flex"));
    }

    #[test]
    fn test_react_component_with_clsx() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["className".to_string()],
        );

        let content = r#"
            export function Button({ variant, size }) {
              return (
                <button
                  className={clsx(
                    "px-4 py-2 rounded font-medium transition-colors",
                    variant === "primary" && "bg-blue-500 text-white hover:bg-blue-600",
                    variant === "secondary" && "bg-gray-200 text-gray-900 hover:bg-gray-300",
                    size === "sm" && "text-sm",
                    size === "lg" && "text-lg"
                  )}
                >
                  Click me
                </button>
              );
            }
        "#;

        let matches = extractor.extract_all(content);
        
        // Should find the clsx call
        assert!(matches.len() > 0);
    }

    #[test]
    fn test_vue_component_with_dynamic_classes() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["class".to_string()],
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
<template>
  <div class="container mx-auto px-4 py-8">
    <h1 class="text-3xl font-bold mb-4">Title</h1>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div class="p-6 bg-white rounded-lg shadow-md">Card 1</div>
      <div class="p-6 bg-white rounded-lg shadow-md">Card 2</div>
      <div class="p-6 bg-white rounded-lg shadow-md">Card 3</div>
    </div>
  </div>
</template>
        "#;

        let matches = parser.parse(content, FileFormat::Vue);
        
        // Should find all class attributes in template
        assert!(matches.len() >= 5);
    }

    #[test]
    fn test_tailwind_ui_pattern() {
        // Common pattern from Tailwind UI components
        let classes = "relative inline-flex items-center px-4 py-2 border \
                      border-gray-300 bg-white text-sm font-medium text-gray-700 \
                      hover:bg-gray-50 focus:z-10 focus:outline-none focus:ring-1 \
                      focus:ring-indigo-500 focus:border-indigo-500";
        
        let result = sort_classes(classes);
        
        // Complex component with many utility classes
        assert!(result.contains("relative"));
        assert!(result.contains("inline-flex"));
        assert!(result.contains("items-center"));
    }

    #[test]
    fn test_dark_mode_component() {
        let classes = "bg-white dark:bg-gray-900 text-gray-900 dark:text-white \
                      border border-gray-200 dark:border-gray-700 \
                      hover:bg-gray-50 dark:hover:bg-gray-800";
        
        let result = sort_classes(classes);
        
        // Dark mode variants should be preserved
        assert!(result.contains("dark:bg-gray-900"));
        assert!(result.contains("dark:text-white"));
        assert!(result.contains("dark:border-gray-700"));
    }

    #[test]
    fn test_animation_and_transition() {
        let classes = "transform transition-all duration-300 ease-in-out \
                      hover:scale-105 hover:-translate-y-1 \
                      animate-fade-in";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("transform"));
        assert!(result.contains("transition-all"));
        assert!(result.contains("duration-300"));
        assert!(result.contains("hover:scale-105"));
    }

    #[test]
    fn test_complex_flex_layout() {
        let classes = "flex flex-col md:flex-row items-start md:items-center \
                      justify-between space-y-4 md:space-y-0 md:space-x-6";
        
        let result = sort_classes(classes);
        
        // Responsive flex direction change
        assert!(result.contains("flex"));
        assert!(result.contains("flex-col"));
        assert!(result.contains("md:flex-row"));
    }

    #[test]
    fn test_print_utilities() {
        let classes = "block print:hidden md:flex print:block";
        let result = sort_classes(classes);
        
        // Print variant classes
        assert!(result.contains("print:hidden"));
        assert!(result.contains("print:block"));
    }

    #[test]
    fn test_group_and_peer_interactions() {
        let classes = "group-hover:opacity-100 peer-checked:bg-blue-500 \
                      peer-focus:ring-2 group-focus:scale-105";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("group-hover:opacity-100"));
        assert!(result.contains("peer-checked:bg-blue-500"));
        assert!(result.contains("peer-focus:ring-2"));
    }

    #[test]
    fn test_arbitrary_properties() {
        let classes = "mt-[117px] bg-[url('/img/hero.jpg')] \
                      text-[#bada55] top-[var(--header-height)]";
        
        let result = sort_classes(classes);
        
        // Arbitrary values should be preserved
        assert!(result.contains("mt-[117px]"));
        assert!(result.contains("bg-[url('/img/hero.jpg')]"));
        assert!(result.contains("text-[#bada55]"));
        assert!(result.contains("top-[var(--header-height)]"));
    }

    #[test]
    fn test_data_and_aria_variants() {
        let classes = "data-[state=open]:bg-gray-100 \
                      aria-[expanded=true]:font-bold \
                      aria-disabled:opacity-50";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("data-[state=open]:bg-gray-100"));
        assert!(result.contains("aria-[expanded=true]:font-bold"));
    }

    #[test]
    fn test_container_queries_pattern() {
        let classes = "@container/sidebar:flex @container/sidebar:flex-col \
                      @lg/sidebar:grid @lg/sidebar:grid-cols-2";
        
        let result = sort_classes(classes);
        
        // Container queries with named containers
        assert!(result.contains("@container/sidebar:"));
        assert!(result.contains("@lg/sidebar:"));
    }

    #[test]
    fn test_prose_typography() {
        let classes = "prose prose-lg prose-slate dark:prose-invert \
                      max-w-none prose-headings:font-bold \
                      prose-a:text-blue-600";
        
        let result = sort_classes(classes);
        
        // Typography plugin classes
        assert!(result.contains("prose"));
        assert!(result.contains("prose-lg"));
        assert!(result.contains("dark:prose-invert"));
    }

    #[test]
    fn test_gradient_backgrounds() {
        let classes = "bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 \
                      hover:from-blue-600 hover:via-purple-600 hover:to-pink-600";
        
        let result = sort_classes(classes);
        
        assert!(result.contains("bg-gradient-to-r"));
        assert!(result.contains("from-blue-500"));
        assert!(result.contains("via-purple-500"));
        assert!(result.contains("to-pink-500"));
    }

    #[test]
    fn test_aspect_ratio_utilities() {
        let classes = "aspect-square aspect-video aspect-[4/3]";
        let result = sort_classes(classes);
        
        assert!(result.contains("aspect-square"));
        assert!(result.contains("aspect-video"));
        assert!(result.contains("aspect-[4/3]"));
    }

    #[test]
    fn test_svelte_component_realistic() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["class".to_string()],
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
<script>
  export let variant = 'primary';
  let isLoading = false;
</script>

<button
  class="px-4 py-2 rounded font-medium transition-colors
         {variant === 'primary' ? 'bg-blue-500 text-white' : 'bg-gray-200'}
         {isLoading ? 'opacity-50 cursor-wait' : 'hover:opacity-80'}"
  on:click={() => isLoading = true}
>
  <slot />
</button>
        "#;

        let matches = parser.parse(content, FileFormat::Svelte);
        
        // Should extract the static classes
        assert!(matches.len() > 0);
    }

    #[test]
    fn test_astro_component_realistic() {
        let extractor = ClassExtractor::new(
            vec!["clsx".to_string()],
            vec!["class".to_string()],
        );
        let parser = FormatParser::new(extractor);

        let content = r#"
---
interface Props {
  title: string;
  featured?: boolean;
}

const { title, featured = false } = Astro.props;
---

<article class="p-6 bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow">
  {featured && <span class="inline-block px-2 py-1 text-xs font-bold bg-yellow-400 text-gray-900 rounded">Featured</span>}
  <h2 class="text-2xl font-bold mb-2">{title}</h2>
  <div class="prose prose-sm">
    <slot />
  </div>
</article>
        "#;

        let matches = parser.parse(content, FileFormat::Astro);
        
        // Should find classes in the markup section
        assert!(matches.len() >= 3);
    }
}
