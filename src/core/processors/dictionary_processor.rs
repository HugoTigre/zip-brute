use indicatif::ProgressBar;
use log::{error, info};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::core::utils::decrypt;
use crate::models::strategy::Strategy;

pub fn process(
    data: &Strategy,
    progress_bar: &Option<ProgressBar>,
) -> Option<String> {
    if let Some(progress_bar) = progress_bar {
        progress_bar.println("Reading dictionary file...");
    } else {
        info!("Reading dictionary file...");
    }

    let zip = data.zip_data();
    let dict_path = data.dict_path().unwrap();

    let dict_string = match std::fs::read_to_string(dict_path) {
        Ok(file) => file,
        Err(err) => {
            error!("Error opening dictionary file: {err}.");
            return None;
        }
    };
    let dict: Vec<&str> = dict_string.lines().collect();

    // set total length of progress bar
    if let Some(progress_bar) = progress_bar {
        let combinations_total = dict.len();
        progress_bar.set_length(combinations_total as u64);
    }

    let result = dict
        .par_iter()
        .find_map_any(|pass| {
            decrypt(pass, &zip.data, progress_bar)
        });

    result.map_or_else(|| None, |r| Some(r.to_string()))
}
