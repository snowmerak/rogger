use crate::writer::Writer;

#[derive(Debug)]
pub struct ConsoleWriter;

impl Writer for ConsoleWriter {
    fn write(&mut self, line: &str) -> std::io::Result<()> {
        println!("{}", line);
        Ok(())
    }
}