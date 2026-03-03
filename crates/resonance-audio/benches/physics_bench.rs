use criterion::{criterion_group, criterion_main, Criterion};
use resonance_audio::physics::PhysicsGrid;

fn criterion_benchmark(c: &mut Criterion) {
    let mut grid = PhysicsGrid::new(200, 200);
    grid.pluck(100, 100, 1.0);
    c.bench_function("step 200x200", |b| b.iter(|| grid.step()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
