#[derive(Debug)]
pub struct ZipData {
    pub path: String,
    pub data: Vec<u8>,
    pub file_index: usize,
}

impl std::fmt::Display for ZipData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ZipData ( path: {}, file_index: {}))", self.path, self.file_index)
    }
}
