//! Modesetting operations that the DRM subsystem exposes.
//!
//! # Summary
//!
//! The DRM subsystem provides Kernel Modesetting (KMS) functionality by
//! exposing the following resource types:
//!
//! * FrameBuffer - Specific to an individual process, these wrap around generic
//! GPU buffers so that they can be attached to a Plane.
//!
//! * Planes - Dedicated memory objects which contain a buffer that can then be
//! scanned out by a CRTC. There exist a few different types of planes depending
//! on the use case.
//!
//! * CRTC - Scanout engines that read pixel data from a Plane and sends it to
//! a Connector. Each CRTC has at least one Primary Plane.
//!
//! * Connector - Represents the physical output, such as a DisplayPort or
//! VGA connector.
//!
//! * Encoder - Encodes pixel data from a CRTC into something a Connector can
//! understand.
//!
//! Further details on each resource can be found in their respective modules.
//!
//! # Usage
//!
//! To begin using modesetting functionality, the [`Device`] trait
//! must be implemented on top of the basic [`super::Device`] trait.

use drm_ffi as ffi;
use drm_ffi::result::SystemError;
pub mod connector;
pub mod crtc;
pub mod dumbbuffer;
pub mod encoder;
pub mod framebuffer;

// pub mod property;

use self::dumbbuffer::*;
use buffer;

use std::mem;

use core::num::NonZeroU32;

/// Raw handle for a drm resource
pub type RawResourceHandle = NonZeroU32;

/// Handle for a drm resource
pub trait ResourceHandle:
    From<RawResourceHandle> + Into<RawResourceHandle> + Into<u32> + Copy + Sized
{
    /// Associated encoded object type
    const FFI_TYPE: u32;
}

/// Convert from a raw drm object value to a typed Handle
///
/// Note: This does no verification on the validity of the original value
pub fn from_u32<T: ResourceHandle>(raw: u32) -> Option<T> {
    RawResourceHandle::new(raw).map(T::from)
}

/// This trait should be implemented by any object that acts as a DRM device and
/// provides modesetting functionality.
///
/// Like the parent [`super::Device`] trait, this crate does not
/// provide a concrete object for this trait.
///
/// # Example
/// ```ignore
/// use drm::control::Device as ControlDevice;
///
/// /// Assuming the [`Card`] wrapper already implements [`drm::Device`]
/// impl ControlDevice for Card {}
/// ```
pub trait Device: super::Device {
    /// Gets the set of resource handles that this device currently controls
    fn resource_handles(&self) -> Result<ResourceHandles, SystemError> {
        let mut fbs = [0u32; 32];
        let mut crtcs = [0u32; 32];
        let mut connectors = [0u32; 32];
        let mut encoders = [0u32; 32];

        let mut fb_slice = &mut fbs[..];
        let mut crtc_slice = &mut crtcs[..];
        let mut conn_slice = &mut connectors[..];
        let mut enc_slice = &mut encoders[..];

        let ffi_res = ffi::mode::get_resources(
            self.as_raw_fd(),
            Some(&mut fb_slice),
            Some(&mut crtc_slice),
            Some(&mut conn_slice),
            Some(&mut enc_slice),
        )?;

        let fb_len = fb_slice.len();
        let crtc_len = crtc_slice.len();
        let conn_len = conn_slice.len();
        let enc_len = enc_slice.len();

        let res = ResourceHandles {
            fbs: unsafe { mem::transmute(fbs) },
            fb_len,
            crtcs: unsafe { mem::transmute(crtcs) },
            crtc_len,
            connectors: unsafe { mem::transmute(connectors) },
            conn_len,
            encoders: unsafe { mem::transmute(encoders) },
            enc_len,
            width: (ffi_res.min_width, ffi_res.max_width),
            height: (ffi_res.min_height, ffi_res.max_height),
        };

        Ok(res)
    }

    /// Returns information about a specific connector
    fn get_connector(&self, handle: connector::Handle) -> Result<connector::Info, SystemError> {
        // Maximum number of encoders is 3 due to kernel restrictions
        let mut encoders = [0u32; 3];
        let mut enc_slice = &mut encoders[..];
        let mut modes = Vec::new();

        let ffi_info = ffi::mode::get_connector(
            self.as_raw_fd(),
            handle.into(),
            None,
            None,
            Some(&mut modes),
            Some(&mut enc_slice),
        )?;

        let connector = connector::Info {
            handle,
            interface: connector::Interface::from(ffi_info.connector_type),
            interface_id: ffi_info.connector_type_id,
            connection: connector::State::from(ffi_info.connection),
            size: match (ffi_info.mm_width, ffi_info.mm_height) {
                (0, 0) => None,
                (x, y) => Some((x, y)),
            },
            modes: unsafe { mem::transmute(modes) },
            encoders: unsafe { mem::transmute(encoders) },
            curr_enc: unsafe { mem::transmute(ffi_info.encoder_id) },
        };

        Ok(connector)
    }

    /// Returns information about a specific encoder
    fn get_encoder(&self, handle: encoder::Handle) -> Result<encoder::Info, SystemError> {
        let info = ffi::mode::get_encoder(self.as_raw_fd(), handle.into())?;

        let enc = encoder::Info {
            handle,
            enc_type: encoder::Kind::from(info.encoder_type),
            crtc: from_u32(info.crtc_id),
            pos_crtcs: info.possible_crtcs,
            pos_clones: info.possible_clones,
        };

        Ok(enc)
    }

    /// Returns information about a specific CRTC
    fn get_crtc(&self, handle: crtc::Handle) -> Result<crtc::Info, SystemError> {
        let info = ffi::mode::get_crtc(self.as_raw_fd(), handle.into())?;

        let crtc = crtc::Info {
            handle,
            position: (info.x, info.y),
            mode: match info.mode_valid {
                0 => None,
                _ => Some(Mode::from(info.mode)),
            },
            fb: from_u32(info.fb_id),
            gamma_length: info.gamma_size,
        };

        Ok(crtc)
    }

    /// Set CRTC state
    fn set_crtc(
        &self,
        handle: crtc::Handle,
        framebuffer: Option<framebuffer::Handle>,
        pos: (u32, u32),
        conns: &[connector::Handle],
        mode: Option<Mode>,
    ) -> Result<(), SystemError> {
        let _info = ffi::mode::set_crtc(
            self.as_raw_fd(),
            handle.into(),
            framebuffer.map(|x| x.into()).unwrap_or(0),
            pos.0,
            pos.1,
            unsafe { &*(conns as *const _ as *const [u32]) },
            unsafe { mem::transmute(mode) },
        )?;

        Ok(())
    }

    /// Create a new dumb buffer with a given size and pixel format
    fn create_dumb_buffer(
        &self,
        size: (u32, u32),
        format: buffer::DrmFourcc,
        bpp: u32,
    ) -> Result<DumbBuffer, SystemError> {
        let info = drm_ffi::mode::dumbbuffer::create(self.as_raw_fd(), size.0, size.1, bpp, 0)?;

        let dumb = DumbBuffer {
            size: (info.width, info.height),
            length: info.size as usize,
            format,
            pitch: info.pitch,
            handle: unsafe { mem::transmute(info.handle) },
        };

        Ok(dumb)
    }

    /// Map the buffer for access
    fn map_dumb_buffer<'a>(
        &self,
        buffer: &'a mut DumbBuffer,
    ) -> Result<DumbMapping<'a>, SystemError> {
        let info = drm_ffi::mode::dumbbuffer::map(self.as_raw_fd(), buffer.handle.into(), 0, 0)?;

        let map = {
            use nix::sys::mman;
            let addr = core::ptr::null_mut();
            let prot = mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE;
            let flags = mman::MapFlags::MAP_SHARED;
            let length = buffer.length;
            let fd = self.as_raw_fd();
            let offset = info.offset as _;
            unsafe { mman::mmap(addr, length, prot, flags, fd, offset)? }
        };

        let mapping = DumbMapping {
            _phantom: ::std::marker::PhantomData,
            map: unsafe { ::std::slice::from_raw_parts_mut(map as *mut _, buffer.length) },
        };

        Ok(mapping)
    }

    /// Destroy a framebuffer
    fn destroy_framebuffer(&self, handle: framebuffer::Handle) -> Result<(), SystemError> {
        ffi::mode::rm_fb(self.as_raw_fd(), handle.into())
    }

    /// Free the memory resources of a dumb buffer
    fn destroy_dumb_buffer(&self, buffer: DumbBuffer) -> Result<(), SystemError> {
        let _info = drm_ffi::mode::dumbbuffer::destroy(self.as_raw_fd(), buffer.handle.into())?;

        Ok(())
    }

    /// Add a new framebuffer
    fn add_framebuffer<B>(
        &self,
        buffer: &B,
        depth: u32,
        bpp: u32,
    ) -> Result<framebuffer::Handle, SystemError>
    where
        B: buffer::Buffer + ?Sized,
    {
        let (w, h) = buffer.size();
        let info = ffi::mode::add_fb(
            self.as_raw_fd(),
            w,
            h,
            buffer.pitch(),
            bpp,
            depth,
            buffer.handle().into(),
        )?;

        Ok(unsafe { mem::transmute(info.fb_id) })
    }

}

/// The set of [`ResourceHandles`] that a
/// [`Device`] exposes. Excluding Plane resources.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ResourceHandles {
    fbs: [Option<framebuffer::Handle>; 32],
    fb_len: usize,
    crtcs: [Option<crtc::Handle>; 32],
    crtc_len: usize,
    connectors: [Option<connector::Handle>; 32],
    conn_len: usize,
    encoders: [Option<encoder::Handle>; 32],
    enc_len: usize,
    width: (u32, u32),
    height: (u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A filter that can be used with a [`ResourceHandles`] to determine the set of
/// Crtcs that can attach to a specific encoder.
pub struct CrtcListFilter(u32);

impl ResourceHandles {
    /// Returns the set of [`connector::Handle`]
    pub fn connectors(&self) -> &[connector::Handle] {
        let buf_len = std::cmp::min(self.connectors.len(), self.conn_len);
        unsafe { &*(&self.connectors[..buf_len] as *const _ as *const [connector::Handle]) }
    }

    /// Returns the set of [`encoder::Handle`]
    pub fn encoders(&self) -> &[encoder::Handle] {
        let buf_len = std::cmp::min(self.encoders.len(), self.enc_len);
        unsafe { &*(&self.encoders[..buf_len] as *const _ as *const [encoder::Handle]) }
    }

    /// Returns the set of [`crtc::Handle`]
    pub fn crtcs(&self) -> &[crtc::Handle] {
        let buf_len = std::cmp::min(self.crtcs.len(), self.crtc_len);
        unsafe { &*(&self.crtcs[..buf_len] as *const _ as *const [crtc::Handle]) }
    }

    /// Returns the set of [`framebuffer::Handle`]
    pub fn framebuffers(&self) -> &[framebuffer::Handle] {
        let buf_len = std::cmp::min(self.fbs.len(), self.fb_len);
        unsafe { &*(&self.fbs[..buf_len] as *const _ as *const [framebuffer::Handle]) }
    }

    /// Apply a filter the all crtcs of these resources, resulting in a list of crtcs allowed.
    pub fn filter_crtcs(&self, filter: CrtcListFilter) -> Vec<crtc::Handle> {
        self.crtcs
            .iter()
            .enumerate()
            .filter(|&(n, _)| (1 << n) & filter.0 != 0)
            .flat_map(|(_, &e)| e)
            .collect()
    }
}

impl std::fmt::Debug for ResourceHandles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ResourceHandles")
            .field("fbs", &self.framebuffers())
            .field("crtcs", &self.crtcs())
            .field("connectors", &self.connectors())
            .field("encoders", &self.encoders())
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

/// Resolution and timing information for a display mode.
#[repr(transparent)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Mode {
    // We're using the FFI struct because the DRM API expects it when giving it
    // to a CRTC or creating a blob from it. Rather than rearranging the fields
    // to convert to/from an abstracted type, just use the raw object.
    mode: ffi::drm_mode_modeinfo,
}

impl Mode {
    /// Returns the name of this mode.
    pub fn name(&self) -> &std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr(&self.mode.name[0] as _) }
    }

    /// Returns the clock speed of this mode.
    pub fn clock(&self) -> u32 {
        self.mode.clock
    }

    /// Returns the size (resolution) of the mode.
    pub fn size(&self) -> (u16, u16) {
        (self.mode.hdisplay, self.mode.vdisplay)
    }

    /// Returns the horizontal sync start, end, and total.
    pub fn hsync(&self) -> (u16, u16, u16) {
        (self.mode.hsync_start, self.mode.hsync_end, self.mode.htotal)
    }

    /// Returns the vertical sync start, end, and total.
    pub fn vsync(&self) -> (u16, u16, u16) {
        (self.mode.vsync_start, self.mode.vsync_end, self.mode.vtotal)
    }

    /// Returns the horizontal skew of this mode.
    pub fn hskew(&self) -> u16 {
        self.mode.hskew
    }

    /// Returns the vertical scan of this mode.
    pub fn vscan(&self) -> u16 {
        self.mode.vscan
    }

    /// Returns the vertical refresh rate of this mode
    pub fn vrefresh(&self) -> u32 {
        self.mode.vrefresh
    }

    /// Returns the bitmask of this mode
    pub fn mode_type(&self) -> ModeTypeFlags {
        ModeTypeFlags::from_bits_truncate(self.mode.type_)
    }
}

impl From<ffi::drm_mode_modeinfo> for Mode {
    fn from(raw: ffi::drm_mode_modeinfo) -> Mode {
        Mode { mode: raw }
    }
}

impl From<Mode> for ffi::drm_mode_modeinfo {
    fn from(mode: Mode) -> Self {
        mode.mode
    }
}

impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Mode")
            .field("name", &self.name())
            .field("clock", &self.clock())
            .field("size", &self.size())
            .field("hsync", &self.hsync())
            .field("vsync", &self.vsync())
            .field("hskew", &self.hskew())
            .field("vscan", &self.vscan())
            .field("vrefresh", &self.vrefresh())
            .field("mode_type", &self.mode_type())
            .finish()
    }
}

bitflags::bitflags! {
    /// Display mode type flags
    pub struct ModeTypeFlags : u32 {
        /// Builtin mode type
        #[deprecated]
        const BUILTIN = ffi::DRM_MODE_TYPE_BUILTIN;
        /// CLOCK_C mode type
        #[deprecated]
        const CLOCK_C = ffi::DRM_MODE_TYPE_CLOCK_C;
        /// CRTC_C mode type
        #[deprecated]
        const CRTC_C = ffi::DRM_MODE_TYPE_CRTC_C;
        /// Preferred mode
        const PREFERRED = ffi::DRM_MODE_TYPE_PREFERRED;
        /// Default mode
        #[deprecated]
        const DEFAULT = ffi::DRM_MODE_TYPE_DEFAULT;
        /// User defined mode type
        const USERDEF = ffi::DRM_MODE_TYPE_USERDEF;
        /// Mode created by driver
        const DRIVER = ffi::DRM_MODE_TYPE_DRIVER;
        /// Bitmask of all valid (non-deprecated) mode type flags
        const ALL = ffi::DRM_MODE_TYPE_ALL;
    }
}