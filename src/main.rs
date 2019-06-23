// https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod cli;
pub mod config;
pub mod oidcdiscovery;
pub mod client;
pub mod server;
pub mod terminal_utils;

// https://github.com/actix/examples/tree/master/hello-world
use actix_web::{middleware, web, App, HttpServer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "trace");

    let cli = cli::new();
    println!("cli={:?}", cli);

    let rust_log = match cli.log_level.as_str() {
        "trace" | "debug" | "info" | "error" => format!(
            "{log_level},actix_web={log_level},actix_server={log_level}",
            log_level = cli.log_level
        ),
        _ => unreachable!(),
    };
    std::env::set_var("RUST_LOG", rust_log);
    env_logger::init();

    let app = || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(server::authorise_callback)))
            .service(web::resource("/hello").to(|| "Hello world!"))
            .service(web::resource("/api/conformancesuite/callback").to(server::api_authorise_callback))
            .service(web::resource("/conformancesuite/callback").to(server::authorise_callback))
    };
    let addr = "127.0.0.1:8080";
    HttpServer::new(app)
        .bind(addr)?
        .system_exit()
        .run();

    std::process::exit(1);

    let config = config::read(cli.config.as_str()).expect("Couldn't config::read()");
    oidcdiscovery::fetch(config).expect("Couldn't oidcdiscovery::fetch");

    Ok(())
}

// https://github.com/ramosbugs/openidconnect-rs/blob/master/src/registration.rs
