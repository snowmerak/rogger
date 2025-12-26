use crate::writer::Writer;

pub struct ConsoleWriter;

impl Writer for ConsoleWriter {
    fn write(&mut self, line: &str) -> std::io::Result<()> {
        println!("{}", line);
        Ok(())
    }
}