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
    let pattern = vec![1, 2, 3, 4, 5];
    let repeats = size / pattern.len();
    pattern
        .iter()
        .cycle()
        .take(repeats * pattern.len())
        .cloned()
        .collect()
}

fn benchmark_quick_sort(c: &mut Criterion) {
    let mut data = vec![5, 4, 3, 2, 1];
    c.bench_function("quick sort 5", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = generate_test_data(100);
    c.bench_function("quick sort 100", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = generate_test_data(1000);
    c.bench_function("quick sort 1000", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = generate_test_data(10000);
    c.bench_function("quick sort 10000", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = generate_test_data(1000000);
    c.bench_function("quick sort 1000000", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = inversely_sorted(1000);
    c.bench_function("quick sort inversely sorted 1000", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = few_unique_elements(1000);
    c.bench_function("quick sort few unique 1000", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });

    let mut data = repeating_patterns(1000);
    c.bench_function("quick sort repeating patterns 1000", |b| {
        b.iter(|| black_box(&mut data).quick_sort())
    });
}

fn benchmark_merge_sort(c: &mut Criterion) {
    let mut data = vec![5, 4, 3, 2, 1];
    c.bench_function("merge sort 5", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = generate_test_data(100);
    c.bench_function("merge sort 100", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = generate_test_data(1000);
    c.bench_function("merge sort 1000", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = generate_test_data(10000);
    c.bench_function("merge sort 10000", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = generate_test_data(1000000);
    c.bench_function("merge sort 1000000", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = inversely_sorted(1000);
    c.bench_function("merge sort inversely sorted 1000", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = few_unique_elements(1000);
    c.bench_function("merge sort few unique 1000", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });

    let mut data = repeating_patterns(1000);
    c.bench_function("merge sort repeating patterns 1000", |b| {
        b.iter(|| black_box(&mut data).merge_sort())
    });
}

fn quicksort_config() -> Criterion {
    let path = Path::new("results/quicksort");
    Criterion::default()
        .sample_size(60) // adjust as needed
        .output_directory(path) // output directory for quicksort results
}

fn mergesort_config() -> Criterion {
    let path = Path::new("results/mergesort");
    Criterion::default()
        .sample_size(60) // adjust as needed
        .output_directory(path) // output directory for mergesort results
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

criterion_main!(quicksort_benches, mergesort_benches);
