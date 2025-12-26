use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub service_name: String,
    pub level: String,
    pub message: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    pub payload: serde_json::Value,
}

impl LogEntry {
    pub fn new(service_name: String, level: String, message: String, payload: serde_json::Value, duration: Option<f64>) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        LogEntry {
            service_name,
            level,
            message,
            timestamp,
            duration: duration,
            payload,
        }
    }

    /// Serialize the LogEntry to a JSON string.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize the LogEntry to JSON bytes.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_string(self).map(|s| s.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_new() {
        let payload = serde_json::json!({"key": "value"});
        let entry = LogEntry::new(
            "test_service".to_string(),
            "info".to_string(),
            "test message".to_string(),
            payload.clone(),
            None,
        );

        assert_eq!(entry.service_name, "test_service");
        assert_eq!(entry.level, "info");
        assert_eq!(entry.message, "test message");
        assert_eq!(entry.payload, payload);
        assert!(entry.timestamp.len() > 0); // Timestamp should be set
    }

    #[test]
    fn test_to_json_string() {
        let payload = serde_json::json!({"user": "alice"});
        let entry = LogEntry::new(
            "service".to_string(),
            "error".to_string(),
            "error occurred".to_string(),
            payload,
            None,
        );

        let json_str = entry.to_json_string().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["service_name"], "service");
        assert_eq!(parsed["level"], "error");
        assert_eq!(parsed["message"], "error occurred");
        assert_eq!(parsed["payload"]["user"], "alice");
        assert!(parsed["timestamp"].is_string());
    }

    #[test]
    fn test_to_json_bytes() {
        let payload = serde_json::json!({"data": 123});
        let entry = LogEntry::new(
            "byte_service".to_string(),
            "debug".to_string(),
            "byte test".to_string(),
            payload,
            None,
        );

        let bytes = entry.to_json_bytes().unwrap();
        let json_str = String::from_utf8(bytes).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        assert_eq!(parsed["service_name"], "byte_service");
        assert_eq!(parsed["level"], "debug");
        assert_eq!(parsed["message"], "byte test");
        assert_eq!(parsed["payload"]["data"], 123);
    }
}