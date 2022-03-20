//! OS-Specific DRM constants

// DRM major value.

// #[cfg(all(target_os = "openbsd", not(target_arch = "i386")))]
// pub const DRM_MAJOR: u64 = 87;


// DRM node prefixes

// #[cfg(not(target_os = "openbsd"))]
pub const PRIMARY_NAME: &str = "card";

// #[cfg(not(target_os = "openbsd"))]
pub const CONTROL_NAME: &str = "controlD";

// #[cfg(not(target_os = "openbsd"))]
pub const RENDER_NAME: &str = "renderD";

