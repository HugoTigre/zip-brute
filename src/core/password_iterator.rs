use std::cmp::{min};
use std::error::Error;

#[derive(Debug)]
pub struct PasswordGenerator<'a> {
    chars: &'a [char],
    max_size: usize,
    size: usize,
    indices: Vec<usize>,
}

impl<'a> PasswordGenerator<'a> {
    pub fn new(chars: &'a [char], min_size: usize, max_size: usize) -> Result<Self, Box<dyn Error>> {
        if min_size == 0 {
            return Err("Min size needs to be bigger than 0.".into());
        }
        if min_size > max_size {
            return Err("Min size cannot be bigger than Max size.".into());
        }
        if chars.is_empty() {
            return Err("Charset cannot be empty.".into());
        }

        Ok(
            PasswordGenerator {
                chars,
                max_size,
                size: min_size,
                indices: vec![0; max_size],
            }
        )
    }

    fn password_length_iterated(&mut self) -> bool {
        let mut i = self.size - 1;
        let chars_len = self.chars.len() - 1;

        while self.indices[i] == chars_len {
            if i == 0 { return true; }
            i -= 1;
        }

        false
    }

    ///
    /// Increments indices to generate next password.
    ///
    /// example:
    /// in a 3 char set and 3 length pass
    /// [0,0,0] -> [0,0,1] -> [0,0,2] -> [0,1,0] -> [0,1,1] -> [0,1,2] -> [0,2,2] -> [1,0,0] -> ...
    ///
    fn increment_indices(&mut self) {
        for i in (0..self.size).rev() {
            if self.indices[i] < self.chars.len() - 1 { // ex: from [0,0] to [0,1]
                self.indices[i] += 1;
                break;
            } else if self.indices[i - 1] < self.indices[i] { // ex: from [0,1] to [1,0]
                self.indices[i] = 0;
                self.indices[i - 1] += 1;
                break;
            } else if self.indices[i - 1] == self.indices[i] { // ex: from [0,1,1] to [1,0,0]
                self.indices[i] = 0;
                continue;
            }
        }
    }
}

impl<'a> Iterator for PasswordGenerator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > self.max_size { return None; }

        // build current password iteration
        let mut pass = String::with_capacity(self.size);
        for i in 0..self.size {
            pass.push(self.chars[self.indices[i]]);
        }

        if self.password_length_iterated() {
            // increment password length and reset indices
            self.size += 1;
            for j in 0..min(self.size, self.indices.len() - 1) {
                self.indices[j] = 0;
            }
            return Some(pass);
        }

        // increment indices
        self.increment_indices();

        Some(pass)
    }
}

