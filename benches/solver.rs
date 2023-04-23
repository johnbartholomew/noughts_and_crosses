use criterion::{black_box, criterion_group, criterion_main, Criterion};
use noughts_and_crosses as xoxo;
use std::time::Duration;

fn solver_benchmark(c: &mut Criterion) {
    c.bench_function("solve (new Board)", |b| {
        b.iter(|| {
            let mut games = 0;
            let result = xoxo::solve(xoxo::Board::new(), &mut games);
            black_box(result); // Don't optimise out the whole thing please.
            black_box(games);
        })
    });
}

criterion_group! {
    name = benches;
    // On my machine the default warm-up isn't long enough; Criterion prints a warning about it.
    config = Criterion::default().warm_up_time(Duration::from_secs(10)).measurement_time(Duration::from_secs(10));
    targets = solver_benchmark
}
criterion_main!(benches);
