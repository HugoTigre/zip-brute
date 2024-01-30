extern crate core;

use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::path::Path;

use clap::{Arg, ArgGroup, Command, crate_authors, crate_description, crate_name, crate_version};

#[derive(Debug)]
pub struct Args {
    pub zip_path: String,
    pub dictionary_file: Option<String>,
    pub charset: Vec<char>,
    pub charset_min_len: Option<usize>,
    pub charset_max_len: Option<usize>,
    pub concurrency: usize,
    pub progress: bool,
    pub zip_file_index: usize,
}

pub fn command() -> clap::Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand_required(true)
        .arg(
            Arg::new("zip_path")
                .help("Path to zip file")
                .long("zip-path")
                .short('z')
                .num_args(1)
                .required(true)
                .value_parser(validate_file_arg),
        )
        .arg(
            Arg::new("concurrency")
                .help("Level of parallelism. Default is nr of system cores - 1.")
                .long("concurrency")
                .short('x')
                .num_args(1)
                .required(false)
                .value_parser(validate_concurrency_arg),
        )
        .arg(
            Arg::new("progress")
                .help("Show/hide the progress bar. Default  is true.")
                .long("progress")
                .short('p')
                .num_args(1)
                .required(false)
                .value_parser(validate_progress_arg),
        )
        .arg(
            Arg::new("zip_file_index")
                .help("Index/position of a encrypted file inside the zip archive. Default is 0.")
                .long("zip-file-index")
                .short('i')
                .num_args(1)
                .required(false)
                .value_parser(validate_zip_file_index_arg),
        )
        .subcommand(
            Command::new("strategy")
                .about("Choose the strategy to find the password. Dictionary file or Custom password generation")
                .arg(
                    Arg::new("dictionary_file")
                        .help("Path to dictionary file")
                        .long("dictionary-file")
                        .short('d')
                        .num_args(1)
                        .required(false)
                        .value_parser(validate_file_arg),
                )
                .arg(
                    Arg::new("charset")
                        .help("List of possible characters to build the password")
                        .long("charset")
                        .short('c')
                        .num_args(1)
                        .required(false)
                        .value_parser(validate_charset_arg),
                )
                .arg(
                    Arg::new("charset_len")
                        .help("Variable length limits of the password. Ex: 1-10
Only applies to password generation strategy")
                        .long("charset-len")
                        .short('l')
                        .num_args(1)
                        .required(false)
                        .value_parser(validate_charset_len),
                )
                .group(
                    ArgGroup::new("strategy")
                        .required(true)
                        .args(["dictionary_file", "charset"]),
                )
        )
}

pub fn build_args() -> Result<Args, Box<dyn Error>> {
    let command = command();
    let matches = command.get_matches();

    let zip_file: &String = matches.get_one("zip_path").expect("validated in args parser");

    let sub_matches = matches.subcommand_matches("strategy").expect("validated in args parser");

    let dic_file: Option<&String> = sub_matches.try_get_one("dictionary_file").expect("validated in args parser");

    let charset: Option<&String> = sub_matches.try_get_one("charset").expect("validated in args parser");
    let charset_vec = charset.unwrap_or(&"".to_string()).chars().collect();

    // if Strategy == Password Generation then charset length is required
    let mut charset_min_len = None;
    let mut charset_max_len = None;
    if dic_file.is_none() {
        let charset_len: Option<&String> = sub_matches.try_get_one("charset_len").expect("validated in args parser");
        if charset_len.is_some() {
            return Err("Charset length is required for password generation strategy.".into());
        } else {
            let (min_len, max_len) = charset_len.unwrap().split_once('-').expect("validated in args parser");
            charset_min_len = Some(min_len.parse::<usize>().expect("validated in args parser"));
            charset_max_len = Some(max_len.parse::<usize>().expect("validated in args parser"));
        }
    }

    let concurrency_arg: Option<&usize> = matches.try_get_one("concurrency").expect("validated in args parser");
    let max_parallelism = num_cpus::get_physical() - 1;
    let concurrency = concurrency_arg.unwrap_or_else(|| max(&1, &max_parallelism));

    let progress: Option<&bool> = matches.try_get_one("progress").expect("validated in args parser");

    let zip_file_index: Option<&usize> = matches.try_get_one("zip_file_index").expect("validated in args parser");

    Ok(Args {
        zip_path: zip_file.clone(),
        dictionary_file: dic_file.cloned(),
        charset: charset_vec,
        charset_min_len,
        charset_max_len,
        concurrency: *concurrency,
        progress: *progress.unwrap_or(&true),
        zip_file_index: *zip_file_index.unwrap_or(&0),
    })
}

///
/// Checks if file exists and have access to it.
///
fn validate_file_arg(path: &str) -> Result<String, String> {
    let sanitized_path = sanitize_path(path);

    if !Path::new(&sanitized_path).is_file() {
        Err("File not found.".to_string())
    } else {
        Ok(path.to_string())
    }
}

///
/// To support both '/' and '\' directory delimiters
///
fn sanitize_path(path: &str) -> String {
    if env::consts::OS.eq("windows") {
        path.replace('/', "\\")
    } else {
        path.replace('\\', "/")
    }
}

///
/// Checks if charset contains duplicated characters
///
fn validate_charset_arg(charset: &str) -> Result<String, String> {
    let chars = charset.chars();
    let mut found_duplicate = false;
    let mut duplicate_char: char = '0';
    let mut char_set: HashSet<char> = HashSet::new();

    for c in chars {
        if char_set.contains(&c) {
            found_duplicate = true;
            duplicate_char = c;
            break;
        } else {
            char_set.insert(c);
        }
    }

    if found_duplicate {
        println!("{:?}", duplicate_char);
        Err(format!("Charset cannot contains duplicate characters. Remove duplication of char '{duplicate_char}'"))
    } else {
        Ok(charset.to_string())
    }
}

///
/// Checks if charset min and mac length are valid
/// todo: max len cannot be smaller than charset length
///
fn validate_charset_len(charset_len: &str) -> Result<String, String> {
    let min_and_max_len = charset_len.split_once('-');
    match min_and_max_len {
        Some((min, max)) => {
            let min_val = &min.parse::<usize>();
            let max_val = &max.parse::<usize>();
            if min_val.is_err() || max_val.is_err() {
                Err("Charset length format is invalid. A valid format would be for example: 1-10".into())
            } else {
                let min_nr = min_val.as_ref().unwrap();
                let max_nr = max_val.as_ref().unwrap();
                if min_nr > max_nr || min_nr.eq(&0) || max_nr.eq(&0) {
                    Err("Charset length is invalid. Length cannot be 0 and Min length cannot be bigger than Max length".into())
                } else {
                    Ok(charset_len.to_string())
                }
            }
        }
        None => Err("Charset length format is invalid. A valid format would be for example: 1-10".into())
    }
}

fn validate_concurrency_arg(concurrency: &str) -> Result<usize, String> {
    match concurrency.parse::<usize>() {
        Ok(concurrency) => {
            let max_parallelism = num_cpus::get_physical();
            if concurrency > max_parallelism {
                Err(format!("Concurrency must not be higher than this computer number of cores {max_parallelism}"))
            } else {
                Ok(concurrency)
            }
        }
        Err(_) => Err("Concurrency must be a positive number".to_string()),
    }
}

fn validate_progress_arg(progress: &str) -> Result<bool, String> {
    let value = progress.to_lowercase();
    if value.eq("true") {
        Ok(true)
    } else if value.eq("false") {
        Ok(false)
    } else {
        Err("Progress must be true or false.".to_string())
    }
}

fn validate_zip_file_index_arg(zip_file_index: &str) -> Result<usize, String> {
    match zip_file_index.parse::<usize>() {
        Ok(zip_file_index) => Ok(zip_file_index),
        Err(_) => Err("Zip file index must be a positive number".to_string()),
    }
}
