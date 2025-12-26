use crate::writer::Writer;

pub struct MultiWriter {
    writers: Vec<Box<dyn Writer>>,
}

impl MultiWriter {
    pub fn new() -> Self {
        MultiWriter {
            writers: Vec::new(),
        }
    }

    pub fn add_writer(&mut self, writer: Box<dyn Writer>) {
        self.writers.push(writer);
    }
}

impl Writer for MultiWriter {
    fn write(&mut self, line: &str) -> std::io::Result<()> {
        for writer in &mut self.writers {
            writer.write(line)?;
        }
        Ok(())
    }
}