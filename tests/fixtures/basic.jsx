import React from 'react';

function BasicComponent() {
  return (
    <div className="p-4 bg-red-500 flex justify-center items-center m-2 text-white">
      <span className="font-bold text-lg p-2 bg-blue-500">
        Hello World
      </span>
      <button className="mt-4 px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600">
        Click me
      </button>
    </div>
  );
}

export default BasicComponent;