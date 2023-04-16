extern crate core;

use std::path::Path;
use std::process;
use std::time::Duration;

use indicatif::{HumanCount, HumanDuration, ProgressBar};
use log::{error, info};

use zip_brute::core::args::{Args, build_args};
use zip_brute::core::utils::{build_progress_bar, set_up_logging, validate_zip};
use zip_brute::models::charset::Charset;
use zip_brute::models::strategy::Strategy::{DictionaryFile, PasswordGen};
use zip_brute::models::zip_data::ZipData;

fn main() {
    // Measures the total runtime
    let start = std::time::Instant::now();

    set_up_logging();

    let Args {
        zip_path,
        dictionary_file,
        charset,
        charset_min_len,
        charset_max_len,
        concurrency,
        progress,
        zip_file_index,
    } = build_args().unwrap_or_else(|err| {
        error!("Application error: {err}");
        process::exit(1);
    });

    if let Err(err) = validate_zip(&zip_path, &zip_file_index) {
        error!("Zip file is not valid: {err}");
        process::exit(1);
    }

    // Reading the ZIP file into RAM
    let zip_file = std::fs::read(&zip_path)
        .unwrap_or_else(|err| {
            error!("Application error: {err}");
            process::exit(1);
        });

    // build zip data
    let zip_data = ZipData {
        path: zip_path,
        data: zip_file,
        file_index: zip_file_index,
    };

    let strategy = match dictionary_file {
        Some(dic_path) => {
            let path = Path::new(&dic_path);
            DictionaryFile {
                zip: zip_data,
                dict_path: path.to_path_buf(),
            }
        }
        None => {
            PasswordGen {
                zip: zip_data,
                charset: Charset {
                    charset,
                    min_len: charset_min_len.unwrap(), // validated in args
                    max_len: charset_max_len.unwrap(), // validated in args
                },
            }
        }
    };

    info!("Run parameters: concurrency = {}, strategy = {}", concurrency, strategy);

    // Create thread pool to use with rayon
    let pool = rayon::ThreadPoolBuilder::new().num_threads(concurrency).build().unwrap();

    info!("Using {} threads", pool.current_num_threads());
    info!("Note: ETA accounts for all possible password combinations.");

    let progress_bar = if progress { Some(build_progress_bar()) } else { None };

    let result = pool.install(||
        strategy.process(&progress_bar)
    );

    let stop = start.elapsed();

    output_results_and_statistics(&progress_bar, result, stop);
}

fn output_results_and_statistics(progress_bar: &Option<ProgressBar>, result: Option<String>, stop: Duration) {
    if let Some(progress_bar) = progress_bar {
        let speed = HumanCount(((progress_bar.position() as f64) / stop.as_secs_f64()) as u64);
        progress_bar.finish();
        info!("Average speed: {speed} passwords/second");
    }
    info!("Total duration: {}", HumanDuration(stop));
    match result {
        None => { info!("Sorry but the password was not found with the given parameters."); }
        Some(psw) => { info!("Password found: {psw}"); }
    }
}
