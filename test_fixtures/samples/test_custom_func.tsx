import React from 'react';

const Component = () => {
  return (
    <div className="p-4 bg-red-500 flex justify-center">
      <button className={myMerge("bg-blue-500 text-white p-2 rounded hover:bg-blue-600 m-4")}>
        Click me
      </button>
      <span className={cx("text-gray-500 text-sm font-medium")}>
        Hello world
      </span>
    </div>
  );
};

export default Component;