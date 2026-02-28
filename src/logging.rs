//! Logging utilities for ecgen-rs
//!
//! This module provides optional logging capabilities via env_logger.
//! It is only available when the `std` feature is enabled.

use std::sync::atomic::{AtomicBool, Ordering};

static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize the logger with default filter (info)
///
/// # Panics
///
/// Panics if a logger has already been initialized.
pub fn init_logger() {
    init_logger_with_filter("info");
}

/// Initialize the logger with a specific filter string
///
/// # Panics
///
/// Panics if a logger has already been initialized.
pub fn init_logger_with_filter(filter: &str) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter))
        .format_timestamp_millis()
        .init();
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
}

/// Try to initialize the logger with default filter (info)
///
/// Returns an error if a logger has already been initialized.
pub fn try_init_logger() -> Result<(), log::SetLoggerError> {
    try_init_logger_with_filter("info")
}

/// Try to initialize the logger with a specific filter string
///
/// Returns an error if a logger has already been initialized.
pub fn try_init_logger_with_filter(filter: &str) -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter))
        .format_timestamp_millis()
        .try_init()?;
    LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
    Ok(())
}

/// Check if the logger has been initialized
pub fn is_logger_initialized() -> bool {
    LOGGER_INITIALIZED.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_init_logger() {
        // This test verifies that try_init_logger works
        // Note: In practice, logger can only be init once per process
        let _ = try_init_logger();
    }

    #[test]
    fn test_try_init_logger_with_filter() {
        // This test verifies that try_init_logger_with_filter works
        let _ = try_init_logger_with_filter("warn");
    }

    #[test]
    fn test_is_logger_initialized() {
        // After trying to init, should be initialized
        let _ = try_init_logger();
        assert!(is_logger_initialized());
    }
}
