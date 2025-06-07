import React from 'react';
import { cn } from './utils';

// Test complex conditional classes
export const ComplexComponent = ({ isActive, size = 'md' }) => {
  const baseClasses = "transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2";
  
  return (
    <div className={cn(
      "w-full h-screen bg-gray-100 flex items-center justify-center p-8",
      isActive && "bg-blue-100"
    )}>
      <div className="max-w-md mx-auto bg-white rounded-xl shadow-lg p-6">
        <button 
          className={`${baseClasses} ${
            size === 'sm' 
              ? 'px-2 py-1 text-sm' 
              : size === 'lg'
              ? 'px-6 py-3 text-lg'
              : 'px-4 py-2 text-base'
          } ${
            isActive 
              ? 'bg-blue-500 text-white hover:bg-blue-600 focus:ring-blue-500' 
              : 'bg-gray-500 text-white hover:bg-gray-600 focus:ring-gray-500'
          }`}
        >
          Complex Button
        </button>
        
        {/* Test template literals */}
        <div className={`grid grid-cols-${size === 'lg' ? '3' : '2'} gap-4 mt-4`}>
          <span className="text-gray-600 leading-relaxed">Item 1</span>
          <span className="text-gray-600 leading-relaxed">Item 2</span>
          {size === 'lg' && (
            <span className="text-gray-600 leading-relaxed">Item 3</span>
          )}
        </div>
        
        {/* Test array patterns */}
        <div className={[
          'mt-6 p-4',
          'border border-gray-200',
          'rounded-lg',
          isActive ? 'border-blue-300' : 'border-gray-200'
        ].join(' ')}>
          Array-based classes
        </div>
      </div>
    </div>
  );
};