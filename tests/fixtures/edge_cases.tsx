import React from 'react';

// Test edge cases and complex patterns

// Empty className
export const EmptyClass = () => <div className="">Empty</div>;

// Single class
export const SingleClass = () => <div className="p-4">Single</div>;

// Whitespace handling
export const WhitespaceTest = () => (
  <div className="  p-4   m-2   bg-red-500  ">
    Whitespace test
  </div>
);

// Very long class list
export const LongClassList = () => (
  <div className="p-1 p-2 p-3 p-4 m-1 m-2 m-3 m-4 bg-red-100 bg-red-200 bg-red-300 bg-red-500 text-white text-black text-gray-500 font-bold font-normal font-light text-xs text-sm text-base text-lg text-xl text-2xl text-3xl border border-2 border-4 border-red-500 border-blue-500 rounded rounded-sm rounded-md rounded-lg rounded-xl rounded-full flex flex-col flex-row items-start items-center items-end justify-start justify-center justify-end justify-between justify-around justify-evenly">
    Very long class list
  </div>
);

// Duplicate classes
export const DuplicateClasses = () => (
  <div className="p-4 p-4 m-2 m-2 bg-red-500 bg-red-500 text-white text-white">
    Duplicate classes
  </div>
);

// Mixed quotes in attributes
export const MixedQuotes = () => (
  <div className='p-4 m-2 bg-red-500' data-testid="test">
    Mixed quotes
  </div>
);

// Conditional classes with complex expressions
export const ConditionalComplex = ({ isActive, size, theme, disabled }) => (
  <div 
    className={`
      ${isActive ? 'bg-blue-500 text-white' : 'bg-gray-200 text-gray-700'}
      ${size === 'sm' ? 'p-2 text-sm' : size === 'lg' ? 'p-6 text-lg' : 'p-4 text-base'}
      ${theme === 'dark' ? 'border-gray-600' : 'border-gray-300'}
      ${disabled ? 'opacity-50 cursor-not-allowed' : 'hover:shadow-lg cursor-pointer'}
      transition-all duration-200 border rounded
    `}
  >
    Complex conditional
  </div>
);

// Nested ternary operators
export const NestedTernary = ({ status }) => (
  <div className={
    status === 'success' 
      ? 'bg-green-500 text-white border-green-600' 
      : status === 'warning'
      ? 'bg-yellow-500 text-black border-yellow-600'
      : status === 'error'
      ? 'bg-red-500 text-white border-red-600'
      : 'bg-gray-500 text-white border-gray-600'
  }>
    Nested ternary
  </div>
);

// Class attribute with newlines
export const MultilineClass = () => (
  <div 
    className="
      p-4 
      m-2 
      bg-red-500 
      text-white 
      rounded 
      shadow-lg
    "
  >
    Multiline class
  </div>
);

// Special characters in class names (should be preserved)
export const SpecialChars = () => (
  <div className="before:content-[''] after:content-[''] hover:scale-105 focus:ring-2 peer-checked:bg-blue-500 group-hover:text-white">
    Special characters
  </div>
);

// Custom CSS variables and arbitrary values
export const ArbitraryValues = () => (
  <div className="top-[117px] bg-[#1da1f2] text-[14px] w-[calc(100%-2rem)] h-[var(--custom-height)]">
    Arbitrary values
  </div>
);

// Component with spread props
export const SpreadProps = ({ className, ...props }) => (
  <div 
    className={`p-4 m-2 bg-red-500 text-white ${className || ''}`}
    {...props}
  >
    Spread props
  </div>
);