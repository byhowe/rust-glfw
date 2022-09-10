use core::ffi::c_int;
use glfw_sys::glfwGetVersion;

/// Version information about glfw.
#[derive(Debug, Clone, Copy)]
pub struct Version {
    major: c_int,
    minor: c_int,
    rev: c_int,
    vendored: bool,
}

impl Version {
    /// Returns a [`Version`] which provides information about glfw.
    pub fn get() -> Self {
        let mut v = Self {
            major: 0,
            minor: 0,
            rev: 0,
            vendored: glfw_sys::vendored(),
        };

        unsafe {
            glfwGetVersion(&mut v.major, &mut v.minor, &mut v.rev);
        }

        v
    }

    /// Returns the version of glfw.
    ///
    /// The return value is tuple of `(major, minor, rev)`.
    pub fn glfw_version(&self) -> (u32, u32, u32) {
        (self.major as u32, self.minor as u32, self.rev as u32)
    }

    /// Returns `true` if `glfw-sys` was built with vendored copy of glfw.
    pub fn vendored(&self) -> bool {
        self.vendored
    }
}
