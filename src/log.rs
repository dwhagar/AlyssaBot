use std::fmt::{Display, Formatter};
use chrono::{Local, DateTime};

static TIME_FORMAT: &str = "%Y/%m/%d %H:%M";

fn timestamp() -> String {
    Local::now().format(TIME_FORMAT).to_string()
}

#[derive(Debug)]
enum LogLevel {
    Essential(u8),
    Error(u8),
    Warning(u8),
    Debug(u8),
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Essential(level) => write!(f, "[Essential:{}]", level),
            LogLevel::Error(level) => write!(f, "[Error:{}]", level),
            LogLevel::Warning(level) => write!(f, "[Warning:{}]", level),
            LogLevel::Debug(level) => write!(f, "[Debug:{}]", level),
        }
    }
}

pub(crate) struct Log {
    verbosity: u8,
}

impl Log {
    pub(crate) fn new(verbosity: u8) -> Log {
        Log { verbosity }
    }

    fn log(&self, level: LogLevel, message: &str) {
        let level_value = match level {
            LogLevel::Essential(value) => value,
            LogLevel::Error(value) => value,
            LogLevel::Warning(value) => value,
            LogLevel::Debug(value) => value,
        };

        if level_value <= self.verbosity {
            println!("{} {}", timestamp(), level);
            println!("{}", message);
        }
    }

    fn essential(&self, message: &str) {
        self.log(LogLevel::Essential(0), message);
    }

    fn error(&self, message: &str) {
        self.log(LogLevel::Error(1), message);
    }

    fn warn(&self, message: &str) {
        self.log(LogLevel::Warning(2), message);
    }

    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug(3), message);
    }
}