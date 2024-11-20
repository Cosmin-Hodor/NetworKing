use log::LevelFilter;
use env_logger;

pub fn setup_logging() {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
    )
    .format_timestamp_millis()
    .filter(None, LevelFilter::Info)
    .init();
}