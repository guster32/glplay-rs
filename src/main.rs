/// Check the `util` module to see how the `Card` structure is implemented.

extern crate drm;
pub mod utils;

use utils::*;


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



pub fn init_drm() {
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

///////////////////////////////   MAIN  ///////////////////////////////
pub fn main() {
    init_drm();
    println!("THisis a test");
}
///////////////////////////////   MAIN  ///////////////////////////////