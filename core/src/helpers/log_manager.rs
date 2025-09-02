use tracing::{debug, error, info, trace, warn, Level};

/// Simple asynchronous logger backed by `tracing`.
#[derive(Default)]
pub struct LogManager;

impl LogManager {
    /// Emit a log message at the provided `level`.
    pub async fn log(&self, level: Level, message: &str) {
        match level {
            Level::ERROR => error!("{}", message),
            Level::WARN => warn!("{}", message),
            Level::INFO => info!("{}", message),
            Level::DEBUG => debug!("{}", message),
            Level::TRACE => trace!("{}", message),
        }
    }
}
