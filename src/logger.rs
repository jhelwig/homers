use log::{self, Log, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};

use settings::Settings;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }
}

pub fn init(settings: &Settings) -> Result<(), SetLoggerError> {
    let log_level = match settings.verbose_level() {
        -1 => LogLevelFilter::Error,
        0 => LogLevelFilter::Info,
        1 => LogLevelFilter::Debug,
        _ => LogLevelFilter::Trace,
    };

    log::set_logger(|max_log_level| {
        max_log_level.set(log_level);
        Box::new(Logger)
    })
}
