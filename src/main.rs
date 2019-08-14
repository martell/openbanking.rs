use better_panic;
use log::info;
use pretty_env_logger;

pub mod cli;
pub mod client;
pub mod config;
pub mod http;
pub mod oidcdiscovery;
pub mod server;
pub mod server_new;
pub mod terminal_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_BACKTRACE", "full");
    // std::env::set_var("RUST_LOG", "trace");

    better_panic::Settings::debug()
        .most_recent_first(true)
        .backtrace_first(true)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();

    let cli = cli::new();
    info!("cli={:?}", cli);

    if std::env::var_os("RUST_LOG").is_none() {
        let log_level = cli.log_level.as_str();
        let rust_log = format!("{},actix_web={},actix_server={}", log_level, log_level, log_level);
        std::env::set_var("RUST_LOG", rust_log);
    }
    pretty_env_logger::init();

    server_new::start();

    // let path = cli.config.into_os_string().into_string().expect("config.into_os_string failed");
    // let config = config::Config::read(path).expect("config::read failed");
    // let openid_configuration = oidcdiscovery::OpenIDConfiguration::fetch(config.clone())
    //     .expect("oidcdiscovery::fetch failed");

    // let client = client::OpenBankingClient::new(config.clone(), openid_configuration.clone())?;

    // let thread = server::start(client.clone());

    // let account_requests_response = client.post_account_access_consents()?;
    // let url = client.post_account_access_consents_hybrid_flow(account_requests_response)?;
    // info!("url={}", url);

    // let _ = thread.join().unwrap();

    Ok(())
}
