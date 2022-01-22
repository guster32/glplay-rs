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