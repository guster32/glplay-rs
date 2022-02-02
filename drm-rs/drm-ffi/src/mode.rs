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

/// Get info about a connector
pub fn get_connector(
    fd: RawFd,
    connector_id: u32,
    props: Option<&mut &mut [u32]>,
    prop_values: Option<&mut &mut [u64]>,
    mut modes: Option<&mut Vec<drm_mode_modeinfo>>,
    encoders: Option<&mut &mut [u32]>,
) -> Result<drm_mode_get_connector, Error> {
    let modes_count = if modes.is_some() {
        let mut info = drm_mode_get_connector {
            connector_id,
            ..Default::default()
        };

        unsafe {
            ioctl::mode::get_connector(fd, &mut info)?;
        }

        info.count_modes
    } else {
        0
    };

    let mut info = drm_mode_get_connector {
        encoders_ptr: map_ptr!(&encoders),
        modes_ptr: match modes.as_mut() {
            Some(modes) => {
                modes.clear();
                modes.reserve_exact(modes_count as usize);
                modes.as_ptr() as _
            }
            None => 0u64,
        },
        props_ptr: map_ptr!(&props),
        prop_values_ptr: map_ptr!(&prop_values),
        count_modes: modes_count,
        count_props: map_len!(&props),
        count_encoders: map_len!(&encoders),
        connector_id,
        ..Default::default()
    };

    unsafe {
        ioctl::mode::get_connector(fd, &mut info)?;
    }

    if let Some(modes) = modes {
        unsafe {
            modes.set_len(info.count_modes as usize);
        }
    }

    map_shrink!(props, info.count_props as usize);
    map_shrink!(prop_values, info.count_props as usize);
    map_shrink!(encoders, info.count_encoders as usize);

    Ok(info)
}

/// Get info about an encoder
pub fn get_encoder(fd: RawFd, encoder_id: u32) -> Result<drm_mode_get_encoder, Error> {
    let mut info = drm_mode_get_encoder {
        encoder_id,
        ..Default::default()
    };

    unsafe {
        ioctl::mode::get_encoder(fd, &mut info)?;
    }

    Ok(info)
}

/// Get info about a CRTC
pub fn get_crtc(fd: RawFd, crtc_id: u32) -> Result<drm_mode_crtc, Error> {
    let mut info = drm_mode_crtc {
        crtc_id,
        ..Default::default()
    };

    unsafe {
        ioctl::mode::get_crtc(fd, &mut info)?;
    }

    Ok(info)
}

/// Set CRTC state
pub fn set_crtc(
    fd: RawFd,
    crtc_id: u32,
    fb_id: u32,
    x: u32,
    y: u32,
    conns: &[u32],
    mode: Option<drm_mode_modeinfo>,
) -> Result<drm_mode_crtc, Error> {
    let mut crtc = drm_mode_crtc {
        set_connectors_ptr: conns.as_ptr() as _,
        count_connectors: conns.len() as _,
        crtc_id,
        fb_id,
        x,
        y,
        mode_valid: match mode {
            Some(_) => 1,
            None => 0,
        },
        mode: mode.unwrap_or_default(),
        ..Default::default()
    };

    unsafe {
        ioctl::mode::set_crtc(fd, &mut crtc)?;
    }

    Ok(crtc)
}

///
/// Dumbbuffers are basic buffers that can be used for scanout.
///
pub mod dumbbuffer {
    use drm_sys::*;
    use ioctl;

    use result::SystemError as Error;
    use std::os::unix::io::RawFd;

    /// Create a dumb buffer
    pub fn create(
        fd: RawFd,
        width: u32,
        height: u32,
        bpp: u32,
        flags: u32,
    ) -> Result<drm_mode_create_dumb, Error> {
        let mut db = drm_mode_create_dumb {
            height,
            width,
            bpp,
            flags,
            ..Default::default()
        };

        unsafe {
            ioctl::mode::create_dumb(fd, &mut db)?;
        }

        Ok(db)
    }

    /// Destroy a dumb buffer
    pub fn destroy(fd: RawFd, handle: u32) -> Result<drm_mode_destroy_dumb, Error> {
        let mut db = drm_mode_destroy_dumb { handle };

        unsafe {
            ioctl::mode::destroy_dumb(fd, &mut db)?;
        }

        Ok(db)
    }

    /// Map a dump buffer and prep it for an mmap
    pub fn map(fd: RawFd, handle: u32, pad: u32, offset: u64) -> Result<drm_mode_map_dumb, Error> {
        let mut map = drm_mode_map_dumb {
            handle,
            pad,
            offset,
        };

        unsafe {
            ioctl::mode::map_dumb(fd, &mut map)?;
        }

        Ok(map)
    }
}

/// Add a new framebuffer.
pub fn add_fb(
    fd: RawFd,
    width: u32,
    height: u32,
    pitch: u32,
    bpp: u32,
    depth: u32,
    handle: u32,
) -> Result<drm_mode_fb_cmd, Error> {
    let mut fb = drm_mode_fb_cmd {
        width,
        height,
        pitch,
        bpp,
        depth,
        handle,
        ..Default::default()
    };

    unsafe {
        ioctl::mode::add_fb(fd, &mut fb)?;
    }

    Ok(fb)
}

/// Remove a framebuffer.
pub fn rm_fb(fd: RawFd, mut id: u32) -> Result<(), Error> {
    unsafe {
        ioctl::mode::rm_fb(fd, &mut id)?;
    }

    Ok(())
}