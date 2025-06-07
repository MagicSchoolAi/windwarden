use std::collections::HashMap;
use once_cell::sync::Lazy;

// Order of categories in Tailwind CSS
pub static CATEGORY_ORDER: &[&str] = &[
    // Layout
    "layout",
    
    // Flexbox & Grid
    "flexbox-grid",
    
    // Spacing
    "spacing",
    
    // Sizing
    "sizing",
    
    // Typography
    "typography",
    
    // Backgrounds
    "backgrounds",
    
    // Borders
    "borders",
    
    // Effects
    "effects",
    
    // Filters
    "filters",
    
    // Tables
    "tables",
    
    // Transitions & Animation
    "transitions",
    
    // Transforms
    "transforms",
    
    // Interactivity
    "interactivity",
    
    // SVG
    "svg",
    
    // Accessibility
    "accessibility",
    
    // Unknown classes (custom, non-Tailwind)
    "unknown",
];

// Mapping of class prefixes to categories
pub static CLASS_CATEGORIES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Layout
    map.insert("aspect-", "layout");
    map.insert("container", "layout");
    map.insert("columns-", "layout");
    map.insert("break-", "layout");
    map.insert("box-", "layout");
    map.insert("block", "layout");
    map.insert("inline", "layout");
    map.insert("hidden", "layout");
    map.insert("table", "layout");
    map.insert("float-", "layout");
    map.insert("clear-", "layout");
    map.insert("isolate", "layout");
    map.insert("object-", "layout");
    map.insert("overflow-", "layout");
    map.insert("overscroll-", "layout");
    map.insert("position", "layout");
    map.insert("static", "layout");
    map.insert("fixed", "layout");
    map.insert("absolute", "layout");
    map.insert("relative", "layout");
    map.insert("sticky", "layout");
    map.insert("inset-", "layout");
    map.insert("top-", "layout");
    map.insert("right-", "layout");
    map.insert("bottom-", "layout");
    map.insert("left-", "layout");
    map.insert("z-", "layout");
    map.insert("visible", "layout");
    map.insert("invisible", "layout");

    // Flexbox & Grid
    map.insert("flex", "flexbox-grid");
    map.insert("grid", "flexbox-grid");
    map.insert("order-", "flexbox-grid");
    map.insert("col-", "flexbox-grid");
    map.insert("row-", "flexbox-grid");
    map.insert("auto-", "flexbox-grid");
    map.insert("gap-", "flexbox-grid");
    map.insert("justify-", "flexbox-grid");
    map.insert("items-", "flexbox-grid");
    map.insert("content-", "flexbox-grid");
    map.insert("self-", "flexbox-grid");
    map.insert("place-", "flexbox-grid");

    // Spacing
    map.insert("p-", "spacing");
    map.insert("px-", "spacing");
    map.insert("py-", "spacing");
    map.insert("pt-", "spacing");
    map.insert("pr-", "spacing");
    map.insert("pb-", "spacing");
    map.insert("pl-", "spacing");
    map.insert("m-", "spacing");
    map.insert("mx-", "spacing");
    map.insert("my-", "spacing");
    map.insert("mt-", "spacing");
    map.insert("mr-", "spacing");
    map.insert("mb-", "spacing");
    map.insert("ml-", "spacing");
    map.insert("space-", "spacing");

    // Sizing
    map.insert("w-", "sizing");
    map.insert("min-w-", "sizing");
    map.insert("max-w-", "sizing");
    map.insert("h-", "sizing");
    map.insert("min-h-", "sizing");
    map.insert("max-h-", "sizing");
    map.insert("size-", "sizing");

    // Typography
    map.insert("font-", "typography");
    map.insert("text-", "typography");
    map.insert("antialiased", "typography");
    map.insert("subpixel-antialiased", "typography");
    map.insert("italic", "typography");
    map.insert("not-italic", "typography");
    map.insert("tracking-", "typography");
    map.insert("leading-", "typography");
    map.insert("list-", "typography");
    map.insert("decoration-", "typography");
    map.insert("underline", "typography");
    map.insert("overline", "typography");
    map.insert("line-through", "typography");
    map.insert("no-underline", "typography");
    map.insert("uppercase", "typography");
    map.insert("lowercase", "typography");
    map.insert("capitalize", "typography");
    map.insert("normal-case", "typography");
    map.insert("truncate", "typography");
    map.insert("overflow-ellipsis", "typography");
    map.insert("overflow-clip", "typography");

    // Backgrounds
    map.insert("bg-", "backgrounds");
    map.insert("from-", "backgrounds");
    map.insert("via-", "backgrounds");
    map.insert("to-", "backgrounds");

    // Borders
    map.insert("border", "borders");
    map.insert("divide-", "borders");
    map.insert("outline-", "borders");
    map.insert("ring-", "borders");
    map.insert("rounded", "borders");

    // Effects
    map.insert("shadow", "effects");
    map.insert("opacity-", "effects");
    map.insert("mix-blend-", "effects");

    // Filters
    map.insert("blur-", "filters");
    map.insert("brightness-", "filters");
    map.insert("contrast-", "filters");
    map.insert("drop-shadow", "filters");
    map.insert("grayscale", "filters");
    map.insert("hue-rotate-", "filters");
    map.insert("invert", "filters");
    map.insert("saturate-", "filters");
    map.insert("sepia", "filters");
    map.insert("backdrop-", "filters");

    // Tables
    map.insert("border-collapse", "tables");
    map.insert("border-separate", "tables");
    map.insert("table-", "tables");

    // Transitions & Animation
    map.insert("transition", "transitions");
    map.insert("duration-", "transitions");
    map.insert("ease-", "transitions");
    map.insert("delay-", "transitions");
    map.insert("animate-", "transitions");

    // Transforms
    map.insert("transform", "transforms");
    map.insert("transform-", "transforms");
    map.insert("translate-", "transforms");
    map.insert("rotate-", "transforms");
    map.insert("skew-", "transforms");
    map.insert("scale-", "transforms");
    map.insert("origin-", "transforms");

    // Interactivity
    map.insert("accent-", "interactivity");
    map.insert("appearance-", "interactivity");
    map.insert("cursor-", "interactivity");
    map.insert("caret-", "interactivity");
    map.insert("pointer-events-", "interactivity");
    map.insert("resize", "interactivity");
    map.insert("scroll-", "interactivity");
    map.insert("select-", "interactivity");
    map.insert("touch-", "interactivity");
    map.insert("user-select-", "interactivity");
    map.insert("will-change-", "interactivity");

    // SVG
    map.insert("fill-", "svg");
    map.insert("stroke-", "svg");

    // Accessibility
    map.insert("sr-only", "accessibility");
    map.insert("not-sr-only", "accessibility");

    map
});