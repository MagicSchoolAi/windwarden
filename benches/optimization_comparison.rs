use criterion::{black_box, criterion_group, criterion_main, Criterion};
use windwarden::optimizations::{sort_classes_optimized, FastPathOptimizer};
use windwarden::sorter::TailwindSorter;

fn bench_sorter_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorter_optimization");

    let test_classes = vec![
        "p-4 bg-red-500 flex justify-center items-center m-2 text-white",
        "w-full h-screen bg-gray-100 flex items-center justify-center p-8",
        "max-w-md mx-auto bg-white rounded-xl shadow-lg p-6",
        "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50",
        "inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background",
    ];

    // Original sorter
    group.bench_function("original", |b| {
        let sorter = TailwindSorter::new();
        b.iter(|| {
            for class_str in &test_classes {
                black_box(sorter.sort_classes(class_str));
            }
        })
    });

    // Optimized sorter
    group.bench_function("optimized", |b| {
        b.iter(|| {
            for class_str in &test_classes {
                black_box(sort_classes_optimized(class_str));
            }
        })
    });

    group.finish();
}

fn bench_fast_path_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_path");

    let test_files = vec![
        // File that needs processing
        r#"import React from 'react';
export const Component = () => (
  <div className="p-4 bg-red-500 flex justify-center items-center m-2 text-white">
    Test
  </div>
);"#,
        // File that doesn't need processing
        r#"import React from 'react';
export const Component = () => (
  <div>
    No classes here
  </div>
);"#,
        // File with single class
        r#"import React from 'react';
export const Component = () => (
  <div className="container">
    Single class
  </div>
);"#,
    ];

    group.bench_function("fast_path_check", |b| {
        b.iter(|| {
            for content in &test_files {
                black_box(FastPathOptimizer::needs_processing(content));
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_sorter_comparison,
    bench_fast_path_optimization
);
criterion_main!(benches);
