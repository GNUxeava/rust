// import necessary functions from criterion
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

// import the fib functions
use test_and_bench::{fib_one, fib_two};

fn bench(c: &mut Criterion) {
    // Create a benchmark group
    let mut group = c.benchmark_group("Fibonacci");
    // For loop to put in inputs
    for i in [0u32, 10u32].iter() {
        // Add fib_one
        group.bench_with_input(
            BenchmarkId::new("Fib One" , i), i, 
            |b, i| b.iter(|| fib_one(*i))
            );
        // Add fib two
        group.bench_with_input(
            BenchmarkId::new("Fib Two", i), i,
            |b, i| b.iter(|| fib_two(*i)));
    }
    // Group is finished
    group.finish();
}

// Create criterion group
criterion_group!(fib_benches, bench);
// Add bench to criterion main
criterion_main!(fib_benches);
