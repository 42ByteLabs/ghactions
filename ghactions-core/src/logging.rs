//! Logging utilities for GitHub Actions
use env_logger::Builder;
use std::io::Write;

/// Initialise and create a `env_logger::Builder` which follows the
/// GitHub Actions logging syntax.
///
pub fn init_logger() -> Builder {
    let mut builder = Builder::from_default_env();

    // Make sure the target is STDOUT
    builder.target(env_logger::Target::Stdout);

    // Find and setup the correct log level
    let log_level = get_log_level();
    log::debug!("Setting log level to {:?}", log_level);

    builder.filter(None, log_level);
    builder.write_style(env_logger::WriteStyle::Always);

    // Custom Formatter for Actions
    builder.format(|buf, record| match record.level().as_str() {
        "DEBUG" => writeln!(buf, "::debug :: {}", record.args()),
        "WARN" => writeln!(buf, "::warning :: {}", record.args()),
        "ERROR" => {
            writeln!(buf, "::error :: {}", record.args())
        }
        _ => writeln!(buf, "{}", record.args()),
    });

    builder
}

/// Get the Log Level for the logger
///
/// The log level is determined by the presence of the
/// `DEBUG` or `RUNNER_DEBUG` environment variables.
fn get_log_level() -> log::LevelFilter {
    if std::env::var("DEBUG").is_ok() || std::env::var("RUNNER_DEBUG").is_ok() {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    }
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
        ::log::log!(::log::Level::Info, "::error file={},line={},col={} :: {}", $file, $line, $column, $msg)
    };
    // errorf!("a {} event", "log")
    ($($arg:tt)+) => (::log::log!($crate::Level::Error, $($arg)+))
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
        ::log::log!(log::Level::Info, "::group::{}", $dst)
    };
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
        ::log::log!(log::Level::Info, "::endgroup::")
    };
}

/// Sets the output of the Actions which can be used in subsequent Actions.
///
/// # Examples
///
/// ```rust
/// use ghactions::setoutput;
///
/// # fn foo() {
/// setoutput!("hello", "world");
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! setoutput {
    // setoutput!("name", "value")
    ($($arg:tt)+) => {
        {
            use std::io::Write;
            let output = ::std::format!("::set-output name={}::{}", $($arg)+);
            #[cfg(feature = "log")]
            {
                ::log::log!(::log::Level::Info, "{}", output);
            }

            let output_file = std::env::var("GITHUB_OUTPUT").unwrap_or_else(|_| "/tmp/github_actions.env".to_string());
            // Append to the file
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_file)
                .unwrap();
            // Append to end of file
            ::std::writeln!(file, "{}", output).unwrap();
        }
    }
}
