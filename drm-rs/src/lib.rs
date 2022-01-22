extern crate drm_ffi;

extern crate drm_fourcc;
extern crate nix;

pub(crate) mod util;

pub mod control;

use std::os::unix::io::AsRawFd;

pub use drm_ffi::result::SystemError;
use util::*;

pub trait Device: AsRawFd {
    /// Acquires the DRM Master lock for this process.
    ///
    /// # Notes
    ///
    /// Acquiring the DRM Master is done automatically when the primary device
    /// node is opened. If you opened the primary device node and did not
    /// acquire the lock, another process likely has the lock.
    ///
    /// This function is only available to processes with CAP_SYS_ADMIN
    /// privileges (usually as root)
    fn acquire_master_lock(&self) -> Result<(), SystemError> {
        drm_ffi::auth::acquire_master(self.as_raw_fd())?;
        Ok(())
    }

    /// Releases the DRM Master lock for another process to use.
    fn release_master_lock(&self) -> Result<(), SystemError> {
        drm_ffi::auth::release_master(self.as_raw_fd())?;
        Ok(())
    }

    /// Requests the driver to expose or hide certain capabilities. See
    /// [`ClientCapability`] for more information.
    fn set_client_capability(
        &self,
        cap: ClientCapability,
        enable: bool,
    ) -> Result<(), SystemError> {
        drm_ffi::set_capability(self.as_raw_fd(), cap as u64, enable)?;
        Ok(())
    }

    /// Gets the [`BusID`] of this device.
    fn get_bus_id(&self) -> Result<BusID, SystemError> {
        let mut buffer = [0u8; 32];

        let buffer_len;

        let _busid = {
            let mut slice = &mut buffer[..];
            let busid = drm_ffi::get_bus_id(self.as_raw_fd(), Some(&mut slice))?;
            buffer_len = slice.len();
            busid
        };

        let bus_id = BusID(SmallOsString::from_u8_buffer(buffer, buffer_len));

        Ok(bus_id)
    }

    /// Gets the value of a capability.
    fn get_driver_capability(&self, cap: DriverCapability) -> Result<u64, SystemError> {
        let cap = drm_ffi::get_capability(self.as_raw_fd(), cap as u64)?;
        Ok(cap.value)
    }

    /// # Possible errors:
    ///   - [`SystemError::MemoryFault`]: Kernel could not copy fields into userspace
    #[allow(missing_docs)]
    fn get_driver(&self) -> Result<Driver, SystemError> {
        let mut name = [0i8; 32];
        let mut date = [0i8; 32];
        let mut desc = [0i8; 32];

        let name_len;
        let date_len;
        let desc_len;

        let _version = {
            let mut name_slice = &mut name[..];
            let mut date_slice = &mut date[..];
            let mut desc_slice = &mut desc[..];

            let version = drm_ffi::get_version(
                self.as_raw_fd(),
                Some(&mut name_slice),
                Some(&mut date_slice),
                Some(&mut desc_slice),
            )?;

            name_len = name_slice.len();
            date_len = date_slice.len();
            desc_len = desc_slice.len();

            version
        };

        let name = SmallOsString::from_i8_buffer(name, name_len);
        let date = SmallOsString::from_i8_buffer(date, date_len);
        let desc = SmallOsString::from_i8_buffer(desc, desc_len);

        let driver = Driver { name, date, desc };

        Ok(driver)
    }
}

/// Bus ID of a device.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BusID(SmallOsString);

impl AsRef<OsStr> for BusID {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

/// Driver version of a device.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Driver {
    name: SmallOsString,
    date: SmallOsString,
    desc: SmallOsString,
}

impl Driver {
    /// Name of driver
    pub fn name(&self) -> &OsStr {
        self.name.as_ref()
    }

    /// Date driver was published
    pub fn date(&self) -> &OsStr {
        self.date.as_ref()
    }

    /// Driver description
    pub fn description(&self) -> &OsStr {
        self.desc.as_ref()
    }
}

/// Used to check which capabilities your graphics driver has.
#[allow(clippy::upper_case_acronyms)]
#[repr(u64)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DriverCapability {
    /// DumbBuffer support for scanout
    DumbBuffer = drm_ffi::DRM_CAP_DUMB_BUFFER as u64,
    /// Unknown
    VBlankHighCRTC = drm_ffi::DRM_CAP_VBLANK_HIGH_CRTC as u64,
    /// Preferred depth to use for dumb buffers
    DumbPreferredDepth = drm_ffi::DRM_CAP_DUMB_PREFERRED_DEPTH as u64,
    /// Unknown
    DumbPreferShadow = drm_ffi::DRM_CAP_DUMB_PREFER_SHADOW as u64,
    /// PRIME handles are supported
    Prime = drm_ffi::DRM_CAP_PRIME as u64,
    /// Unknown
    MonotonicTimestamp = drm_ffi::DRM_CAP_TIMESTAMP_MONOTONIC as u64,
    /// Asynchronous page flipping support
    ASyncPageFlip = drm_ffi::DRM_CAP_ASYNC_PAGE_FLIP as u64,
    /// Width of cursor buffers
    CursorWidth = drm_ffi::DRM_CAP_CURSOR_WIDTH as u64,
    /// Height of cursor buffers
    CursorHeight = drm_ffi::DRM_CAP_CURSOR_HEIGHT as u64,
    /// Create framebuffers with modifiers
    AddFB2Modifiers = drm_ffi::DRM_CAP_ADDFB2_MODIFIERS as u64,
    /// Unknown
    PageFlipTarget = drm_ffi::DRM_CAP_PAGE_FLIP_TARGET as u64,
    /// Uses the CRTC's ID in vblank events
    CRTCInVBlankEvent = drm_ffi::DRM_CAP_CRTC_IN_VBLANK_EVENT as u64,
    /// SyncObj support
    SyncObj = drm_ffi::DRM_CAP_SYNCOBJ as u64,
}

/// Used to enable/disable capabilities for the process.
#[repr(u64)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ClientCapability {
    /// The driver provides 3D screen control
    Stereo3D = drm_ffi::DRM_CLIENT_CAP_STEREO_3D as u64,
    /// The driver provides more plane types for modesetting
    UniversalPlanes = drm_ffi::DRM_CLIENT_CAP_UNIVERSAL_PLANES as u64,
    /// The driver provides atomic modesetting
    Atomic = drm_ffi::DRM_CLIENT_CAP_ATOMIC as u64,
}
