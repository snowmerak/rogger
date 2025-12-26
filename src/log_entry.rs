use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub service_name: String,
    pub level: String,
    pub message: String,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

impl LogEntry {
    pub fn new(service_name: String, level: String, message: String, payload: serde_json::Value) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        LogEntry {
            service_name,
            level,
            message,
            timestamp,
            payload,
        }
    }
}