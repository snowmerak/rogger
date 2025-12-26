
pub trait Writer: Send + std::fmt::Debug + std::any::Any {
    fn write(&mut self, line: &str) -> std::io::Result<()>;
}
