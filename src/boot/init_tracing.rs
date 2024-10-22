
// use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;
use crate::types::MyError;

pub fn init_tracing(level: &str) -> Result<(), MyError> {
    // Convert trace level input to uppercase to handle case insensitivity
    let trace_level_input = level.to_uppercase();

    // Set default level filter based on the config
    let level_filter = match trace_level_input.as_str() {
        "TRACE" => LevelFilter::TRACE,
        "DEBUG" => LevelFilter::DEBUG,
        "INFO" => LevelFilter::INFO,
        "WARN" => LevelFilter::WARN,
        "ERROR" => LevelFilter::ERROR,
        _ => LevelFilter::ERROR,  // Default if not matched
    };

    // Initialize the env filter, applying the default level to all modules
    let mut env_filter = EnvFilter::new(format!("{}", level_filter));

    // Conditionally add the hyper_util trace level directive only when TRACE is enabled globally
    if level_filter == LevelFilter::TRACE {
        env_filter = env_filter.add_directive("hyper_util::client::legacy::pool=trace".parse()?);
    }

    // Initialize the tracing subscriber with the constructed filter
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    Ok(())
}

