extern crate drm;
extern crate gbm;
extern crate khronos_egl as egl;

mod utils;
use utils::*;

// use drm::control::Device as ControlDevice;

// use drm::buffer::DrmFourcc;

use drm::control::{connector};
use gbm::{BufferObjectFlags, Device, Format};


fn print_card_info(card: &dyn utils::Device) {
    // Attempt to acquire and release master lock
    println!("Get Master lock: {:?}", card.acquire_master_lock());
    println!("Release Master lock: {:?}", card.release_master_lock());

    // Get the Bus ID of the device
    println!("Getting Bus ID: {:?}", card.get_bus_id().unwrap().as_ref());

    // Figure out driver in use
    println!("Getting driver info");
    let driver = card.get_driver().unwrap();
    println!("\tName: {:?}", driver.name());
    println!("\tDate: {:?}", driver.date());
    println!("\tDesc: {:?}", driver.description());

    // Enable all possible client capabilities
    println!("Setting client capabilities");
    for &cap in capabilities::CLIENT_CAP_ENUMS {
        println!("\t{:?}: {:?}", cap, card.set_client_capability(cap, true));
    }

    // Get driver capabilities
    println!("Getting driver capabilities");
    for &cap in capabilities::DRIVER_CAP_ENUMS {
        println!("\t{:?}: {:?}", cap, card.get_driver_capability(cap));
    }
}

pub fn main() {
    let card = Card::open_global();

    print_card_info(&card);

    // Load the information.
    let res = card
        .resource_handles()
        .expect("Could not load normal resource ids.");
    let coninfo: Vec<connector::Info> = res
        .connectors()
        .iter()
        .flat_map(|con| card.get_connector(*con))
        .collect();
    // let crtcinfo: Vec<crtc::Info> = res
    //     .crtcs()
    //     .iter()
    //     .flat_map(|crtc| card.get_crtc(*crtc))
    //     .collect();

    // Filter each connector until we find one that's connected.
    let con: &connector::Info = coninfo
        .iter()
        .find(|&i| i.state() == connector::State::Connected)
        .expect("No connected connectors");

    let enc = con.encoders().get(0)
        .expect("No encoder found for connector")
        .unwrap();
    let encinfo = card.get_encoder(enc).unwrap();
    println!("Connector Kind: {:?}", encinfo.kind());

    // Get the first (usually best) mode
    let &mode = con.modes().get(0).expect("No modes found on connector");

    let (disp_width, disp_height) = mode.size();

    println!("disp_width: {}, disp_height: {}",disp_width, disp_height);

    // Find a crtc and FB
    let crtc = encinfo.crtc().expect("No crtcs found");

    // init a GBM device
    let gbm = Device::new(card).unwrap();

    // create a buffer
    let mut bo = gbm
        //.create_surface::<()>(
        .create_buffer_object::<()>(
            disp_width.into(),
            disp_height.into(),
            //Format::Xbgr8888,
            Format::Argb8888,
            //BufferObjectFlags::SCANOUT | BufferObjectFlags::RENDERING,
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
    gbm.set_crtc(crtc, Some(fb), (0, 0), &[con.handle()], Some(mode))
         .unwrap();

}