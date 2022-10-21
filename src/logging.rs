
use std::io::Write;
use std::env;
use env_logger::Builder;


/// Initialise and create a `env_logger::Builder` which follows the 
/// GitHub Actions logging syntax.
///
/// # Examples
///
/// ```
/// use log::{info, debug, warn, error};
/// use ghactions::init_logger;
///
/// # fn foo() {
/// let mut builder = init_logger();
/// // Initialise the logger
/// builder.init();
///
/// info!("Log some information");
/// debug!("Debugging content in Actions");
/// warn!("Warn the user something is going wrong...");
/// error!("Well, something really went wrong! Even in Rust");
/// # }
/// ```
pub fn init_logger() -> Builder {
    let mut builder = Builder::from_default_env();
    // Make sure the target is STDOUT
    builder.target(env_logger::Target::Stdout);
    // Find and setup the corrent log level 
    builder.filter_level(get_log_level());
    // Custom Formatter for Actions
    builder
        .format(|buf, record| {
            match record.level().as_str() {
                "DEBUG" => writeln!(buf, "::debug :: {}", record.args()), 
                "WARN" => writeln!(buf, "::warning :: {}", record.args()),
                "ERROR" => {
                    writeln!(buf, "::error :: {}", record.args())
                },
                _ => writeln!(buf, "{}", record.args())
            }
        });

    builder
}


/// Get the Log Level for the logger
fn get_log_level() -> log::LevelFilter {
    // DEBUG 
    match env::var("DEBUG") {
        Ok(_value) => return log::LevelFilter::Debug,
        Err(_err) => ()
    }
    // ACTIONS_RUNNER_DEBUG
    match env::var("ACTIONS_RUNNER_DEBUG") {
        Ok(_value) => return log::LevelFilter::Debug,
        Err(_err) => ()
    };

    log::LevelFilter::Info
}


/// Error for files (including line and column numbers)
///
/// # Examples
///
/// ```
/// use ghactions::errorf;
///
/// # fn foo() {
/// errorf!(
///     file: "src/main.rs",
///     line: 0,
///     column: 0,
///     "Error checking file"
/// );
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! errorf {
    // errorf!(file: "./lib.rs", line: 0, column: 0, "Sample Error")
    (file: $file:expr, line: $line:expr, column: $column:expr, $msg:tt) => {
        log!($crate::Level::Info, "::error file={},line={},col={} :: {}", $file, $line, $column, $msg)
    };
    // errorf!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::Level::Error, $($arg)+))
}


/// Group Macros 
///
/// # Examples
///
/// ```
/// use ghactions::group;
///
/// # fn foo() {
/// group!("working group");
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! group {
    // group!("Group name")
    ($dst:expr $(,)?) => {
        log!(log::Level::Info, "::group::{}", $dst)
    }
}

/// End Group Macros 
///
/// # Examples
///
/// ```
/// use ghactions::groupend;
///
/// # fn foo() {
/// groupend!();
/// # }

/// ```
#[macro_export(local_inner_macros)]
macro_rules! groupend {
    // group_end!()
    () => {
        log!(log::Level::Info, "::endgroup::")
    }
}


