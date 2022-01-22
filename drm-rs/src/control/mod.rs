
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
}