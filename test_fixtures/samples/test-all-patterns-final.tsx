import { cn, cva } from "lib/utils";

// Template literals
const baseStyles = tw`p-4 flex m-2 items-center bg-white rounded-lg shadow`;

// CVA with nested arrays
const buttonVariants = cva(['p-4', 'flex', 'm-2', 'items-center'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2', 'gap-1'],
      lg: ['text-lg', 'p-6', 'gap-4']
    },
    variant: {
      primary: ['bg-blue-500', 'text-white', 'hover:bg-blue-600'],
      secondary: ['bg-gray-200', 'text-gray-900', 'hover:bg-gray-300']
    }
  }
});

// Nested function calls with mixed patterns
export function ComplexButton({ className, variant, size, ...props }) {
  return (
    <button
      className={cn(
        "p-4 flex items-center bg-blue-500 text-white font-semibold rounded-lg",
        cn("border-2 shadow-sm", variant === "outline" && "border-blue-500 bg-transparent text-blue-500"),
        twMerge("hover:bg-blue-600 focus:ring-2", "focus:ring-blue-300 focus:outline-none"),
        size === "sm" && cn("text-sm p-2", `gap-1 shadow-sm border`),
        className
      )}
      {...props}
    >
      {/* JSX with nested template and array */}
      <span className={`${cn("mr-2 text-sm", ["font-bold", "tracking-wide"])} ${baseStyles}`}>
        Button Text
      </span>
    </button>
  );
}