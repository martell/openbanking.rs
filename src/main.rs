// #[macro_use]
// extern crate log;
extern crate dialoguer;
extern crate term_size;


use log::{error, info};
use rand::{thread_rng, Rng};
use std::time::Duration;

use dialoguer::{theme::ColorfulTheme, Select};

mod client;
mod config;
mod oidcdiscovery;

// https://github.com/ramosbugs/openidconnect-rs/blob/master/src/registration.rs

// fn main() -> Result<(), std::io::Error> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // std::env::set_var("RUST_LOG", "trace,actix_web=trace,actix_server=trace");
    // std::env::set_var("RUST_LOG", "debug,actix_web=debug,actix_server=debug");
    std::env::set_var("RUST_LOG", "info,actix_web=info,actix_server=info");
    // std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    if let Some((w, h)) = term_size::dimensions() {
        info!("width={}, height: {}", w, h);
    } else {
        error!("Unable to get term size")
    }

    // let selections = &[
    //     "Ice Cream",
    //     "Vanilla Cupcake",
    //     "Chocolate Muffin",
    //     "A Pile of sweet, sweet mustard",
    // ];

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Pick your flavor")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();
    // println!("Enjoy your {}!", selections[selection]);

    // let pb = indicatif::ProgressBar::new(100);
    // // pb.enable_steady_tick(200);

    // let wait = Duration::from_millis(thread_rng().gen_range(10, 30));
    // let child = std::thread::spawn(move || {
    //     std::thread::sleep(wait);
    //     std::thread::sleep(Duration::from_millis(750));
    let config = config::read().expect("Couldn't config::read()");
    // std::thread::sleep(wait);
    // std::thread::sleep(Duration::from_millis(750));
    oidcdiscovery::fetch(config).expect("Couldn't oidcdiscovery::fetch");
    // });

    // for _ in 0..100 {
    //     std::thread::sleep(Duration::from_millis(25));
    //     // pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_and_clear();

    // child
    //     .join()
    //     .expect("Couldn't join on the associated thread");

    Ok(())
}
