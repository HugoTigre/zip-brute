use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use indicatif::ProgressBar;
use crate::core::processors::{dictionary_processor, password_gen_processor};
use crate::core::utils::count_combinations;
use crate::models::charset::Charset;
use crate::models::strategy::Strategy;
use crate::models::zip_data::ZipData;

impl Strategy {
    pub fn process(&self, progress_bar: &Option<ProgressBar>) -> Option<String> {
        match self {
            Strategy::DictionaryFile { .. } => dictionary_processor::process(self, progress_bar),
            Strategy::PasswordGen { .. } => password_gen_processor::process(self, progress_bar),
        }
    }

    pub fn charset(&self) -> Option<&Charset> {
        match self {
            Strategy::DictionaryFile { .. } => None,
            Strategy::PasswordGen { charset, .. } => Some(charset),
        }
    }

    pub fn zip_data(&self) -> &ZipData {
        match self {
            Strategy::DictionaryFile { zip, .. } => zip,
            Strategy::PasswordGen { zip, .. } => zip,
        }
    }

    pub fn dict_path(&self) -> Option<&PathBuf> {
        match self {
            Strategy::DictionaryFile { dict_path, .. } => Some(dict_path),
            Strategy::PasswordGen { .. } => None,
        }
    }

    pub fn count_combinations(&self) -> usize {
        match self {
            Strategy::DictionaryFile { dict_path, .. } => {
                let file = File::open(dict_path).expect("Unable to open file");
                let mut reader = BufReader::new(file);
                let mut total_password_count = 0;
                let mut line_buffer = Vec::new();
                loop {
                    let res = reader
                        .read_until(b'\n', &mut line_buffer)
                        .expect("Unable to read file");

                    if res == 0 { break; }  // end of file

                    line_buffer.clear();
                    total_password_count += 1;
                }
                total_password_count
            }

            Strategy::PasswordGen { charset, .. } =>
                count_combinations(charset),
        }
    }
}
