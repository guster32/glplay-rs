extern crate drm;
extern crate gbm;

mod utils;
use utils::*;

// use drm::control::Device as ControlDevice;

// use drm::buffer::DrmFourcc;

use drm::control::{connector, crtc};
use gbm::{BufferObjectFlags, Device, Format};

pub fn main() {
    let card = Card::open_global();

    // Load the information.
    let res = card
        .resource_handles()
        .expect("Could not load normal resource ids.");
    let coninfo: Vec<connector::Info> = res
        .connectors()
        .iter()
        .flat_map(|con| card.get_connector(*con))
        .collect();
    let crtcinfo: Vec<crtc::Info> = res
        .crtcs()
        .iter()
        .flat_map(|crtc| card.get_crtc(*crtc))
        .collect();

    // Filter each connector until we find one that's connected.
    let con: &connector::Info = coninfo
        .iter()
        .find(|&i| i.state() == connector::State::Connected)
        .expect("No connected connectors");

    // Get the first (usually best) mode
    let &mode = con.modes().get(0).expect("No modes found on connector");

    let (disp_width, disp_height) = mode.size();

    println!("disp_width: {}, disp_height: {}",disp_width, disp_height);

    // Find a crtc and FB
    let crtc = crtcinfo.get(0).expect("No crtcs found");

    // init a GBM device
    let gbm = Device::new(card).unwrap();

    // create a buffer
    let mut bo = gbm
        .create_buffer_object::<()>(
            disp_width.into(),
            disp_height.into(),
            Format::Argb8888,
            BufferObjectFlags::SCANOUT | BufferObjectFlags::WRITE,
        )
        .unwrap();

    // write something to it (usually use import or egl rendering instead)
    let buffer = {
        let mut buffer = Vec::new();
        for i in 0..disp_width {
            for _ in 0..disp_height {
                buffer.push(if i % 2 == 0 { 0 } else { 255 });
            }
        }
        buffer
    };
    let _noop = bo.write(&buffer).unwrap();

    // create a framebuffer from our buffer
    let fb = gbm.add_framebuffer(&bo, 32, 32).unwrap();

    // display it (and get a crtc, mode and connector before)
    gbm.set_crtc(crtc.handle(), Some(fb), (0, 0), &[con.handle()], Some(mode))
        .unwrap();

}