use std::cmp::Ordering;

/// Represents a parsed TailwindCSS class with its components
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct TailwindClass {
    /// Original class string
    pub original: String,
    /// Important modifier (!)
    pub important: bool,
    /// Variants (e.g., ["dark", "hover", "md"])
    pub variants: Vec<String>,
    /// Base class name (e.g., "text-red-500")
    pub base: String,
    /// Negative modifier (-)
    pub negative: bool,
    /// Arbitrary value (e.g., "[100px]")
    pub arbitrary: bool,
}

impl TailwindClass {
    /// Parse a TailwindCSS class string into its components
    #[allow(dead_code)]
    pub fn parse(class: &str) -> Self {
        let class = class.trim();
        let mut remaining = class;

        // Check for important modifier
        let important = remaining.starts_with('!');
        if important {
            remaining = &remaining[1..];
        }

        // Split variants and base class
        let parts: Vec<&str> = remaining.split(':').collect();
        let (variants, base_part) = if parts.len() > 1 {
            let variants = parts[..parts.len() - 1]
                .iter()
                .map(|s| s.to_string())
                .collect();
            (variants, parts[parts.len() - 1])
        } else {
            (Vec::new(), parts[0])
        };

        // Check for negative modifier
        let negative = base_part.starts_with('-');
        let base_without_neg = if negative { &base_part[1..] } else { base_part };

        // Check for arbitrary value
        let arbitrary = base_without_neg.contains('[');

        TailwindClass {
            original: class.to_string(),
            important,
            variants,
            base: base_without_neg.to_string(),
            negative,
            arbitrary,
        }
    }

    /// Get the category priority for sorting
    /// Based on TailwindCSS official class order
    #[allow(dead_code)]
    fn category_priority(&self) -> u32 {
        // Extract the utility prefix (e.g., "text" from "text-red-500")
        let prefix = self.base.split('-').next().unwrap_or(&self.base);

        // TailwindCSS recommended order following Prettier plugin
        match prefix {
            // Layout - Display, Position, Overflow
            "container" | "box" | "block" | "inline" | "hidden" => 100,
            "float" | "clear" | "object" | "overflow" | "overscroll" => 110,

            // Flexbox & Grid
            "flex" | "grow" | "shrink" | "basis" | "order" => 200,
            "grid" | "col" | "row" | "gap" | "auto" | "justify" | "items" | "content" | "place" => {
                210
            }

            // Spacing (margin, padding) - comes EARLY in Tailwind order
            "m" | "mx" | "my" | "mt" | "mr" | "mb" | "ml" | "margin" => 300,
            "p" | "px" | "py" | "pt" | "pr" | "pb" | "pl" | "padding" => 310,
            "space" => 320,

            // Sizing
            "w" | "width" | "h" | "height" => 400,
            "min" | "max" => 410,

            // Position & Z-Index - comes AFTER spacing
            "position" | "static" | "fixed" | "absolute" | "relative" | "sticky" => 500,
            "top" | "right" | "bottom" | "left" | "inset" => 510,
            "z" => 520,

            // Typography
            "font" | "text" | "tracking" | "leading" | "list" | "align" => 600,
            "whitespace" | "break" | "truncate" => 610,

            // Backgrounds
            "bg" | "from" | "via" | "to" => 700,

            // Borders
            "border" | "divide" | "outline" | "ring" => 800,
            "rounded" => 810,

            // Effects
            "shadow" | "opacity" | "mix" | "blur" => 900,

            // Filters
            "filter" | "backdrop" | "brightness" | "contrast" | "grayscale" => 1000,

            // Tables
            "caption" | "table" => 1100,

            // Transitions & Animation
            "transition" | "duration" | "ease" | "delay" | "animate" => 1200,

            // Transforms
            "transform" | "origin" | "scale" | "rotate" | "translate" | "skew" => 1300,

            // Interactivity
            "cursor" | "select" | "resize" | "pointer" | "appearance" => 1400,

            // SVG
            "fill" | "stroke" => 1500,

            // Accessibility
            "sr" | "screen" => 1600,

            // Custom/Unknown - last
            _ => 9999,
        }
    }

    /// Get the variant priority for sorting
    #[allow(dead_code)]
    fn variant_priority(variant: &str) -> u32 {
        match variant {
            // Responsive breakpoints
            "sm" => 100,
            "md" => 110,
            "lg" => 120,
            "xl" => 130,
            "2xl" => 140,

            // Dark mode
            "dark" => 200,

            // State variants
            "hover" => 300,
            "focus" => 310,
            "active" => 320,
            "visited" => 330,
            "disabled" => 340,
            "enabled" => 350,

            // Group/Peer
            "group" => 400,
            "peer" => 410,

            // Position
            "first" => 500,
            "last" => 510,
            "odd" => 520,
            "even" => 530,

            // Other
            _ => 9999,
        }
    }

    /// Compare variants for sorting
    #[allow(dead_code)]
    fn compare_variants(&self, other: &Self) -> Ordering {
        // Compare variant count first
        match self.variants.len().cmp(&other.variants.len()) {
            Ordering::Equal => {
                // Compare each variant by priority
                for (v1, v2) in self.variants.iter().zip(other.variants.iter()) {
                    let p1 = Self::variant_priority(v1);
                    let p2 = Self::variant_priority(v2);
                    match p1.cmp(&p2) {
                        Ordering::Equal => {
                            // If same priority, compare alphabetically
                            match v1.cmp(v2) {
                                Ordering::Equal => continue,
                                other => return other,
                            }
                        }
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for TailwindClass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TailwindClass {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. Non-important classes first, important classes last
        match self.important.cmp(&other.important) {
            Ordering::Equal => {}
            other => return other,
        }

        // 2. Compare by category priority
        let cat1 = self.category_priority();
        let cat2 = other.category_priority();
        match cat1.cmp(&cat2) {
            Ordering::Equal => {}
            other => return other,
        }

        // 3. Within same category: classes without variants first
        match self.variants.len().cmp(&other.variants.len()) {
            Ordering::Equal => {}
            other => return other,
        }

        // 4. Compare by variants if both have variants
        if !self.variants.is_empty() || !other.variants.is_empty() {
            match self.compare_variants(other) {
                Ordering::Equal => {}
                other => return other,
            }
        }

        // 5. Positive values before negative
        match self.negative.cmp(&other.negative) {
            Ordering::Equal => {}
            other => return other,
        }

        // 6. Non-arbitrary before arbitrary values
        match self.arbitrary.cmp(&other.arbitrary) {
            Ordering::Equal => {}
            other => return other,
        }

        // 7. Finally, compare base class names alphabetically
        self.base.cmp(&other.base)
    }
}

/// Sort a space-separated list of TailwindCSS classes
#[allow(dead_code)]
pub fn sort_classes(classes: &str) -> String {
    let trimmed = classes.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // Parse all classes
    let mut parsed_classes: Vec<TailwindClass> = trimmed
        .split_whitespace()
        .map(TailwindClass::parse)
        .collect();

    // Sort the classes
    parsed_classes.sort();

    // Reconstruct the string
    parsed_classes
        .iter()
        .map(|c| c.original.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_class() {
        let class = TailwindClass::parse("text-red-500");
        assert_eq!(class.base, "text-red-500");
        assert!(!class.important);
        assert!(!class.negative);
        assert!(!class.arbitrary);
        assert!(class.variants.is_empty());
    }

    #[test]
    fn test_parse_important_class() {
        let class = TailwindClass::parse("!bg-blue-500");
        assert_eq!(class.base, "bg-blue-500");
        assert!(class.important);
        assert!(!class.negative);
        assert!(!class.arbitrary);
    }

    #[test]
    fn test_parse_negative_class() {
        let class = TailwindClass::parse("-mt-4");
        assert_eq!(class.base, "mt-4");
        assert!(!class.important);
        assert!(class.negative);
        assert!(!class.arbitrary);
    }

    #[test]
    fn test_parse_arbitrary_value() {
        let class = TailwindClass::parse("w-[100px]");
        assert_eq!(class.base, "w-[100px]");
        assert!(!class.important);
        assert!(!class.negative);
        assert!(class.arbitrary);
    }

    #[test]
    fn test_parse_with_variants() {
        let class = TailwindClass::parse("hover:bg-blue-500");
        assert_eq!(class.base, "bg-blue-500");
        assert_eq!(class.variants, vec!["hover"]);
        assert!(!class.important);
    }

    #[test]
    fn test_parse_with_multiple_variants() {
        let class = TailwindClass::parse("dark:hover:focus:text-white");
        assert_eq!(class.base, "text-white");
        assert_eq!(class.variants, vec!["dark", "hover", "focus"]);
    }

    #[test]
    fn test_parse_complex_class() {
        let class = TailwindClass::parse("!md:hover:-mt-[20px]");
        assert_eq!(class.base, "mt-[20px]");
        assert!(class.important);
        assert!(class.negative);
        assert!(class.arbitrary);
        assert_eq!(class.variants, vec!["md", "hover"]);
    }

    #[test]
    fn test_sort_simple_classes() {
        let input = "z-10 p-4 mt-2";
        let expected = "mt-2 p-4 z-10";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_with_variants() {
        let input = "hover:bg-blue-500 bg-red-500";
        let expected = "bg-red-500 hover:bg-blue-500";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_responsive_breakpoints() {
        let input = "xl:text-xl md:text-md text-base";
        let expected = "text-base md:text-md xl:text-xl";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_with_negative_values() {
        let input = "-mt-4 mt-4 pt-4";
        let expected = "mt-4 -mt-4 pt-4";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_with_important() {
        let input = "!text-red-500 text-blue-500";
        let expected = "text-blue-500 !text-red-500";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_with_arbitrary_values() {
        let input = "w-[100px] w-full";
        let expected = "w-full w-[100px]";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_mixed_complex() {
        let input = "z-10 hover:bg-blue-500 p-4 mt-2 !font-bold md:text-lg -mb-4 bg-white";
        let expected = "mt-2 -mb-4 p-4 z-10 md:text-lg bg-white hover:bg-blue-500 !font-bold";
        assert_eq!(sort_classes(input), expected);
    }

    #[test]
    fn test_sort_empty_string() {
        assert_eq!(sort_classes(""), "");
        assert_eq!(sort_classes("   "), "");
    }

    #[test]
    fn test_sort_single_class() {
        assert_eq!(sort_classes("text-red-500"), "text-red-500");
    }

    #[test]
    fn test_sort_preserves_unique_classes() {
        let input = "text-red-500 bg-blue-500 p-4";
        let result = sort_classes(input);
        assert!(result.contains("text-red-500"));
        assert!(result.contains("bg-blue-500"));
        assert!(result.contains("p-4"));
    }

    #[test]
    fn test_category_priority_layout() {
        let c1 = TailwindClass::parse("block");
        let c2 = TailwindClass::parse("text-red-500");
        assert!(c1 < c2);
    }

    #[test]
    fn test_category_priority_spacing() {
        let c1 = TailwindClass::parse("mt-4");
        let c2 = TailwindClass::parse("text-red-500");
        assert!(c1 < c2);
    }

    #[test]
    fn test_variant_priority_responsive() {
        let c1 = TailwindClass::parse("sm:text-sm");
        let c2 = TailwindClass::parse("lg:text-lg");
        assert!(c1 < c2);
    }

    #[test]
    fn test_variant_priority_state() {
        let c1 = TailwindClass::parse("hover:text-blue-500");
        let c2 = TailwindClass::parse("focus:text-red-500");
        assert!(c1 < c2);
    }

    #[test]
    fn test_real_world_example_1() {
        let input =
            "shadow-lg rounded-lg p-6 bg-white text-gray-900 hover:shadow-xl transition-shadow";
        let result = sort_classes(input);
        // Should be grouped by category: spacing, typography, backgrounds, borders, effects, transitions
        let classes: Vec<&str> = result.split_whitespace().collect();

        // Verify all classes are present
        assert_eq!(classes.len(), 7);
        assert!(result.contains("p-6"));
        assert!(result.contains("text-gray-900"));
        assert!(result.contains("bg-white"));
        assert!(result.contains("rounded-lg"));
        assert!(result.contains("shadow-lg"));
        assert!(result.contains("transition-shadow"));
        assert!(result.contains("hover:shadow-xl"));
    }

    #[test]
    fn test_real_world_example_2() {
        let input = "flex items-center justify-between w-full h-16 px-4 bg-gray-800 text-white";
        let result = sort_classes(input);
        let classes: Vec<&str> = result.split_whitespace().collect();

        assert_eq!(classes.len(), 8);
        // Layout should come first
        assert!(result.starts_with("flex"));
    }
}
