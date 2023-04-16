use std::path::Path;

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use zip_brute::core::utils::build_progress_bar;
use zip_brute::models::strategy::Strategy::DictionaryFile;
use zip_brute::models::zip_data::ZipData;

fn dictionary_processor(c: &mut Criterion) {
    c.bench_function("dictionary_processor", |b| {
        let zip_path = "tests/resources/zipbrute.zip".to_string();
        let dictionary_file = Path::new("tests/resources/xato-net-10-million-passwords.txt");

        b.iter(|| {
            let progress_bar = build_progress_bar();

            let zip_file = std::fs::read(&zip_path).expect("Zip file not found");

            let zip_data = ZipData {
                path: zip_path.clone(),
                data: zip_file,
                file_index: 0,
            };

            let strategy = DictionaryFile{
                zip:zip_data,
                dict_path: dictionary_file.to_path_buf(),
            };

            black_box(
                strategy.process(&Some(progress_bar))
            );
        });
    });
}

criterion_group! {
    name = benches;
    // config = Criterion::default().sample_size(10).measurement_time(Duration::from_secs(100));
    config = Criterion::default().sample_size(10);
    targets = dictionary_processor
}
criterion_main!(benches);
