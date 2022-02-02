#![allow(missing_docs)]

use drm_sys::*;
use nix::ioctl_none;
use nix::ioctl_readwrite;
use nix::ioctl_write_ptr;

// Gets the bus ID of the device
// # Locks DRM mutex: Yes
// # Permissions: None
// # Nodes: Primary
ioctl_readwrite!(get_bus_id, DRM_IOCTL_BASE, 0x01, drm_unique);

// Acquires the DRM Master lock
// # Locks DRM mutex: No
// # Permissions: Root
// # Nodes: Primary
ioctl_none!(acquire_master, DRM_IOCTL_BASE, 0x1e);

//Drops the DRM Master lock
// # Locks DRM mutex: No
// # Permissions: Root
// # Nodes: Primary
ioctl_none!(release_master, DRM_IOCTL_BASE, 0x1f);

//Get capabilities of the device.
// # Locks DRM mutex: No
// # Permissions: None
// # Nodes: Primary, Render
ioctl_readwrite!(get_cap, DRM_IOCTL_BASE, 0x0c, drm_get_cap);

//Tells the device we understand a capability
// # Locks DRM mutex: Yes
// # Permissions: None
// # Nodes: Primary
ioctl_write_ptr!(set_cap, DRM_IOCTL_BASE, 0x0d, drm_set_client_cap);

//Gets the current interface version
// # Locks DRM mutex: No
// # Permissions: None
// # Nodes: All
ioctl_readwrite!(get_version, DRM_IOCTL_BASE, 0x00, drm_version);

pub(crate) mod mode {
    use drm_sys::*;
    use nix::libc::c_uint;
    use nix::ioctl_readwrite;

    //Modesetting resources
    ioctl_readwrite!(get_resources, DRM_IOCTL_BASE, 0xA0, drm_mode_card_res);

    /// Connector related functions
    ioctl_readwrite!(get_connector, DRM_IOCTL_BASE, 0xA7, drm_mode_get_connector);

    /// Encoder related functions
    ioctl_readwrite!(get_encoder, DRM_IOCTL_BASE, 0xA6, drm_mode_get_encoder);

    /// CRTC related functions
    ioctl_readwrite!(get_crtc, DRM_IOCTL_BASE, 0xA1, drm_mode_crtc);
    ioctl_readwrite!(set_crtc, DRM_IOCTL_BASE, 0xA2, drm_mode_crtc);


    /// Dumbbuffer related functions
    ioctl_readwrite!(create_dumb, DRM_IOCTL_BASE, 0xB2, drm_mode_create_dumb);

    ioctl_readwrite!(map_dumb, DRM_IOCTL_BASE, 0xB3, drm_mode_map_dumb);
    
    ioctl_readwrite!(destroy_dumb, DRM_IOCTL_BASE, 0xB4, drm_mode_destroy_dumb);

    /// FB related functions
    ioctl_readwrite!(get_fb, DRM_IOCTL_BASE, 0xAD, drm_mode_fb_cmd);
    ioctl_readwrite!(get_fb2, DRM_IOCTL_BASE, 0xCE, drm_mode_fb_cmd2);
    ioctl_readwrite!(add_fb, DRM_IOCTL_BASE, 0xAE, drm_mode_fb_cmd);
    ioctl_readwrite!(add_fb2, DRM_IOCTL_BASE, 0xB8, drm_mode_fb_cmd2);
    ioctl_readwrite!(rm_fb, DRM_IOCTL_BASE, 0xAF, c_uint);
}