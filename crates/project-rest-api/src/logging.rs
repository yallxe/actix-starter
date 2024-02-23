use flexi_logger::{DeferredNow, style, TS_DASHES_BLANK_COLONS_DOT_BLANK};
use log::Record;

fn custom_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();

    #[cfg(debug_assertions)]
    return write!(
        w,
        "[{}] {} [{}] {}:{}: {}",
        style(level).paint(now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK).to_string()),
        style(level).paint(record.level().to_string()),
        record.module_path().unwrap_or("<unnamed>"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        style(level).paint(&record.args().to_string())
    );

    #[cfg(not(debug_assertions))]
    return write!(
        w,
        "[{}] {} [{}]: {}",
        style(level).paint(now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK).to_string()),
        style(level).paint(record.level().to_string()),
        record.module_path().unwrap_or("<unnamed>"),
        style(level).paint(&record.args().to_string())
    );

}

pub fn setup_logging() {
    flexi_logger::Logger::try_with_env_or_str("debug")
        .unwrap()
        .format(custom_format)
        .start()
        .unwrap();
}