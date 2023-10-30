use std::os::fd::{AsRawFd, RawFd};

extern crate drm_rs;

/// A simple wrapper for a device node.
pub struct Card(std::fs::File);

use drm_rs::control::{
    connector::State as ConnectorState, crtc, framebuffer, Device as ControlDevice,
};
use drm_rs::Device;

/// Simple helper methods for opening a `Card`.
impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        Card(options.open(path).unwrap())
    }
}

impl AsRawFd for Card {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl drm_rs::Device for Card {}
impl ControlDevice for Card {}

fn main() {
    let gpu = Card::open("/dev/dri/card0");
    let drv = gpu.get_driver().unwrap();
    println!("{:#?}", drv.description());

    // Get a set of all modesetting resource handles (excluding planes):
    let res_handles = ControlDevice::resource_handles(&gpu).unwrap();

    // Use first connected connector
    let connector_info = res_handles
        .connectors()
        .iter()
        .map(|conn| gpu.get_connector(*conn).unwrap())
        .find(|conn| conn.state() == ConnectorState::Connected)
        .unwrap();
    println!("{:#?}", connector_info);
}
