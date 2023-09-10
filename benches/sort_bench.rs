use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferric_sort::file::SortVecI64;
use rand::prelude::*;

pub fn generate_test_data(size: usize) -> Vec<i64> {
    let mut vec = Vec::new();

    for _ in 0..size {
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..10000);
        vec.push(num);
    }

    vec
}

fn inversely_sorted(size: usize) -> Vec<i64> {
    (0..size as i64).rev().collect()
}

fn few_unique_elements(size: usize) -> Vec<i64> {
    let mut vec = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        vec.push(rng.gen_range(0..5)); // Only 5 unique numbers
    }
    vec
}

fn repeating_patterns(size: usize) -> Vec<i64> {
    let pattern = [1, 2, 3, 4, 5];
    let repeats = size / pattern.len();
    pattern
        .iter()
        .cycle()
        .take(repeats * pattern.len())
        .cloned()
        .collect()
}

macro_rules! benchmark_sort {
    ($c:expr, $func:ident, $name:expr) => {
        let sizes = [5, 100, 1000, 10000, 1000000, 100000000];
        let mut data: Vec<i64>;

        for &size in &sizes {
            data = generate_test_data(size);
            let benchmark_name = format!("{} random {}", $name, size);
            $c.bench_function(&benchmark_name, |b| b.iter(|| black_box(&mut data).$func()));

            data = inversely_sorted(size);
            let benchmark_name = format!("{} inversely sorted {}", $name, size);
            $c.bench_function(&benchmark_name, |b| b.iter(|| black_box(&mut data).$func()));

            data = few_unique_elements(size);
            let benchmark_name = format!("{} few unique {}", $name, size);
            $c.bench_function(&benchmark_name, |b| b.iter(|| black_box(&mut data).$func()));

            data = repeating_patterns(size);
            let benchmark_name = format!("{} repeating patterns {}", $name, size);
            $c.bench_function(&benchmark_name, |b| b.iter(|| black_box(&mut data).$func()));
        }
    };
}

fn benchmark_standard_sort(c: &mut Criterion) {
    benchmark_sort!(c, sort, "standard sort");
}

fn benchmark_quick_sort(c: &mut Criterion) {
    benchmark_sort!(c, quick_sort, "quick sort");
}

fn benchmark_merge_sort(c: &mut Criterion) {
    benchmark_sort!(c, merge_sort, "merge sort");
}

fn standard_sort() -> Criterion {
    let path = Path::new("results/standard_sort");
    Criterion::default()
        .sample_size(10) // adjust as needed
        .output_directory(path) // output directory for standard sort results
}

fn quicksort_config() -> Criterion {
    let path = Path::new("results/quicksort");
    Criterion::default()
        .sample_size(10) // adjust as needed
        .output_directory(path) // output directory for quicksort results
}

fn mergesort_config() -> Criterion {
    let path = Path::new("results/mergesort");
    Criterion::default()
        .sample_size(10) // adjust as needed
        .output_directory(path) // output directory for mergesort results
}

criterion_group! {
    name = standard_sort_benches;
    config = standard_sort();
    targets = benchmark_standard_sort  // Your standard sort benchmark function
}

criterion_group! {
    name = quicksort_benches;
    config = quicksort_config();
    targets = benchmark_quick_sort  // Your quicksort benchmark function
}

criterion_group! {
    name = mergesort_benches;
    config = mergesort_config();
    targets = benchmark_merge_sort  // Your mergesort benchmark function
}

criterion_main!(standard_sort_benches, quicksort_benches, mergesort_benches);
