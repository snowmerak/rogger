use std::path::Path;


pub struct RotationWriter {
    path: Box<Path>,
    base_filename: String,
    max_index: usize,

    current_file: Option<std::fs::File>,
    current_index: usize,
}

impl RotationWriter {
    pub fn new(path: &Path, base_name: String, max_index: usize) -> Self {
        RotationWriter {
            path: path.into(),
            base_filename: base_name,
            max_index,
            current_file: None,
            current_index: 0,
        }
    }
}
