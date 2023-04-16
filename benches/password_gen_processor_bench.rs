use criterion::{black_box, Criterion, criterion_group, criterion_main};

use zip_brute::core::utils::build_progress_bar;
use zip_brute::models::charset::Charset;
use zip_brute::models::strategy::Strategy::PasswordGen;
use zip_brute::models::zip_data::ZipData;

fn password_gen_processor(c: &mut Criterion) {
    c.bench_function("password_gen_processor", |b| {
        let chars = vec!['a', '1', '2', '3'];
        let min_size = 1;
        let max_size = 6;

        let zip_path = "tests/resources/zipbrute.zip".to_string();

        b.iter(|| {
            let progress_bar = build_progress_bar();

            let zip_file = std::fs::read(&zip_path).expect("Zip file not found");

            let zip_data = ZipData {
                path: zip_path.clone(),
                data: zip_file,
                file_index: 0,
            };

            let strategy = PasswordGen {
                zip: zip_data,
                charset: Charset {
                    charset: chars.clone(),
                    min_len: min_size,
                    max_len: max_size,
                },
            };

            black_box(
                strategy.process(&Some(progress_bar))
            );
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = password_gen_processor
}
criterion_main!(benches);
