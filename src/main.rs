extern crate nix;

extern crate libloading;
extern crate khronos_egl as egl;

use nix::fcntl::{ open, OFlag };
use nix::sys::stat::Mode;
use gbm_sys::{ gbm_device};

fn main() {

    let lib = unsafe { libloading::Library::new("libEGL.so.1").expect("unable to find libEGL.so.1") }; 
    let egl = unsafe { egl::DynamicInstance::<egl::EGL1_5>::load_required_from(lib).expect("unable to load libEGL.so.1") };
    
    let input_fd = open("/dev/dri/card0", OFlag::O_RDWR, Mode::empty())
         .unwrap();
    let gbm_device:*mut gbm_device = unsafe { gbm_sys::gbm_create_device(input_fd) };
    let attr:&[usize] = &[0];
    let egl_dpy = egl.get_platform_display(0x31D7, gbm_device as *mut std::ffi::c_void, attr).unwrap();
    let res = egl.initialize(egl_dpy);
    println!("Hello, world! res is OKay!");
    
}
