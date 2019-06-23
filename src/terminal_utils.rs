extern crate dialoguer;
extern crate term_size;

use log::{error, trace};

pub fn dimensions_or_exit() -> (usize, usize) {
    if let Some((w, h)) = term_size::dimensions() {
        trace!("term_size::dimensions: width={}, height={}", w, h);
        (w, h)
    } else {
        error!("term_size::dimensions: unable to get term size");
        std::process::exit(1);
    }
}
