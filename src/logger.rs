use config::Config;
use log::{Record, Level, Metadata};
use log::LevelFilter;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(config: Config) -> Result<(), String> {
    let level_string = config.get_string("log_level").map_err(|e| format!("config error : {:?}", e))?;
    let level = from_str(level_string)?;

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level)).map_err(|e| format!("Logger setting error: {:?}", e))
}


fn from_str(level: String) -> Result<LevelFilter, String> {
    match level.to_lowercase().as_str() {
        "off" => Ok(LevelFilter::Off),
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        _ => Err(String::from(format!("Invalid string for log level: {:?}", level)))
    }
}