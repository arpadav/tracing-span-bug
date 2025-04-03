// --------------------------------------------------
// re-exports
// --------------------------------------------------
pub use tokio;
pub use tracing;

// --------------------------------------------------
// external
// --------------------------------------------------
use std::sync::OnceLock;
use tracing::Dispatch;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::MakeWriterExt;

/// Directory to store log files
pub const LOG_DIR: &str = "logs";

/// Global guard to keep the global default logger's [`WorkerGuard`] alive
static GLOBAL_LOGGER_GUARD: OnceLock<WorkerGuard> = OnceLock::new();

/// Initializes the a logger. This is a custom logger, with a given name
/// and a [`tracing::Level`]
pub fn init_global_logger(file_name: &str, level: tracing::Level) {
    let _ = std::fs::create_dir_all(LOG_DIR);
    let file_appender = tracing_appender::rolling::never(LOG_DIR, file_name);
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);
    match GLOBAL_LOGGER_GUARD.set(guard) {
        Ok(_) => (),
        Err(_) => {
            tracing::trace!("Global logger already set. Ignoring.");
            return;
        },
    }
    let combined_writer = non_blocking_file.and(std::io::stdout.with_max_level(level));
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_writer(combined_writer)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global default logger");
}

/// Constructs a logger that writes logs to a file and the console.
pub fn build_logger(file_name: &str, level: tracing::Level) -> (Dispatch, WorkerGuard) {
    let file_appender = tracing_appender::rolling::never(LOG_DIR, file_name);
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);
    let combined_writer = non_blocking_file.and(std::io::stdout.with_max_level(level));
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(level)
        .with_writer(combined_writer)
        .finish();
    let dispatch = Dispatch::new(subscriber);
    (dispatch, guard)
}
