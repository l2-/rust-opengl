use std::io::Write;
use chrono::Local;

const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

pub fn init_logger() {
    pretty_env_logger::formatted_builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}:{} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.file().unwrap_or_else(|| ""),
                record.line().unwrap_or_else(|| u32::MAX),
                record.args()
            )
        })
        .filter(None, LOG_LEVEL)
        .init();
}
