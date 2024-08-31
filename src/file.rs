pub struct FileSize {
    pub path: String,
    pub size: usize,
    pub depth: usize,
    pub is_dir: bool
}

impl FileSize {
    pub fn new(path: String, size: usize, depth: usize, is_dir: bool) -> Self {
        FileSize { path, size, depth, is_dir}
    }
}
