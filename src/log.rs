use std::{borrow::Cow, sync::Arc, time::SystemTime};

use chrono::Local;
use parking_lot::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct Log {
    entries: Arc<Mutex<Vec<LogEntry>>>,
}

pub struct LogEntry {
    pub message: Cow<'static, str>,
    pub level: LogLevel,
    pub timestamp: chrono::DateTime<Local>,
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Log {
    pub fn new() -> Log {
        Log {
            entries: Arc::new(Mutex::new(vec![LogEntry {
                message: "Starting Nautical Navigator".into(),
                level: LogLevel::Info,
                timestamp: Local::now()
            }])),
        }
    }

    pub fn entries(&self) -> MutexGuard<'_, Vec<LogEntry>> {
        self.entries.lock()
    }

    pub fn log(&self, message: impl Into<Cow<'static, str>>, level: LogLevel) {
        self.entries.lock().push(LogEntry {
            timestamp: Local::now(),
            message: message.into(),
            level,
        });
    }

    pub fn info(&self, message: impl Into<Cow<'static, str>>) {
        self.log(message, LogLevel::Info);
    }

    pub fn warning(&self, message: impl Into<Cow<'static, str>>) {
        self.log(message, LogLevel::Warning);
    }

    pub fn error(&self, message: impl Into<Cow<'static, str>>) {
        self.log(message, LogLevel::Error);
    }
}

impl LogLevel {
    pub fn name(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        }
    }
}
