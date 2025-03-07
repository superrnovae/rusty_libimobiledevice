// jkcoxson

use std::os::raw::{c_char, c_uint};

use crate::{
    bindings as unsafe_bindings, error::SbservicesError, idevice::Device,
    services::lockdownd::LockdowndService,
};

use plist_plus::Plist;

/// A service to manage Springboard on iOS
pub struct SpringboardServicesClient<'a> {
    pub(crate) pointer: unsafe_bindings::sbservices_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl SpringboardServicesClient<'_> {
    /// Creates a preboard client from a springboard service
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `descriptor` - The lockdown service to connect on
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device, descriptor: LockdowndService) -> Result<Self, SbservicesError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::sbservices_client_new(device.pointer, descriptor.pointer, &mut pointer)
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Starts a new connection and adds a springboard service client to it
    /// # Arguments
    /// * `device` - The device to connect to
    /// * `label` - The label for the connection
    /// # Returns
    /// A struct containing the handle to the connection
    ///
    /// ***Verified:*** False
    pub fn start_service(device: &Device, label: String) -> Result<Self, SbservicesError> {
        let mut pointer = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::sbservices_client_start_service(
                device.pointer,
                &mut pointer,
                label.as_ptr() as *const c_char,
            )
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        Ok(Self {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Gets the icon states on the device
    /// # Arguments
    /// * `format_version` - Usage unknown. Not needed for iOS <4.0
    /// # Returns
    /// A plist with the icon state
    ///
    /// ***Verified:*** False
    pub fn get_icon_state(&self, format_version: Option<String>) -> Result<Plist, SbservicesError> {
        let mut plist = std::ptr::null_mut();
        let result = unsafe {
            unsafe_bindings::sbservices_get_icon_state(
                self.pointer,
                &mut plist,
                format_version
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()) as *mut c_char,
            )
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        Ok(plist.into())
    }

    /// Sets the icon state on the homescreen
    /// # Arguments
    /// * `state` - The state of the icons as a plist
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn set_icon_state(&self, state: Plist) -> Result<(), SbservicesError> {
        let result = unsafe {
            unsafe_bindings::sbservices_set_icon_state(self.pointer, state.get_pointer())
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        Ok(())
    }

    /// Get the icon of an app
    /// # Arguments
    /// * `bundle_id` - The bundle ID of the app to take the icon from
    /// # Returns
    /// A vector of bytes containing the .png
    ///
    /// ***Verified:*** False
    pub fn get_icon_png_data(&self, bundle_id: String) -> Result<Vec<c_char>, SbservicesError> {
        let mut data = std::ptr::null_mut();
        let mut size = 0;
        let result = unsafe {
            unsafe_bindings::sbservices_get_icon_pngdata(
                self.pointer,
                bundle_id.as_ptr() as *const c_char,
                &mut data,
                &mut size,
            )
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        let mut vec = Vec::with_capacity(size as usize);
        unsafe {
            std::ptr::copy_nonoverlapping(data, vec.as_mut_ptr(), size as usize);
        }

        Ok(vec)
    }

    /// Gets the orientation of the device
    /// # Arguments
    /// *none*
    /// # Returns
    /// An orientation
    ///
    /// ***Verified:*** False
    pub fn get_interface_orientation(&self) -> Result<Orientation, SbservicesError> {
        let mut orientation: c_uint = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::sbservices_get_interface_orientation(self.pointer, &mut orientation)
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        Ok(orientation.into())
    }

    /// Gets the wallpaper of the homescreen
    /// # Arguments
    /// *none*
    /// # Returns
    /// A vector of bytes containing the .png
    ///
    /// ***Verified:*** False
    pub fn get_home_screen_wallpaper_pngdata(&self) -> Result<Vec<c_char>, SbservicesError> {
        let mut data = std::ptr::null_mut();
        let mut size = 0;
        let result = unsafe {
            unsafe_bindings::sbservices_get_home_screen_wallpaper_pngdata(
                self.pointer,
                &mut data,
                &mut size,
            )
        }
        .into();

        if result != SbservicesError::Success {
            return Err(result);
        }

        let mut vec = Vec::with_capacity(size as usize);
        unsafe {
            std::ptr::copy_nonoverlapping(data, vec.as_mut_ptr(), size as usize);
        }

        Ok(vec)
    }
}

/// A device orientation
pub enum Orientation {
    Unknown,
    Portrait,
    PortraitUpsideDown,
    LandscapeRight,
    LandscapeLeft,
}

impl From<Orientation> for c_uint {
    fn from(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Unknown => 0,
            Orientation::Portrait => 1,
            Orientation::PortraitUpsideDown => 2,
            Orientation::LandscapeRight => 3,
            Orientation::LandscapeLeft => 4,
        }
    }
}

impl From<c_uint> for Orientation {
    fn from(orientation: c_uint) -> Self {
        match orientation {
            0 => Orientation::Unknown,
            1 => Orientation::Portrait,
            2 => Orientation::PortraitUpsideDown,
            3 => Orientation::LandscapeRight,
            4 => Orientation::LandscapeLeft,
            _ => panic!("Unknown orientation: {}", orientation),
        }
    }
}

impl Drop for SpringboardServicesClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::sbservices_client_free(self.pointer);
        }
    }
}
