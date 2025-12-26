use crate::log_entry::LogEntry;
use crate::writer::Writer;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

static LEVEL: OnceLock<LogLevel> = OnceLock::new();
static SERVICE: OnceLock<String> = OnceLock::new();
static WRITER: OnceLock<Mutex<Box<dyn Writer + Send>>> = OnceLock::new();

pub struct Logger;

impl Logger {
    pub fn init(level: LogLevel, service: String, writer: Box<dyn Writer + Send>) {
        LEVEL.set(level).unwrap();
        SERVICE.set(service).unwrap();
        WRITER.set(Mutex::new(writer)).unwrap();
    }

    pub fn log(level: LogLevel, message: &str, payload: serde_json::Value) {
        if level >= *LEVEL.get().unwrap() {
            let entry = LogEntry::new(
                SERVICE.get().unwrap().clone(),
                level.to_string(),
                message.to_string(),
                payload,
                None,
            );
            let json = entry.to_json_string().unwrap();
            WRITER
                .get()
                .unwrap()
                .lock()
                .unwrap()
                .write(&json)
                .unwrap();
        }
    }

    pub fn debug(message: &str, payload: serde_json::Value) {
        Self::log(LogLevel::Debug, message, payload);
    }

    pub fn info(message: &str, payload: serde_json::Value) {
        Self::log(LogLevel::Info, message, payload);
    }

    pub fn warn(message: &str, payload: serde_json::Value) {
        Self::log(LogLevel::Warn, message, payload);
    }

    pub fn error(message: &str, payload: serde_json::Value) {
        Self::log(LogLevel::Error, message, payload);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

    #[derive(Debug)]
    struct TestWriter;

    impl Writer for TestWriter {
        fn write(&mut self, line: &str) -> std::io::Result<()> {
            TEST_LOGS.lock().unwrap().push(line.to_string());
            Ok(())
        }
    }

    fn clear_test_logs() {
        TEST_LOGS.lock().unwrap().clear();
    }

    #[test]
    fn test_logger() {
        clear_test_logs();
        let writer = Box::new(TestWriter);
        Logger::init(LogLevel::Info, "test_service".to_string(), writer);

        // Log info level
        Logger::info("info message", serde_json::json!({"key": "value"}));

        // Log debug (should be filtered)
        Logger::debug("debug message", serde_json::json!({}));

        // Check logs
        let logs = TEST_LOGS.lock().unwrap();
        assert_eq!(logs.len(), 1);
        let parsed: serde_json::Value = serde_json::from_str(&logs[0]).unwrap();
        assert_eq!(parsed["service_name"], "test_service");
        assert_eq!(parsed["level"], "info");
        assert_eq!(parsed["message"], "info message");
        assert_eq!(parsed["payload"]["key"], "value");
    }
}