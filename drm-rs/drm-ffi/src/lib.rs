//!
//! Foreign function interface
//!

#![warn(missing_docs)]
#![allow(unused_doc_comments)]
pub extern crate drm_sys;
pub use drm_sys::*;

extern crate nix;

#[macro_use]
pub(crate) mod utils;

use result::SystemError as Error;
pub mod ioctl;
pub mod mode;
pub mod result;

use std::os::unix::io::RawFd;

///
/// Bindings to the methods of authentication the DRM provides.
///
pub mod auth {
    use ioctl;

    use nix::Error;
    use std::os::unix::io::RawFd;

    /// Acquire the 'Master DRM Lock' for this file descriptor.
    pub fn acquire_master(fd: RawFd) -> Result<(), Error> {
        unsafe {
            ioctl::acquire_master(fd)?;
        }

        Ok(())
    }

    /// Release the 'Master DRM Lock' for this file descriptor.
    pub fn release_master(fd: RawFd) -> Result<(), Error> {
        unsafe {
            ioctl::release_master(fd)?;
        }

        Ok(())
    }
}

/// Load this device's Bus ID into a buffer.
///
/// If the buffer is too small, this will load the maximum bytes in the buffer.
/// If the buffer is too big, this will coerce the buffer to the proper size.
pub fn get_bus_id(fd: RawFd, buf: Option<&mut &mut [u8]>) -> Result<drm_unique, Error> {
    let mut busid = drm_unique {
        unique_len: map_len!(&buf),
        unique: map_ptr!(&buf),
    };

    unsafe {
        ioctl::get_bus_id(fd, &mut busid)?;
    }

    map_shrink!(buf, busid.unique_len as usize);

    Ok(busid)
}

/// Check if a capability is set.
pub fn get_capability(fd: RawFd, cty: u64) -> Result<drm_get_cap, Error> {
    let mut cap = drm_get_cap {
        capability: cty,
        ..Default::default()
    };

    unsafe {
        ioctl::get_cap(fd, &mut cap)?;
    }

    Ok(cap)
}

/// Attempt to enable/disable a client's capability.
pub fn set_capability(fd: RawFd, cty: u64, val: bool) -> Result<drm_set_client_cap, Error> {
    let cap = drm_set_client_cap {
        capability: cty,
        value: val as u64,
    };

    unsafe {
        ioctl::set_cap(fd, &cap)?;
    }

    Ok(cap)
}

/// Gets the driver version for this device.
///
/// If any buffer is too small, this will load the maximum bytes in the buffer.
/// If any buffer is too big, this will coerce the buffer to the proper size.
pub fn get_version(
    fd: RawFd,
    name_buf: Option<&mut &mut [i8]>,
    date_buf: Option<&mut &mut [i8]>,
    desc_buf: Option<&mut &mut [i8]>,
) -> Result<drm_version, Error> {
    let mut version = drm_version {
        name_len: map_len!(&name_buf),
        name: map_ptr!(&name_buf),
        date_len: map_len!(&date_buf),
        date: map_ptr!(&date_buf),
        desc_len: map_len!(&desc_buf),
        desc: map_ptr!(&desc_buf),
        ..Default::default()
    };

    unsafe {
        ioctl::get_version(fd, &mut version)?;
    }

    map_shrink!(name_buf, version.name_len as usize);
    map_shrink!(date_buf, version.date_len as usize);
    map_shrink!(desc_buf, version.desc_len as usize);

    Ok(version)
}