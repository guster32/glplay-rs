#[macro_use]
extern crate slog;
extern crate smithay;
extern crate tracing;

use std::sync::Mutex;

use slog::Drain;

///////////////////////////////   MAIN  ///////////////////////////////
fn main() {
    let log = slog::Logger::root(Mutex::new(slog_term::term_full().fuse()).fuse(), o!());
    info!(log, "This main needs to be re-written.");
}
///////////////////////////////   MAIN  ///////////////////////////////
