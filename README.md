# Snowlog

A structured logging library for Rust that outputs logs in JSON format with support for log rotation, multiple writers, and global logging.

## Features

- **Structured Logging**: Logs are structured as JSON objects with fields like service name, level, message, timestamp, and payload.
- **Log Levels**: Supports Debug, Info, Warn, Error levels with filtering.
- **Writers**:
  - `ConsoleWriter`: Outputs to stdout.
  - `RotationWriter`: Writes to files with automatic rotation based on line count.
  - `MultiWriter`: Combines multiple writers for simultaneous output.
- **Global Logger**: Uses `OnceLock` for thread-safe global configuration.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rogger = "0.1.0"
serde_json = "1.0"  # For JSON payloads
```

## Usage

### Basic Setup

```rust
use rogger::{Logger, LogLevel, ConsoleWriter};

fn main() {
    // Initialize the global logger
    let writer = Box::new(ConsoleWriter);
    Logger::init(LogLevel::Info, "my_app".to_string(), writer);

    // Log messages
    Logger::info("Application started", serde_json::json!({"version": "1.0"}));
    Logger::error("An error occurred", serde_json::json!({"error_code": 500}));
}
```

### Using File Rotation

```rust
use rogger::RotationWriter;
use std::path::Path;

let writer = Box::new(RotationWriter::new(
    Path::new("./logs"),
    "app.log".to_string(),
    1000,  // Max lines per file
));
Logger::init(LogLevel::Debug, "my_app".to_string(), writer);
```

### Multiple Writers

```rust
use rogger::{MultiWriter, ConsoleWriter, RotationWriter};

let multi_writer = Box::new(MultiWriter::new());
multi_writer.add_writer(Box::new(ConsoleWriter));
multi_writer.add_writer(Box::new(RotationWriter::new(
    Path::new("./logs"),
    "app.log".to_string(),
    1000,
)));
Logger::init(LogLevel::Info, "my_app".to_string(), multi_writer);
```

### Log Levels

- `LogLevel::Debug`
- `LogLevel::Info`
- `LogLevel::Warn`
- `LogLevel::Error`

Logs below the set level are filtered out.

## API

### Logger

- `Logger::init(level: LogLevel, service: String, writer: Box<dyn Writer + Send>)`: Initialize the global logger.
- `Logger::debug(message: &str, payload: serde_json::Value)`
- `Logger::info(message: &str, payload: serde_json::Value)`
- `Logger::warn(message: &str, payload: serde_json::Value)`
- `Logger::error(message: &str, payload: serde_json::Value)`

### Writers

All writers implement the `Writer` trait.

- `ConsoleWriter`: Prints to stdout.
- `RotationWriter::new(path: &Path, base_filename: String, max_lines: usize)`: Rotates files when `max_lines` is reached.
- `MultiWriter::new()` + `add_writer(writer: Box<dyn Writer>)`: Combines writers.

### LogEntry

Represents a log entry. Can be serialized to JSON.

```rust
let entry = LogEntry::new("service".to_string(), "info".to_string(), "message".to_string(), payload, None);
let json = entry.to_json_string().unwrap();
```

## License

MPL-2.0