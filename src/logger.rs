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