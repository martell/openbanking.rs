use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug, Default, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[structopt(raw(global_settings = "&[
        structopt::clap::AppSettings::ColoredHelp,
        structopt::clap::AppSettings::VersionlessSubcommands,
        structopt::clap::AppSettings::ArgRequiredElseHelp,
        ]"))]
pub struct Cli {
    #[structopt(
        short = "l",
        long = "log_level",
        help = "The level to configure the logger.",
        raw(default_value = "LogLevel::default().as_str()"),
        raw(possible_values = "&LogLevel::possible_values()")
    )]
    pub log_level: LogLevel,
    #[structopt(
        short = "c",
        long = "config",
        help = "The config file to use.",
        parse(from_os_str),
        default_value = "keys/config/config_tls_client_auth_ps256_ozone.yml"
    )]
    pub config: std::path::PathBuf,
}

#[derive(StructOpt, Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::str::FromStr for LogLevel {
    type Err = Box<std::error::Error>;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        use LogLevel::*;
        match s {
            "error" => Ok(Error),
            "warn" => Ok(Warn),
            "info" => Ok(Info),
            "debug" => Ok(Debug),
            "trace" => Ok(Trace),
            log_level => Err(format!("log_level={} invalid", log_level).into()),
        }
    }
}

impl std::default::Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        use LogLevel::*;
        match self {
            Error => "error",
            Warn => "warn",
            Info => "info",
            Debug => "debug",
            Trace => "trace",
        }
    }

    pub fn possible_values() -> &'static [&'static str] {
        &["error", "warn", "info", "debug", "trace"]
    }
}

pub fn new() -> Cli {
    Cli::from_args()
}
