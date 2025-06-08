import React from 'react';

// This demonstrates custom function names working
const ExampleComponent = () => {
  return (
    <div className="flex justify-center bg-gray-100 p-4">
      {/* Default supported function: cn */}
      <button className={cn("bg-blue-500 text-white p-2 rounded hover:bg-blue-600")}>
        Default Function
      </button>
      
      {/* Custom functions (myMerge, cx) - only work when added to config */}
      <button className={myMerge("bg-green-500 text-white p-2 rounded hover:bg-green-600")}>
        Custom Function 1
      </button>
      
      <span className={cx("text-gray-600 text-sm font-medium")}>
        Custom Function 2
      </span>
    </div>
  );
};