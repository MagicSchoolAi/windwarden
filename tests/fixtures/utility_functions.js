import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs) {
  return twMerge(clsx(inputs));
}

const buttonClasses = cn(
  "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
);

const cardClasses = clsx(
  "w-full max-w-sm mx-auto bg-white rounded-lg shadow-md overflow-hidden"
);

export { buttonClasses, cardClasses };