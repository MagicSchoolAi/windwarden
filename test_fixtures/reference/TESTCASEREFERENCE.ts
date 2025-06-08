// WindWarden Test Cases Reference
// Each test case includes input, expected output, and context

interface TestCase {
  name: string;
  description: string;
  input: string;
  expected: string;
  fileType: 'tsx' | 'jsx' | 'ts' | 'js' | 'html' | 'vue' | 'svelte';
  shouldProcess: boolean; // Some patterns should be skipped
}

const testCases: TestCase[] = [
  // ===== BASIC JSX ATTRIBUTES =====
  {
    name: "basic_jsx_classname",
    description: "Simple className attribute",
    input: '<div className="p-4 flex m-2 items-center">',
    expected: '<div className="flex items-center m-2 p-4">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "basic_jsx_class",
    description: "Simple class attribute (non-React)",
    input: '<div class="p-4 flex m-2 items-center">',
    expected: '<div class="flex items-center m-2 p-4">',
    fileType: 'jsx',
    shouldProcess: true
  },
  {
    name: "jsx_single_quotes",
    description: "Single quotes should be preserved",
    input: "<div className='p-4 flex m-2 items-center'>",
    expected: "<div className='flex items-center m-2 p-4'>",
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "jsx_duplicates",
    description: "Remove duplicate classes",
    input: '<div className="flex p-4 flex items-center p-4">',
    expected: '<div className="flex items-center p-4">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "jsx_variants",
    description: "Sort with responsive and state variants",
    input: '<div className="p-4 hover:bg-blue-500 flex md:flex-row flex-col hover:p-6 sm:p-2">',
    expected: '<div className="flex flex-col p-4 hover:bg-blue-500 hover:p-6 sm:p-2 md:flex-row">',
    fileType: 'tsx',
    shouldProcess: true
  },

  // ===== FUNCTION CALLS =====
  {
    name: "cn_basic",
    description: "Basic cn() function call",
    input: 'cn("p-4 flex m-2 items-center")',
    expected: 'cn("flex items-center m-2 p-4")',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "cn_multiple_args",
    description: "cn() with multiple string arguments",
    input: 'cn("p-4 flex", "m-2 items-center", "bg-white dark:bg-black")',
    expected: 'cn("flex p-4", "items-center m-2", "bg-white dark:bg-black")',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "cn_with_conditionals",
    description: "cn() with conditional arguments (non-string args preserved)",
    input: 'cn("p-4 flex", isActive && "bg-blue-500 text-white", "m-2")',
    expected: 'cn("flex p-4", isActive && "bg-blue-500 text-white", "m-2")',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "cn_with_objects",
    description: "cn() with object arguments should be skipped",
    input: 'cn("p-4 flex", { "bg-blue-500": isActive }, "m-2")',
    expected: 'cn("flex p-4", { "bg-blue-500": isActive }, "m-2")',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "twMerge_basic",
    description: "twMerge() function call",
    input: 'twMerge("p-4 flex m-2", "p-2 items-center")',
    expected: 'twMerge("flex m-2 p-4", "items-center p-2")',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "clsx_basic",
    description: "clsx() function call",
    input: 'clsx("p-4 flex m-2 items-center")',
    expected: 'clsx("flex items-center m-2 p-4")',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "custom_function",
    description: "Custom utility function (when configured)",
    input: 'myMerge("p-4 flex m-2 items-center")',
    expected: 'myMerge("flex items-center m-2 p-4")',
    fileType: 'tsx',
    shouldProcess: true
  },

  // ===== TEMPLATE LITERALS =====
  {
    name: "template_literal_static",
    description: "Static template literal",
    input: "className={`p-4 flex m-2 items-center`}",
    expected: "className={`flex items-center m-2 p-4`}",
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "template_literal_dynamic",
    description: "Template literal with variables (skip sorting)",
    input: "className={`p-4 ${baseStyles} m-2 items-center`}",
    expected: "className={`p-4 ${baseStyles} m-2 items-center`}",
    fileType: 'tsx',
    shouldProcess: false
  },
  {
    name: "tagged_template_tw",
    description: "Tagged template literal",
    input: "const styles = tw`p-4 flex m-2 items-center`",
    expected: "const styles = tw`flex items-center m-2 p-4`",
    fileType: 'tsx',
    shouldProcess: true
  },

  // ===== ARRAYS =====
  {
    name: "array_basic",
    description: "Basic array of classes",
    input: 'className={["p-4", "flex", "m-2", "items-center"].join(" ")}',
    expected: 'className={["flex", "items-center", "m-2", "p-4"].join(" ")}',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "cva_basic",
    description: "CVA base classes array",
    input: "cva(['p-4', 'flex', 'm-2', 'items-center'], { variants: {} })",
    expected: "cva(['flex', 'items-center', 'm-2', 'p-4'], { variants: {} })",
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "cva_with_variants",
    description: "CVA with variant arrays",
    input: `cva(['p-4', 'flex'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2', 'gap-1'],
      lg: ['text-lg', 'p-6', 'gap-4']
    }
  }
})`,
    expected: `cva(['flex', 'p-4'], {
  variants: {
    size: {
      sm: ['gap-1', 'p-2', 'text-sm'],
      lg: ['gap-4', 'p-6', 'text-lg']
    }
  }
})`,
    fileType: 'tsx',
    shouldProcess: true
  },

  // ===== COMPLEX PATTERNS =====
  {
    name: "nested_cn_calls",
    description: "Nested cn() calls",
    input: 'cn("p-4", cn("flex m-2", "items-center"))',
    expected: 'cn("p-4", cn("flex m-2", "items-center"))',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "multiline_string",
    description: "Multiline class string",
    input: `className={
  "p-4 flex m-2 " +
  "items-center bg-white " +
  "hover:bg-gray-100"
}`,
    expected: `className={
  "flex items-center m-2 " +
  "p-4 bg-white " +
  "hover:bg-gray-100"
}`,
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "object_syntax",
    description: "Object with className property",
    input: 'const props = { className: "p-4 flex m-2 items-center" }',
    expected: 'const props = { className: "flex items-center m-2 p-4" }',
    fileType: 'tsx',
    shouldProcess: true
  },

  // ===== EDGE CASES =====
  {
    name: "empty_string",
    description: "Empty className",
    input: '<div className="">',
    expected: '<div className="">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "single_class",
    description: "Single class",
    input: '<div className="flex">',
    expected: '<div className="flex">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "arbitrary_values",
    description: "Tailwind arbitrary values",
    input: '<div className="p-[23px] flex m-[1.5rem] items-center">',
    expected: '<div className="flex items-center m-[1.5rem] p-[23px]">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "important_modifier",
    description: "Important modifier",
    input: '<div className="!p-4 flex !m-2 items-center">',
    expected: '<div className="flex items-center !m-2 !p-4">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "negative_values",
    description: "Negative value classes",
    input: '<div className="-m-4 flex p-2 -translate-x-2">',
    expected: '<div className="flex -translate-x-2 -m-4 p-2">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "custom_classes",
    description: "Non-Tailwind classes mixed with Tailwind",
    input: '<div className="my-custom-class p-4 flex another-custom m-2">',
    expected: '<div className="my-custom-class flex another-custom m-2 p-4">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "whitespace_variations",
    description: "Various whitespace patterns",
    input: '<div className="  p-4   flex    m-2  items-center  ">',
    expected: '<div className="flex items-center m-2 p-4">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "dynamic_bracket_notation",
    description: "Dynamic values in brackets (preserve)",
    input: '<div className={`p-[${padding}px] flex m-2`}>',
    expected: '<div className={`p-[${padding}px] flex m-2`}>',
    fileType: 'tsx',
    shouldProcess: false
  },

  // ===== FRAMEWORK SPECIFIC =====
  {
    name: "vue_static_class",
    description: "Vue static class",
    input: '<div class="p-4 flex m-2 items-center">',
    expected: '<div class="flex items-center m-2 p-4">',
    fileType: 'vue',
    shouldProcess: true
  },
  {
    name: "vue_bind_class",
    description: "Vue :class binding",
    input: '<div :class="\'p-4 flex m-2 items-center\'">',
    expected: '<div :class="\'flex items-center m-2 p-4\'">',
    fileType: 'vue',
    shouldProcess: true
  },
  {
    name: "svelte_class",
    description: "Svelte class attribute",
    input: '<div class="p-4 flex m-2 items-center">',
    expected: '<div class="flex items-center m-2 p-4">',
    fileType: 'svelte',
    shouldProcess: true
  },

  // ===== MALFORMED/INVALID CASES =====
  {
    name: "unclosed_quote",
    description: "Unclosed quote (should not crash)",
    input: '<div className="p-4 flex m-2>',
    expected: '<div className="p-4 flex m-2>',
    fileType: 'tsx',
    shouldProcess: false
  },
  {
    name: "syntax_error_in_function",
    description: "Syntax error in function call",
    input: 'cn("p-4 flex", , "m-2")',
    expected: 'cn("p-4 flex", , "m-2")',
    fileType: 'tsx',
    shouldProcess: false
  },

  // ===== PERFORMANCE STRESS TESTS =====
  {
    name: "very_long_class_list",
    description: "100+ classes in single attribute",
    input: '<div className="' + Array(100).fill(0).map((_, i) => `class-${i}`).join(' ') + '">',
    expected: '<div className="' + Array(100).fill(0).map((_, i) => `class-${i}`).sort().join(' ') + '">',
    fileType: 'tsx',
    shouldProcess: true
  },
  {
    name: "deeply_nested_structure",
    description: "Deeply nested JSX with classes at various levels",
    input: `
<div className="p-4 flex">
  <div className="m-2 grid">
    <div className="gap-4 items-center">
      <span className="text-sm font-bold">Text</span>
    </div>
  </div>
</div>`,
    expected: `
<div className="flex p-4">
  <div className="grid m-2">
    <div className="items-center gap-4">
      <span className="font-bold text-sm">Text</span>
    </div>
  </div>
</div>`,
    fileType: 'tsx',
    shouldProcess: true
  }
];

// Test verification function
function verifyTestCase(testCase: TestCase, actualOutput: string): TestResult {
  const passed = actualOutput === testCase.expected;
  return {
    name: testCase.name,
    passed,
    expected: testCase.expected,
    actual: actualOutput,
    shouldHaveProcessed: testCase.shouldProcess,
    message: passed ? 'PASS' : `FAIL: Expected "${testCase.expected}" but got "${actualOutput}"`
  };
}

// Category order reference for sorting
const TAILWIND_CATEGORY_ORDER = [
  // Layout
  'aspect', 'container', 'columns', 'break', 'box', 'display', 'float', 'clear', 'isolation', 'object', 'overflow', 'overscroll', 'position', 'visibility', 'z',

  // Flexbox & Grid
  'flex', 'grid', 'order', 'justify', 'items', 'content', 'self', 'place', 'gap',

  // Spacing
  'p', 'px', 'py', 'pt', 'pr', 'pb', 'pl', 'm', 'mx', 'my', 'mt', 'mr', 'mb', 'ml', 'space',

  // Sizing
  'w', 'h', 'size', 'min-w', 'max-w', 'min-h', 'max-h',

  // Typography
  'font', 'text', 'tracking', 'leading', 'list', 'decoration', 'underline', 'uppercase', 'lowercase', 'capitalize',

  // Backgrounds
  'bg', 'gradient',

  // Borders
  'border', 'divide', 'ring', 'outline',

  // Effects
  'shadow', 'opacity', 'mix-blend', 'filter', 'backdrop',

  // Transitions & Animation
  'transition', 'duration', 'ease', 'delay', 'animate',

  // Transforms
  'transform', 'translate', 'rotate', 'skew', 'scale', 'origin',

  // Interactivity
  'cursor', 'select', 'resize', 'scroll', 'touch', 'will-change',

  // SVG
  'fill', 'stroke',

  // Accessibility
  'sr-only', 'not-sr-only'
];

// Variant order reference
const VARIANT_ORDER = [
  // Responsive
  'sm:', 'md:', 'lg:', 'xl:', '2xl:',

  // Dark mode
  'dark:',

  // States
  'hover:', 'focus:', 'focus-within:', 'focus-visible:', 'active:', 'visited:', 'disabled:',

  // Group states
  'group-hover:', 'group-focus:', 'group-active:', 'group-visited:',

  // Peer states
  'peer-hover:', 'peer-focus:', 'peer-active:', 'peer-visited:',

  // Other
  'first:', 'last:', 'odd:', 'even:', 'first-of-type:', 'last-of-type:',
  'empty:', 'before:', 'after:', 'placeholder:', 'selection:',
  'rtl:', 'ltr:', 'open:'
];

export { testCases, TAILWIND_CATEGORY_ORDER, VARIANT_ORDER, verifyTestCase };
