use log::{info, warn, error, debug, trace};

/// Initializes the logger.
pub fn init_logger() {
    env_logger::init();
}
