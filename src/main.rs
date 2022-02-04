/// Check the `util` module to see how the `Card` structure is implemented.

extern crate drm_rs;
extern crate gbm_rs;
pub mod utils;

use utils::*;

use drm_rs::buffer::DrmFourcc;
use drm_rs::control::{connector, crtc};
// use gbm_rs::Device as GbmDevice;


// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]
// #![allow(dead_code)]
// // pub mod api;
// use libc;
// use std::cell::RefCell;
// use std::os::unix::io;
// use std::result::Result::Err as Error
// use std::os::unix::io::AsRawFd;

// // mod egl {
// //     include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));
// // }

// ///////////////////////////////  DRM  ///////////////////////////////

// mod drm { include!(concat!(env!("OUT_DIR"), "/drm_bindings.rs")); }
// pub const DRM_CONTEXT_VERSION: libc::c_int = 2;

// pub trait DrmEventContext {
//     fn vblank_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32);
//     fn page_flip_handler(&mut self, fd: io::RawFd, sequence: u32, sec: u32, usec: u32, data: i32);
// }

// // Thread local storage for event contexts.
// thread_local!(static CONTEXT: RefCell<Option<Box<dyn DrmEventContext>>> = RefCell::new(None));

// extern "C" fn vblank_handler(fd: libc::c_int,
//     sequence: libc::c_uint,
//     tv_sec: libc::c_uint,
//     tv_usec: libc::c_uint,
//     user_data: *mut libc::c_void) {
//     CONTEXT.with(|s| if let Some(ref mut context) = *s.borrow_mut(){
//             context.vblank_handler(fd, sequence, tv_sec, tv_usec, user_data as i32);
//     });
// }

// extern "C" fn page_flip_handler(fd: libc::c_int,
//     sequence: libc::c_uint,
//     tv_sec: libc::c_uint,
//     tv_usec: libc::c_uint,
//     user_data: *mut libc::c_void) {
//     CONTEXT.with(|s| if let Some(ref mut context) = *s.borrow_mut() {
//         context.page_flip_handler(fd, sequence, tv_sec, tv_usec, user_data as i32);
//     });
// }

// pub fn handle_event(fd: io::RawFd, context: Box<dyn DrmEventContext>) {
//     CONTEXT.with(|s| *s.borrow_mut() = Some(context));

//     let mut drm_context = drm::drmEventContext {
//         version: DRM_CONTEXT_VERSION,
//         vblank_handler: Option::Some(vblank_handler),
//         page_flip_handler: Option::Some(page_flip_handler),
//         page_flip_handler2: None,
//         sequence_handler: None
//     };

//     unsafe {
//         drm::drmHandleEvent(fd, &mut drm_context as *mut drm::drmEventContext);
//     }

//     CONTEXT.with(|s| *s.borrow_mut() = None);
// }

// struct MyDrm {
//     fd: io::RawFd,
//     mode: drm::drmModeModeInfo,
//     // crtc_id: libc::c_uint,
//     // connector_id: libc::c_uint
// }
pub fn read_card_properties() {
    let card = Card::open_global();

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

pub fn init_drm() {
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
    let con = coninfo
        .iter()
        .find(|&i| i.state() == connector::State::Connected)
        .expect("No connected connectors");

    // Get the first (usually best) mode
    let &mode = con.modes().get(0).expect("No modes found on connector");

    let (disp_width, disp_height) = mode.size();

    let depth:u32 = card.get_driver_capability(drm_rs::DriverCapability::DumbPreferredDepth).unwrap() as u32;
    println!("{:#?}", depth);
    // Find a crtc and FB
    let crtc = crtcinfo.get(0).expect("No crtcs found");

    // Select the pixel format
    let fmt = DrmFourcc::Xbgr8888;

    // let _gbm = GbmDevice::new(card).expect("Could not create gbm device");


    // Create a DB
    // If buffer resolution is larger than display resolution, an ENOSPC (not enough video memory)
    // error may occur
    let mut db = card
        .create_dumb_buffer((disp_width.into(), disp_height.into()), fmt, 32)
        .expect("Could not create dumb buffer");

    // Map it and grey it out.
    {
        let mut map = card
            .map_dumb_buffer(&mut db)
            .expect("Could not map dumbbuffer");
        for b in map.as_mut() {
            *b = 128;
        }
    }

    // Create an FB:
    let fb = card
        .add_framebuffer(&db, 24, 32)
        .expect("Could not create FB");

    println!("{:#?}", mode);
    println!("{:#?}", fb);
    println!("{:#?}", db);

    // Set the crtc
    // On many setups, this requires root access.
    card.set_crtc(crtc.handle(), Some(fb), (0, 0), &[con.handle()], Some(mode))
        .expect("Could not set CRTC");

    let five_seconds = ::std::time::Duration::from_millis(5000);
    ::std::thread::sleep(five_seconds);

    card.destroy_framebuffer(fb).unwrap();
    card.destroy_dumb_buffer(db).unwrap();
}

///////////////////////////////   MAIN  ///////////////////////////////
pub fn main() {
    read_card_properties();
    init_drm();
    println!("THisis a test");
}
///////////////////////////////   MAIN  ///////////////////////////////