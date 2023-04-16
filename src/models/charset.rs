#[derive(Debug)]
pub struct Charset {
    pub charset: Vec<char>,
    pub min_len: usize,
    pub max_len: usize,
}