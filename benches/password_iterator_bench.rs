use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zip_brute::core::password_iterator::{PasswordGenerator};

fn iterate_passwords(c: &mut Criterion) {
    c.bench_function("iterate_passwords", |b| {
        // 19_173_960 combinations/iterations
        let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', '*'];
        let min_size = 1;
        let max_size = 8;

        b.iter(|| {
            let iterator = PasswordGenerator::new(&chars, min_size, max_size).unwrap();

            black_box(iterator.last());
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = iterate_passwords
}
criterion_main!(benches);
