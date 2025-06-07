import React from 'react';

export const AlreadyFormattedComponent = () => (
  <div className="flex items-center justify-center p-4">
    <div className="rounded-lg bg-white p-6 shadow-lg">
      <h1 className="mb-4 text-2xl font-bold text-gray-900">
        Already Formatted
      </h1>
      <p className="leading-relaxed text-gray-600">
        This component already has properly sorted Tailwind classes.
      </p>
      <button className="mt-4 rounded bg-blue-500 px-4 py-2 text-white hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50">
        Click me
      </button>
    </div>
  </div>
);