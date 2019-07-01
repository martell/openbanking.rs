// https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use log::info;

pub mod cli;
pub mod client;
pub mod config;
pub mod oidcdiscovery;
pub mod server;
pub mod terminal_utils;

// https://github.com/actix/examples/tree/master/hello-world
use actix_web::{middleware, web, App, HttpServer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "trace");

    let cli = cli::new();
    let rust_log = cli.log_level.as_str();
    std::env::set_var(
        "RUST_LOG",
        format!(
            "{log_level},actix_web={log_level},actix_server={log_level}",
            log_level = rust_log
        ),
    );
    env_logger::init();
    info!("cli={:?}", cli);

    // let app = || {
    //     App::new()
    //         // enable logger
    //         .wrap(middleware::Logger::default())
    //         .service(web::resource("/").route(web::get().to(server::authorise_callback)))
    //         .service(web::resource("/hello").to(|| "Hello world!"))
    //         .service(
    //             web::resource("/api/conformancesuite/callback").to(server::api_authorise_callback),
    //         )
    //         .service(web::resource("/conformancesuite/callback").to(server::authorise_callback))
    // };
    // let addr = "127.0.0.1:8080";
    // HttpServer::new(app).bind(addr)?.system_exit().run();

    // std::process::exit(1);

    let path = cli
        .config
        .into_os_string()
        .into_string()
        .expect("config.into_os_string failed");
    let config = config::read(path).expect("config::read failed");
    // let config = config::read(cli.config.to_str().unwrap()).expect("Couldn't config::read()");
    // let config = config::read(cli.config.as_str()).expect("Couldn't config::read()");
    oidcdiscovery::fetch(config).expect("oidcdiscovery::fetch failed");

    Ok(())
}

// https://github.com/ramosbugs/openidconnect-rs/blob/master/src/registration.rs
