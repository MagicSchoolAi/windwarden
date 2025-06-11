use criterion::{Criterion, black_box, criterion_group, criterion_main};
use windwarden::sorter::TailwindSorter;

fn bench_sorter_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorter_performance");

    let test_classes = vec![
        "p-4 bg-red-500 flex justify-center items-center m-2 text-white",
        "w-full h-screen bg-gray-100 flex items-center justify-center p-8",
        "max-w-md mx-auto bg-white rounded-xl shadow-lg p-6",
        "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50",
        "inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background",
    ];

    // Single sorter instance (reused)
    group.bench_function("reused_sorter", |b| {
        let sorter = TailwindSorter::new();
        b.iter(|| {
            for class_str in &test_classes {
                black_box(sorter.sort_classes(class_str));
            }
        })
    });

    // New sorter instance each time
    group.bench_function("new_sorter_each_time", |b| {
        b.iter(|| {
            for class_str in &test_classes {
                let sorter = TailwindSorter::new();
                black_box(sorter.sort_classes(class_str));
            }
        })
    });

    group.finish();
}

fn bench_class_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("class_complexity");
    let sorter = TailwindSorter::new();

    // Simple classes
    group.bench_function("simple", |b| {
        b.iter(|| {
            black_box(sorter.sort_classes("p-4 m-2 bg-red-500"));
        })
    });

    // Medium complexity
    group.bench_function("medium", |b| {
        b.iter(|| {
            black_box(sorter.sort_classes("flex items-center justify-center p-4 m-2 bg-red-500 text-white rounded-lg shadow-md"));
        })
    });

    // High complexity
    group.bench_function("complex", |b| {
        b.iter(|| {
            black_box(sorter.sort_classes("inline-flex items-center justify-center font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none ring-offset-background px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_sorter_performance, bench_class_complexity);
criterion_main!(benches);
