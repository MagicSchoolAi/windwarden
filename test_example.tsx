import React from 'react';

function TestComponent() {
  return (
    <div className="flex items-center justify-center m-2 p-4 text-white bg-red-500">
      <span className="p-2 font-bold text-lg bg-blue-500">
        Hello World
      </span>
      <button className="mt-4 px-6 py-2 text-white bg-green-500 hover:bg-green-600 rounded">
        Click me
      </button>
    </div>
  );
}

export default TestComponent;