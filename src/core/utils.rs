use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use zip::result::ZipError::UnsupportedArchive;
use zip::ZipArchive;

use crate::models::charset::Charset;

///
/// Determine the nr of password possibilities for a given charset and size
///
pub fn count_combinations(charset: &Charset) -> usize {
    let mut total_password_count = 0;
    for i in charset.min_len..=charset.max_len {
        total_password_count += charset.charset.len().pow(i as u32)
    }
    total_password_count
}

///
/// # Arguments
///
/// - `password`: The password to test
/// - `zip_file`: The zip file to test the password against
/// - `progress_bar`: The progress bar to be updated
/// - `show_progress`: If false progress bar is not drawn/shown
///
/// returns: Option<(usize, &str)>
///
pub fn decrypt<'a>(
    password: &'a str,
    zip_file: &[u8],
    progress_bar: &Option<ProgressBar>,
) -> Option<&'a str> {
    let cursor = Cursor::new(zip_file);
    let mut archive = ZipArchive::new(cursor).expect("Failed opening ZIP archive");
    let result = archive.by_index_decrypt(0, password.as_bytes());

    if let Some(progress_bar) = progress_bar {
        progress_bar.inc(1); // atomic: possible contention point
    }

    match result {
        Ok(Ok(mut zip)) => {
            let mut buffer = Vec::with_capacity(zip.size() as usize);
            match zip.read_to_end(&mut buffer) {
                Err(_) => {
                    // False positive in ZipCrypto algorithm (see [ZipArchive.by_index_decrypt] docs)
                    // todo: count false positives
                    None
                }
                Ok(_) => {
                    Some(password)
                }
            }
        }
        _ => None,
    }
}

///
/// Builds a new progress bar.
///
/// Applies style.
/// Refresh terminal once per second.
///
/// # Arguments
/// `total`: The total number of passwords to be tested
///
pub fn build_progress_bar() -> ProgressBar {
    // Create style
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {wide_bar:.cyan/blue} {percent}% {pos:>7}/{len:7} throughput:{per_sec} eta:{eta}")
        .expect("Failed to create progress style");

    // Create progress bar and apply style
    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(progress_style);

    // Refresh terminal once per second (helps with eta smoothing)
    let draw_target = ProgressDrawTarget::stdout_with_hz(1);
    progress_bar.set_draw_target(draw_target);
    progress_bar.enable_steady_tick(Duration::from_millis(1000));

    progress_bar
}

pub fn validate_zip(path: &String, file_index: &usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let zip_result = archive.by_index(*file_index);
    match zip_result {
        Ok(_) => Err("Zip file not encrypted".into()),
        Err(UnsupportedArchive(msg)) if msg == "Password required to decrypt file" => Ok(()),
        Err(e) => Err(format!("Failed to load zip archive: {e}").into()),
    }
}

pub fn set_up_logging() {
    if Path::new("logging.yaml").exists() {
        log4rs::init_file("logging.yaml", Default::default()).unwrap();
    } else {
        let stdout_appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}")))
            .build();

        let config = log4rs::Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout_appender)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();
    }
}
