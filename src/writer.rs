
pub trait Writer: Send + std::fmt::Debug {
    fn write(&mut self, line: &str) -> std::io::Result<()>;
}
