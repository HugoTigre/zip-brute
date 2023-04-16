use indicatif::ProgressBar;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::core::password_iterator;
use crate::core::utils::decrypt;
use crate::models::strategy::Strategy;

pub fn process(
    data: &Strategy,
    progress_bar: &Option<ProgressBar>,
) -> Option<String> {
    let combinations_total = data.count_combinations();
    if let Some(progress_bar) = progress_bar {
        progress_bar.set_length(combinations_total as u64);
    }

    let charset = data.charset().unwrap();
    let pass_gen_iter = password_iterator::PasswordGenerator::new(
        &charset.charset,
        charset.min_len,
        charset.max_len,
    );

    let zip = data.zip_data();

    pass_gen_iter.unwrap()
        .par_bridge()
        .find_map_any(|pass|
            match decrypt(&pass, &zip.data, progress_bar) {
                Some(..) => Some(pass),
                None => None
            }
        )
}
