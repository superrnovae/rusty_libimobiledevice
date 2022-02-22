// jkcoxson
// These are done to ensure memory safety for the programmer. 
// The libimobiledevice library does not ensure that dependencies
// for other structs are satisfied, and this could cause undefined
// behavior and segfaults. For example, a Lockdownd Service 
// required a pointer to an iDevice, but if the device is freed
// then we will have undefined behavior accessing that service.
// Simply giving the dependencies of a struct is not possible
// due to the library's design not conforming with Rust's 
// one-time mutability requirement.

use std::sync::{Mutex, Arc};

use crate::bindings as unsafe_bindings;
use unsafe_bindings::{ idevice_private, idevice_t, lockdownd_client_private, lockdownd_client_t, lockdownd_service_descriptor, lockdownd_service_descriptor_t, mobile_image_mounter_client_t, mobile_image_mounter_client_private };

pub struct IdeviceMemoryLock {
    pub pointer: Arc<Mutex<Option<idevice_private>>>,
}

impl IdeviceMemoryLock {
    pub fn new(pointer: unsafe_bindings::idevice_t) -> Self {
        unsafe { 
            IdeviceMemoryLock {
                pointer: Arc::new(Mutex::new(Some(*pointer))) 
            }
        }
        
    }

    pub fn check(&self) -> Result<idevice_t, ()> {
        match self.pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(lock) => Ok(&mut lock.clone()),
                    None => Err(()),
                }
            },
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn invalidate(&mut self) {
        self.pointer.lock().unwrap().take();
    }
}

/// Lockdownd Clients rely on devices
pub struct LockdowndClientLock {
    pub pointer: Arc<Mutex<Option<lockdownd_client_private>>>,
    pub idevice_pointer: Arc<Mutex<Option<idevice_private>>>
}

impl LockdowndClientLock {
    pub fn new(pointer: lockdownd_client_t, idevice_pointer: Arc<Mutex<Option<idevice_private>>>) -> Self {
        unsafe { 
            LockdowndClientLock {
                pointer: Arc::new(Mutex::new(Some(*pointer))),
                idevice_pointer,
            }
        }
        
    }

    /// Returns a pointer to the object if all dependencies are satisfied
    pub fn check(&self) -> Result<unsafe_bindings::lockdownd_client_t, ()> {
        match self.idevice_pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(_) => {},
                    None => { return Err(()); },
                }
            },
            Err(_) => {
                return Err(());
            }
        }
        match self.pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(lock) => Ok(&mut lock.clone()),
                    None => Err(()),
                }
            },
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn invalidate(&mut self) {
        self.pointer.lock().unwrap().take();
    }
}

pub struct LockdowndServiceLock {
    pub pointer: Arc<Mutex<Option<lockdownd_service_descriptor>>>,
    pub lockdownd_client_pointer: Arc<Mutex<Option<lockdownd_client_private>>>
}

impl LockdowndServiceLock {
    pub fn new(pointer: lockdownd_service_descriptor_t, lockdownd_client_pointer: Arc<Mutex<Option<lockdownd_client_private>>>) -> Self {
        unsafe { 
            LockdowndServiceLock {
                pointer: Arc::new(Mutex::new(Some(*pointer))),
                lockdownd_client_pointer,
            }
        }
        
    }

    /// Returns a pointer to the object if all dependencies are satisfied
    pub fn check(&self) -> Result<unsafe_bindings::lockdownd_service_descriptor_t, ()> {
        match self.lockdownd_client_pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(_) => {},
                    None => { return Err(()); },
                }
            },
            Err(_) => {
                return Err(());
            }
        }
        match self.pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(lock) => Ok(&mut lock.clone()),
                    None => Err(()),
                }
            },
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn invalidate(&mut self) {
        self.pointer.lock().unwrap().take();
    }
}

pub struct MobileImageMounterLock {
    pub pointer: Arc<Mutex<Option<mobile_image_mounter_client_private>>>,
    pub service_pointer: Arc<Mutex<Option<lockdownd_service_descriptor>>>,
}

impl MobileImageMounterLock {
    pub fn new(pointer: mobile_image_mounter_client_t, service_pointer: lockdownd_service_descriptor_t) -> Self {
        unsafe { 
            MobileImageMounterLock {
                pointer: Arc::new(Mutex::new(Some(*pointer))),
                service_pointer: Arc::new(Mutex::new(Some(*service_pointer))),
            }
        }
        
    }

    /// Returns a pointer to the object if all dependencies are satisfied
    pub fn check(&self) -> Result<mobile_image_mounter_client_t, ()> {
        match self.service_pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(_) => {},
                    None => { return Err(()); },
                }
            },
            Err(_) => {
                return Err(());
            }
        }
        match self.pointer.lock() {
            Ok(lock) => {
                match *lock {
                    Some(lock) => Ok(&mut lock.clone()),
                    None => Err(()),
                }
            },
            Err(_) => {
                Err(())
            }
        }
    }

    pub fn invalidate(&mut self) {
        self.pointer.lock().unwrap().take();
    }
}