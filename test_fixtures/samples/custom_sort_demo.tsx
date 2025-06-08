import React from 'react';

// Comprehensive demo of custom sorting
export const CustomSortDemo = () => {
  return (
    <div className="filter-blur backdrop-blur-sm border-4 border-dashed border-blue-500 p-8 m-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white text-xl font-bold shadow-2xl rounded-lg flex flex-col items-center justify-center gap-4 w-full h-screen transform rotate-1 hover:rotate-0 transition-all duration-300 ease-in-out">
      <h1 className="font-extrabold text-4xl tracking-wider uppercase transform scale-110 hover:scale-125 transition-transform duration-200">
        Custom Sort Demo
      </h1>
      
      <p className="text-center text-lg leading-relaxed max-w-md opacity-90 hover:opacity-100 transition-opacity">
        This demonstrates custom category ordering in WindWarden.
      </p>
    </div>
  );
};