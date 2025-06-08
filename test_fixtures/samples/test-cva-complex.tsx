import { cva } from "class-variance-authority";

const buttonVariants = cva(['p-4', 'flex', 'm-2', 'items-center'], {
  variants: {
    size: {
      sm: ['text-sm', 'p-2', 'gap-1'],
      md: ['text-base', 'p-3', 'gap-2'],  
      lg: ['text-lg', 'p-6', 'gap-4']
    },
    variant: {
      primary: ['bg-blue-500', 'text-white', 'hover:bg-blue-600'],
      secondary: ['bg-gray-200', 'text-gray-900', 'hover:bg-gray-300'],
      outline: ['border-2', 'border-blue-500', 'text-blue-500', 'hover:bg-blue-50']
    }
  }
});

const cardVariants = cva(['bg-white', 'rounded-lg', 'shadow'], {
  variants: {
    padding: {
      sm: ['p-2', 'gap-1'],
      lg: ['p-6', 'gap-4']
    }
  }
});