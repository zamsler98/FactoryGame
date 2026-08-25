use log::{Level, Log, Metadata, Record};

struct MacroquadLogger;

static LOGGER: MacroquadLogger = MacroquadLogger;

impl Log for MacroquadLogger {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        let message = format!("[{}] {}", record.target(), record.args());

        match record.level() {
            Level::Error => macroquad::logging::error!("{}", message),
            Level::Warn => macroquad::logging::warn!("{}", message),
            Level::Info => macroquad::logging::info!("{}", message),
            Level::Debug => macroquad::logging::debug!("{}", message),
            Level::Trace => macroquad::logging::trace!("{}", message),
        }
    }

    fn flush(&self) {}
}

pub(crate) fn init_logging() {
    log::set_logger(&LOGGER).expect("logger should only be initialized once");
    //TODO - Change this based off of build version
    log::set_max_level(log::LevelFilter::Debug)
}
