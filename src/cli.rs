extern crate clap;

use structopt::StructOpt;

// Our CLI arguments. (help and version are automatically generated)
// Documentation on how to use:
// https://docs.rs/structopt/0.2.10/structopt/index.html#how-to-derivestructopt
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Cli {
    #[structopt(
        short = "l",
        long = "log_level",
        default_value = "info",
        help = "The level to configure the logger.",
        raw(possible_values = r#"&["error", "warn", "info", "debug"]"#)
    )]
    pub log_level: String,
    #[structopt(
        short = "c",
        long = "config",
        default_value = "keys/config/config_tls_client_auth_ps256_ozone.yml",
        help = "The config file to use."
    )]
    pub config: String,
}

pub fn new() -> Cli {
    let args = Cli::from_args();
    args

    // let app = clap::App::new("openbanking.rs")
    //     // .version("0.1.0")
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author("Mohamed Bana <m@bana.io>")
    //     .about("Open Banking client written in Rust Programming Language")
    //     .arg(
    //         clap::Arg::with_name("log_level")
    //             .short("l")
    //             .long("log_level")
    //             .help("The level to configure the logger")
    //             .default_value("info")
    //             .possible_values(&["trace", "debug", "info", "error"])
    //             .takes_value(true)
    //             .index(1),
    //     )
    //     .arg(
    //         clap::Arg::with_name("config")
    //             .short("c")
    //             .long("config")
    //             .help("The config file to use")
    //             .default_value("keys/config/config_tls_client_auth_ps256_ozone.yml")
    //             .takes_value(true)
    //             .index(2),
    //     );
}

