// TODO: add better logger.
use log::{Level, LevelFilter, Metadata, Record};

struct Logger {
    level: Level,
}

impl Logger {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init_log(level: Level) {
    log::set_boxed_logger(Box::new(Logger::new(level)))
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Could not initialized logger");
}
