#[macro_use]
extern crate slog;

use slog::Drain;

use std::sync::Mutex;

extern crate drm_rs;

/// A simple wrapper for a device node.
pub struct Card(std::fs::File);

use drm_rs::control::{
    connector::State as ConnectorState, crtc, framebuffer, Device as ControlDevice,
};
use drm_rs::Device;

/// Implementing `AsFd` is a prerequisite to implementing the traits found
/// in this crate. Here, we are just calling `as_fd()` on the inner File.
impl std::os::unix::io::AsFd for Card {
    fn as_fd(&self) -> std::os::unix::io::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

/// With `AsFd` implemented, we can now implement `drm::Device`.
impl Device for Card {}
impl ControlDevice for Card {}

/// Simple helper methods for opening a `Card`.
impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        Card(options.open(path).unwrap())
    }

    pub fn open_global() -> Self {
        Self::open("/dev/dri/card0")
    }
}

///////////////////////////////   MAIN  ///////////////////////////////
fn main() {
    let log = slog::Logger::root(Mutex::new(slog_term::term_full().fuse()).fuse(), o!());
    let gpu = Card::open("/dev/dri/card0");
    let drv = gpu.get_driver().unwrap();
    println!("{:#?}", drv.description());

    // Get a set of all modesetting resource handles (excluding planes):
    let res_handles = ControlDevice::resource_handles(&gpu).unwrap();

    // Use first connected connector
    let connector_info = res_handles
        .connectors()
        .iter()
        .map(|conn| gpu.get_connector(*conn, false).unwrap())
        .find(|conn| conn.state() == ConnectorState::Connected)
        .unwrap();
    info!(log, "{:#?}", connector_info);
}
///////////////////////////////   MAIN  ///////////////////////////////
