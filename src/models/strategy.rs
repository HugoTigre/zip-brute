use std::path::PathBuf;

use crate::models::charset::Charset;
use crate::models::zip_data::ZipData;

pub enum Strategy {
    DictionaryFile {
        zip: ZipData,
        dict_path: PathBuf,
    },
    PasswordGen {
        zip: ZipData,
        charset: Charset,
    },
}

impl std::fmt::Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Strategy::DictionaryFile { zip, dict_path, .. } => {
                write!(f, "DictionaryFile ( {}, dict_path: {}))", zip, dict_path.display())
            }
            Strategy::PasswordGen { zip, charset, .. } => {
                write!(f, "PasswordGen ( {}, charset: {:?}))", zip, charset)
            }
        }
    }
}
