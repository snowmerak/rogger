use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RotationWriter {
    path: Box<Path>,
    base_filename: String,
    max_lines: usize,
    current_file: Option<std::fs::File>,
    current_lines: usize,
}

impl RotationWriter {
    pub fn new(path: &Path, base_filename: String, max_lines: usize) -> Self {
        RotationWriter {
            path: path.into(),
            base_filename,
            max_lines,
            current_file: None,
            current_lines: 0,
        }
    }

    pub fn write(&mut self, line: &str) -> std::io::Result<()> {
        if self.current_file.is_none() || self.current_lines >= self.max_lines {
            self.rotate()?;
        }
        if let Some(ref mut file) = self.current_file {
            writeln!(file, "{}", line)?;
            self.current_lines += 1;
        }
        Ok(())
    }

    fn rotate(&mut self) -> std::io::Result<()> {
        // Close current file if open
        self.current_file = None;

        // Generate timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Create new filename
        let filename = format!("{}.{}", self.base_filename, timestamp);
        let filepath = self.path.join(filename);

        // Open new file
        self.current_file = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(filepath)?
        );
        self.current_lines = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_rotation_writer() {
        let temp_dir = std::env::temp_dir().join(format!("rogger_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        fs::create_dir_all(&temp_dir).unwrap();

        let mut writer = RotationWriter::new(&temp_dir, "test.log".to_string(), 3);

        // Write lines
        writer.write("line1").unwrap();
        writer.write("line2").unwrap();
        writer.write("line3").unwrap();
        // Should rotate on next write
        writer.write("line4").unwrap();

        // Check that at least two files exist
        let mut files: Vec<_> = fs::read_dir(&temp_dir).unwrap().map(|e| e.unwrap()).collect();
        files.sort_by_key(|e| e.file_name());
        assert!(files.len() >= 2, "Should have at least two files after rotation");

        // Check content of first file
        let first_file = &files[0];
        let mut content = String::new();
        fs::File::open(first_file.path()).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!(content.trim(), "line1\nline2\nline3");

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
