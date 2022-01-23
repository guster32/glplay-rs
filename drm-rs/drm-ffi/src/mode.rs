//!
//! Bindings to the DRM's modesetting capabilities.
//!

use drm_sys::*;
use ioctl;

use result::SystemError as Error;
use std::os::unix::io::RawFd;

use utils;

/// Enumerate most card resources.
pub fn get_resources(
    fd: RawFd,
    fbs: Option<&mut &mut [u32]>,
    crtcs: Option<&mut &mut [u32]>,
    connectors: Option<&mut &mut [u32]>,
    encoders: Option<&mut &mut [u32]>,
) -> Result<drm_mode_card_res, Error> {
    let mut res = drm_mode_card_res {
        fb_id_ptr: map_ptr!(&fbs),
        crtc_id_ptr: map_ptr!(&crtcs),
        connector_id_ptr: map_ptr!(&connectors),
        encoder_id_ptr: map_ptr!(&encoders),
        count_fbs: map_len!(&fbs),
        count_crtcs: map_len!(&crtcs),
        count_connectors: map_len!(&connectors),
        count_encoders: map_len!(&encoders),
        ..Default::default()
    };

    unsafe {
        ioctl::mode::get_resources(fd, &mut res)?;
    }

    map_shrink!(fbs, res.count_fbs as usize);
    map_shrink!(crtcs, res.count_crtcs as usize);
    map_shrink!(connectors, res.count_connectors as usize);
    map_shrink!(encoders, res.count_encoders as usize);

    Ok(res)
}