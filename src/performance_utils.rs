use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metrics tracking for identifying bottlenecks
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub parse_time: Duration,
    pub sort_time: Duration,
    pub format_time: Duration,
    pub total_time: Duration,
    pub file_size: usize,
    pub class_count: usize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            parse_time: Duration::default(),
            sort_time: Duration::default(),
            format_time: Duration::default(),
            total_time: Duration::default(),
            file_size: 0,
            class_count: 0,
        }
    }

    pub fn classes_per_second(&self) -> f64 {
        if self.total_time.as_secs_f64() > 0.0 {
            self.class_count as f64 / self.total_time.as_secs_f64()
        } else {
            0.0
        }
    }

    pub fn bytes_per_second(&self) -> f64 {
        if self.total_time.as_secs_f64() > 0.0 {
            self.file_size as f64 / self.total_time.as_secs_f64()
        } else {
            0.0
        }
    }
}

/// Performance profiler for tracking execution time of different operations
pub struct PerformanceProfiler {
    start_time: Instant,
    timings: HashMap<String, Duration>,
    current_operation: Option<(String, Instant)>,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            timings: HashMap::new(),
            current_operation: None,
        }
    }

    pub fn start_operation(&mut self, name: &str) {
        if let Some((prev_name, prev_start)) = self.current_operation.take() {
            // Finish previous operation
            let duration = prev_start.elapsed();
            self.timings.insert(prev_name, duration);
        }

        self.current_operation = Some((name.to_string(), Instant::now()));
    }

    pub fn finish_operation(&mut self) {
        if let Some((name, start)) = self.current_operation.take() {
            let duration = start.elapsed();
            self.timings.insert(name, duration);
        }
    }

    pub fn get_total_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn get_operation_time(&self, name: &str) -> Option<Duration> {
        self.timings.get(name).copied()
    }

    pub fn get_all_timings(&self) -> &HashMap<String, Duration> {
        &self.timings
    }

    pub fn print_summary(&self) {
        println!("Performance Summary:");
        println!("==================");

        let total = self.get_total_time();
        println!("Total time: {:.2}ms", total.as_secs_f64() * 1000.0);

        let mut sorted_timings: Vec<_> = self.timings.iter().collect();
        sorted_timings.sort_by(|a, b| b.1.cmp(a.1));

        for (name, duration) in sorted_timings {
            let percentage = (duration.as_secs_f64() / total.as_secs_f64()) * 100.0;
            println!(
                "  {}: {:.2}ms ({:.1}%)",
                name,
                duration.as_secs_f64() * 1000.0,
                percentage
            );
        }
    }
}

/// Memory usage tracking for identifying memory bottlenecks
#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub peak_memory_usage: usize,
    pub allocations: usize,
    pub deallocations: usize,
    pub current_memory: usize,
}

impl MemoryMetrics {
    pub fn new() -> Self {
        Self {
            peak_memory_usage: 0,
            allocations: 0,
            deallocations: 0,
            current_memory: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) {
        self.allocations += 1;
        self.current_memory += size;
        if self.current_memory > self.peak_memory_usage {
            self.peak_memory_usage = self.current_memory;
        }
    }

    pub fn deallocate(&mut self, size: usize) {
        self.deallocations += 1;
        self.current_memory = self.current_memory.saturating_sub(size);
    }

    pub fn memory_efficiency(&self) -> f64 {
        if self.allocations > 0 {
            self.deallocations as f64 / self.allocations as f64
        } else {
            0.0
        }
    }
}

#[cfg(feature = "performance-profiling")]
#[macro_export]
macro_rules! profile_operation {
    ($profiler:expr, $operation:expr, $code:block) => {
        $profiler.start_operation($operation);
        let result = $code;
        $profiler.finish_operation();
        result
    };
}

#[cfg(not(feature = "performance-profiling"))]
#[macro_export]
macro_rules! profile_operation {
    ($profiler:expr, $operation:expr, $code:block) => {
        $code
    };
}
