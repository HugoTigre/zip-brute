#[cfg(test)]
mod tests {
    use crate::core::utils;
    use crate::core::utils::build_progress_bar;
    use crate::models::charset::Charset;

    #[test]
    fn should_determine_nr_of_password_combinations_variable_length() {
        let charset = Charset { charset: vec!['a', 'b'], min_len: 1, max_len: 2 };
        let count = utils::count_combinations(&charset);
        assert_eq!(count, 6);

        let charset = Charset { charset: vec!['a', 'b', 'A', 'B', '1', '*'], min_len: 1, max_len: 2 };
        let count = utils::count_combinations(&charset);
        assert_eq!(count, 42);

        let charset = Charset { charset: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', '*'], min_len: 1, max_len: 7 };
        let count = utils::count_combinations(&charset);
        assert_eq!(count, 2_396_744);

        let charset = Charset { charset: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', '*'], min_len: 1, max_len: 10 };
        let count = utils::count_combinations(&charset);
        assert_eq!(count, 1_227_133_512);
    }

    #[test]
    fn should_determine_nr_of_password_combinations_single_length() {
        let charset = Charset { charset: vec!['a', 'b'], min_len: 2, max_len: 2 };
        let count = utils::count_combinations(&charset);
        assert_eq!(count, 4);
    }

    #[test]
    fn should_ignore_false_positives() {
        let password = "12121".to_string(); // false positive
        let zip_file = "tests/resources/zipbrute.zip";
        let zip_file_vec = std::fs::read(zip_file).expect("Zip file not found");

        let pb = build_progress_bar();
        pb.set_length(1);

        let result = utils::decrypt(&password, &zip_file_vec, &Some(pb));

        assert!(result.is_none());
    }

    #[test]
    fn should_open_zip_file_with_correct_password() {
        let password = "123123".to_string(); // correct password
        let zip_file = "tests/resources/zipbrute.zip";
        let zip_file_vec = std::fs::read(zip_file).expect("Zip file not found");

        let pb = build_progress_bar();
        pb.set_length(1);

        let result = utils::decrypt(&password, &zip_file_vec, &Some(pb));

        assert!(result.is_some());
    }
}
