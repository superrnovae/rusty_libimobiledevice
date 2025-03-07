// jkcoxson

use std::{convert::TryFrom, ffi::CStr, os::raw::c_char};

use crate::{
    bindings as unsafe_bindings, error::AfcError, idevice::Device,
    services::house_arrest::HouseArrest, services::lockdownd::LockdowndService,
};

/// Transfers files between host and the iDevice
pub struct AfcClient<'a> {
    pub(crate) pointer: unsafe_bindings::afc_client_t,
    phantom: std::marker::PhantomData<&'a Device>,
}

impl AfcClient<'_> {
    /// Creates a new afc service connection to the device
    /// The use of this function is unknown
    /// # Arguments
    /// * `device` - The device to create the service with
    /// # Returns
    /// The lockdownd service
    ///
    /// ***Verified:*** False
    pub fn new(device: &Device) -> Result<(Self, LockdowndService), String> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let mut client_pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_client_new(device.pointer, &mut pointer, &mut client_pointer)
        };
        if result != 0 {
            return Err(format!("afc_client_new failed: {}", result));
        }
        Ok((
            AfcClient {
                pointer: client_pointer,
                phantom: std::marker::PhantomData,
            },
            LockdowndService {
                pointer: &mut pointer,
                port: pointer.port as u32,
                phantom: std::marker::PhantomData,
            },
        ))
    }

    /// Starts an afc service connection to the device
    /// # Arguments
    /// * `device` - The device to create the service with
    /// * `service_name` - The name of the service to start
    /// # Returns
    /// An afc service connection
    ///
    /// ***Verified:*** False
    pub fn start_service(device: &Device, service_name: &str) -> Result<Self, AfcError> {
        let mut pointer = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_client_start_service(
                device.pointer,
                &mut pointer,
                service_name.as_ptr() as *const c_char,
            )
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(AfcClient {
            pointer,
            phantom: std::marker::PhantomData,
        })
    }

    /// Get information about the device
    /// # Arguments
    /// *none*
    /// # Returns
    /// A string containing the device information
    ///
    /// ***Verified:*** False
    pub fn get_device_info(&self) -> Result<String, AfcError> {
        let mut info = unsafe { std::mem::zeroed() };
        let mut info_ptr: *mut *mut c_char = &mut info;
        let result =
            unsafe { unsafe_bindings::afc_get_device_info(self.pointer, &mut info_ptr) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(unsafe { CStr::from_ptr(info) }
            .to_string_lossy()
            .into_owned())
    }

    /// Read a directory on the device
    /// # Arguments
    /// * `directory` - The directory to read
    /// # Returns
    /// A vector of strings containing the directory contents
    ///
    /// ***Verified:*** False
    pub fn read_directory(&self, directory: String) -> Result<String, AfcError> {
        let directory_ptr: *const c_char = directory.as_ptr() as *const c_char;
        let mut entries = unsafe { std::mem::zeroed() };
        let mut entries_ptr: *mut *mut c_char = &mut entries;
        let result = unsafe {
            unsafe_bindings::afc_read_directory(self.pointer, directory_ptr, &mut entries_ptr)
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(unsafe { CStr::from_ptr(entries) }
            .to_string_lossy()
            .into_owned())
    }

    /// Get information about a file on the device
    /// # Arguments
    /// * `path` - The path to the file
    /// # Returns
    /// A string containing the file information
    ///
    /// ***Verified:*** False
    pub fn get_file_info(&self, path: String) -> Result<String, AfcError> {
        let path_ptr: *const c_char = path.as_ptr() as *const c_char;
        let mut info = unsafe { std::mem::zeroed() };
        let mut info_ptr: *mut *mut c_char = &mut info;
        let result =
            unsafe { unsafe_bindings::afc_get_file_info(self.pointer, path_ptr, &mut info_ptr) }
                .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(unsafe { CStr::from_ptr(info) }
            .to_string_lossy()
            .into_owned())
    }

    /// Open a file on the device and return a handle to it
    /// # Arguments
    /// * `path` - The path to the file
    /// * `mode` - The mode to open the file in
    /// # Returns
    /// The file handle
    ///
    /// ***Verified:*** False
    pub fn file_open(&self, path: String, mode: AfcFileMode) -> Result<u64, AfcError> {
        let file_name_ptr: *const c_char = path.as_ptr() as *const c_char;
        let mut handle = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_file_open(self.pointer, file_name_ptr, mode.into(), &mut handle)
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(handle)
    }

    /// Closes a file on the device
    /// # Arguments
    /// * `handle` - The handle to the file
    /// # Returns
    /// An error code
    ///
    /// ***Verified:*** False
    pub fn file_close(&self, handle: u64) -> Result<(), AfcError> {
        let result = unsafe { unsafe_bindings::afc_file_close(self.pointer, handle) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Locks a file on the device
    /// # Arguments
    /// * `handle` - The handle to the file
    /// * `lock_type` - The type of lock to lock the file with
    /// # Returns
    /// An error code
    ///
    /// ***Verified:*** False
    pub fn file_lock(&self, handle: u64, lock_type: AfcLockOp) -> Result<(), AfcError> {
        let result =
            unsafe { unsafe_bindings::afc_file_lock(self.pointer, handle, lock_type.into()) }
                .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Reads out a file from the device
    /// # Arguments
    /// * `handle` - The handle to the file
    /// * `length` - The length of the data to read
    /// # Returns
    /// A vector of bytes containing the data read
    ///
    /// ***Verified:*** False
    pub fn file_read(&self, handle: u64, length: u32) -> Result<Vec<i8>, AfcError> {
        let mut buffer = unsafe { std::mem::zeroed() };
        let mut bytes_written = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_file_read(
                self.pointer,
                handle,
                &mut buffer,
                length,
                &mut bytes_written,
            )
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }

        let vec = unsafe {
            Vec::from_raw_parts(
                buffer as *mut i8,
                bytes_written as usize,
                bytes_written as usize,
            )
        };

        Ok(vec)
    }

    /// Writes data to a file on the device
    /// # Arguments
    /// * `handle` - The handle to the file
    /// * `data` - The data to write
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn file_write(&self, handle: u64, data: String) -> Result<(), AfcError> {
        let data_ptr: *const c_char = data.as_ptr() as *const c_char;
        let mut bytes_written = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_file_write(
                self.pointer,
                handle,
                data_ptr,
                data.len() as u32,
                &mut bytes_written,
            )
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Seeks for a file or something
    /// # Arguments
    /// * `handle` - The handle to the file
    /// * `offset` - Unknown
    /// * `whence` - Unknown
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn file_seek(&self, handle: u64, offset: i64, whence: u8) -> Result<(), AfcError> {
        let result =
            unsafe { unsafe_bindings::afc_file_seek(self.pointer, handle, offset, whence.into()) }
                .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Unknown usage
    /// # Arguments
    /// * `handle` - The handle to the file
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn file_tell(&self, handle: u64) -> Result<u64, AfcError> {
        let mut position = unsafe { std::mem::zeroed() };
        let result =
            unsafe { unsafe_bindings::afc_file_tell(self.pointer, handle, &mut position) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(position)
    }

    /// Truncates a file on the iOS device
    /// # Arguments
    /// * `handle` - The handle to the file
    /// * `length` - The length of which to truncate the file to
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn file_truncate(&self, handle: u64, length: u64) -> Result<(), AfcError> {
        let result =
            unsafe { unsafe_bindings::afc_file_truncate(self.pointer, handle, length) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Removes a path on the iOS device
    /// # Arguments
    /// * `path` - The path to the folder that's being removed
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn remove_path(&self, path: String) -> Result<(), AfcError> {
        let path_ptr: *const c_char = path.as_ptr() as *const c_char;
        let result = unsafe { unsafe_bindings::afc_remove_path(self.pointer, path_ptr) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Renames or moves a folder on the iOS device
    /// # Arguments
    /// * `old_path` - The path to the folder to rename
    /// * `new_path` - The destination path
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn rename_path(&self, old_path: String, new_path: String) -> Result<(), AfcError> {
        let old_path_ptr: *const c_char = old_path.as_ptr() as *const c_char;
        let new_path_ptr: *const c_char = new_path.as_ptr() as *const c_char;
        let result =
            unsafe { unsafe_bindings::afc_rename_path(self.pointer, old_path_ptr, new_path_ptr) }
                .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Creates a directory on the iOS device
    /// # Arguments
    /// * `path` - The path to create
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn make_directory(&self, path: String) -> Result<(), AfcError> {
        let path_ptr: *const c_char = path.as_ptr() as *const c_char;
        let result = unsafe { unsafe_bindings::afc_make_directory(self.pointer, path_ptr) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Usage unknown
    /// # Arguments
    /// * `handle` - The handle to the file
    /// * `length` - Unknown
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn truncate(&self, path: String, length: u64) -> Result<(), AfcError> {
        let path_ptr: *const c_char = path.as_ptr() as *const c_char;
        let result =
            unsafe { unsafe_bindings::afc_truncate(self.pointer, path_ptr, length) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Creates a symbolic link on the iOS device
    /// # Arguments
    /// * `target` - The path to the file/folder being linked
    /// * `link_type` - The type of link being created
    /// * `link_path` - The path to place the link
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn make_link(
        &self,
        target: String,
        link_type: LinkType,
        link_path: String,
    ) -> Result<(), AfcError> {
        let target_ptr: *const c_char = target.as_ptr() as *const c_char;
        let link_name_ptr: *const c_char = link_path.as_ptr() as *const c_char;
        let result = unsafe {
            unsafe_bindings::afc_make_link(
                self.pointer,
                link_type.into(),
                target_ptr,
                link_name_ptr,
            )
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Sets the time metadata of a file
    /// # Arguments
    /// * `path` - The path to the file
    /// * `mtime` - The unix epoch time in miliseconds
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn set_file_time(&self, path: String, mtime: u64) -> Result<(), AfcError> {
        let path_ptr: *const c_char = path.as_ptr() as *const c_char;
        let result =
            unsafe { unsafe_bindings::afc_set_file_time(self.pointer, path_ptr, mtime) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Removes a path and the files inside it
    /// # Arguments
    /// * `path` - The path to the folder being destroyed
    /// # Returns
    /// *none*
    ///
    /// ***Verified:*** False
    pub fn remove_path_and_contents(&self, path: String) -> Result<(), AfcError> {
        let path_ptr: *const c_char = path.as_ptr() as *const c_char;
        let result =
            unsafe { unsafe_bindings::afc_remove_path_and_contents(self.pointer, path_ptr) }.into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(())
    }

    /// Gets a specific value for a key on the device's connection
    /// # Arguments
    /// * `key` - The key of which to look up
    /// # Returns
    /// The info value of the lookup
    ///
    /// ***Verified:*** False
    pub fn get_device_info_key(&self, key: String) -> Result<String, AfcError> {
        let key_ptr: *const c_char = key.as_ptr() as *const c_char;
        let mut value_ptr = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_get_device_info_key(self.pointer, key_ptr, &mut value_ptr)
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(unsafe { CStr::from_ptr(value_ptr) }
            .to_string_lossy()
            .into_owned())
    }
}

impl TryFrom<HouseArrest<'_>> for AfcClient<'_> {
    type Error = AfcError;

    fn try_from(house_arrest: HouseArrest<'_>) -> Result<Self, Self::Error> {
        let mut to_fill = unsafe { std::mem::zeroed() };
        let result = unsafe {
            unsafe_bindings::afc_client_new_from_house_arrest_client(
                house_arrest.pointer,
                &mut to_fill,
            )
        }
        .into();
        if result != AfcError::Success {
            return Err(result);
        }
        Ok(Self {
            pointer: to_fill,
            phantom: std::marker::PhantomData,
        })
    }
}

pub enum AfcFileMode {
    ReadOnly,
    ReadWrite,
    WriteOnly,
    WriteRead,
    Append,
    ReadAppend,
}

impl From<i8> for AfcFileMode {
    fn from(mode: i8) -> Self {
        match mode {
            1 => AfcFileMode::ReadOnly,
            2 => AfcFileMode::ReadWrite,
            3 => AfcFileMode::WriteOnly,
            4 => AfcFileMode::WriteRead,
            5 => AfcFileMode::Append,
            6 => AfcFileMode::ReadAppend,
            _ => panic!("Invalid file mode"),
        }
    }
}

impl From<AfcFileMode> for u32 {
    fn from(mode: AfcFileMode) -> Self {
        match mode {
            AfcFileMode::ReadOnly => 1,
            AfcFileMode::ReadWrite => 2,
            AfcFileMode::WriteOnly => 3,
            AfcFileMode::WriteRead => 4,
            AfcFileMode::Append => 5,
            AfcFileMode::ReadAppend => 6,
        }
    }
}

pub enum AfcLockOp {
    Sh,
    Ex,
    Un,
}

impl From<AfcLockOp> for u32 {
    fn from(op: AfcLockOp) -> Self {
        match op {
            AfcLockOp::Sh => 5,
            AfcLockOp::Ex => 6,
            AfcLockOp::Un => 12,
        }
    }
}

pub enum LinkType {
    HardLink,
    SymbolicLink,
}

impl From<LinkType> for u32 {
    fn from(link_type: LinkType) -> Self {
        match link_type {
            LinkType::HardLink => 1,
            LinkType::SymbolicLink => 2,
        }
    }
}

impl Drop for AfcClient<'_> {
    fn drop(&mut self) {
        unsafe {
            unsafe_bindings::afc_client_free(self.pointer);
        }
    }
}
