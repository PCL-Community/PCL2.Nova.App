use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;
use std::path::PathBuf;

pub struct Logger {
    level: LogLevel,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = -1,
    Warn = 0,
    Info = 1,
    Debug = 2,
    Trace = 3,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level <= self.level {
            let timestamp = SystemTime::now();
            let log_path: PathBuf = dirs_next::config_dir().unwrap().join("PCLNova").join("log.txt");
            let mut log_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(log_path)
                .expect("Failed to open log file.");
            write!(&mut log_file, "[{:?}] {:?}: {}", level, timestamp, message)
                .expect("Failed to write logs.");
        }
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
}
