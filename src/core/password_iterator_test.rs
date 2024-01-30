#[cfg(test)]
mod tests {
    use crate::core::password_iterator;
    use crate::core::utils::count_combinations;
    use crate::models::charset::Charset;

    #[test]
    fn should_not_allow_empty_charset() {
        let chars = vec![];
        let iter =
            password_iterator::PasswordGenerator::new(&chars, 1, 2);
        assert!(iter.is_err());
        assert_eq!(iter.err().unwrap().to_string(), "Charset cannot be empty.");
    }

    #[test]
    fn should_not_allow_min_size_0() {
        let chars = vec!['a'];
        let iter =
            password_iterator::PasswordGenerator::new(&chars, 0, 2);
        assert!(iter.is_err());
        assert_eq!(iter.err().unwrap().to_string(), "Min size needs to be bigger than 0.");
    }

    #[test]
    fn should_not_allow_min_size_bigger_than_max_size() {
        let chars = vec!['a'];
        let iter =
            password_iterator::PasswordGenerator::new(&chars, 3, 2);
        assert!(iter.is_err());
        assert_eq!(iter.err().unwrap().to_string(), "Min size cannot be bigger than Max size.");
    }

    #[test]
    fn should_iterate_passwords_with_max_size_2() {
        let chars = vec!['1', 'b', 'c'];
        let mut iter =
            password_iterator::PasswordGenerator::new(&chars, 1, 2).unwrap();
        assert_eq!(iter.next(), Some("1".into()));
        assert_eq!(iter.next(), Some("b".into()));
        assert_eq!(iter.next(), Some("c".into()));
        assert_eq!(iter.next(), Some("11".into()));
        assert_eq!(iter.next(), Some("1b".into()));
        assert_eq!(iter.next(), Some("1c".into()));
        assert_eq!(iter.next(), Some("b1".into()));
        assert_eq!(iter.next(), Some("bb".into()));
        assert_eq!(iter.next(), Some("bc".into()));
        assert_eq!(iter.next(), Some("c1".into()));
        assert_eq!(iter.next(), Some("cb".into()));
        assert_eq!(iter.next(), Some("cc".into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_iterate_passwords_with_min_and_max_size_2() {
        let chars = vec!['-', 'b', 'c'];
        let mut iter =
            password_iterator::PasswordGenerator::new(&chars, 2, 2).unwrap();
        assert_eq!(iter.next(), Some("--".into()));
        assert_eq!(iter.next(), Some("-b".into()));
        assert_eq!(iter.next(), Some("-c".into()));
        assert_eq!(iter.next(), Some("b-".into()));
        assert_eq!(iter.next(), Some("bb".into()));
        assert_eq!(iter.next(), Some("bc".into()));
        assert_eq!(iter.next(), Some("c-".into()));
        assert_eq!(iter.next(), Some("cb".into()));
        assert_eq!(iter.next(), Some("cc".into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_support_max_size_bigger_than_char_length() {
        let chars = vec!['a'];
        let mut iter =
            password_iterator::PasswordGenerator::new(&chars, 1, 3).unwrap();
        assert_eq!(iter.next(), Some("a".into()));
        assert_eq!(iter.next(), Some("aa".into()));
        assert_eq!(iter.next(), Some("aaa".into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_support_mina_and_max_size_bigger_than_char_length() {
        let chars = vec!['a'];
        let mut iter =
            password_iterator::PasswordGenerator::new(&chars, 2, 3).unwrap();
        assert_eq!(iter.next(), Some("aa".into()));
        assert_eq!(iter.next(), Some("aaa".into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_support_special_chars() {
        let chars = vec![' ', '\\'];
        let mut iter =
            password_iterator::PasswordGenerator::new(&chars, 1, 2).unwrap();
        assert_eq!(iter.next(), Some(" ".into()));
        assert_eq!(iter.next(), Some("\\".into()));
        assert_eq!(iter.next(), Some("  ".into()));
        assert_eq!(iter.next(), Some(" \\".into()));
        assert_eq!(iter.next(), Some("\\ ".into()));
        assert_eq!(iter.next(), Some("\\\\".into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn should_iterate_the_correct_nr_of_password() {
        let chars = ['a', 'b', 'c', 'd'];
        let min_length = [1, 2, 3, 4];
        let max_length = [1, 2, 3, 4];

        min_length.iter().for_each(|&min_length| {
            max_length.iter().filter(|length| length >= &&min_length).for_each(|&max_length| {
                let iter =
                    password_iterator::PasswordGenerator::new(&chars, min_length, max_length).unwrap();

                let charset = Charset { charset: chars.to_vec(), min_len: min_length, max_len: max_length };

                let expected_nr_of_passwords = count_combinations(&charset);
                let nr_of_passwords = iter.count();

                assert_eq!(nr_of_passwords, expected_nr_of_passwords);
            });
        });
    }
}
