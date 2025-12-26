
pub trait Writer {
    fn write(&mut self, line: &str) -> std::io::Result<()>;
}
