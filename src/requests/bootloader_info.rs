use core::{cell::UnsafeCell, ffi::CStr, ptr::NonNull};

use crate::sys;

#[repr(C)]
pub struct BootloaderInfoRequest {
    id: [u64; 4],
    revision: u64,
    response: UnsafeCell<Option<BootloaderInfoResponse>>,
}

unsafe impl Sync for BootloaderInfoRequest {}
unsafe impl Send for BootloaderInfoRequest {}

impl BootloaderInfoRequest {
    pub const fn new() -> Self {
        Self {
            id: [
                0xc7b1dd30df4c8b88,
                0x0a82e883a194f07b,
                0xf55038d8e2a1202f,
                0x279426fcf5f59740,
            ],
            revision: 0,
            response: UnsafeCell::new(None),
        }
    }

    pub fn get_response(&self) -> Option<BootloaderInfoResponse> {
        unsafe { self.response.get().read_volatile() }
    }
}

#[repr(transparent)]
pub struct BootloaderInfoResponse {
    raw: NonNull<sys::limine_bootloader_info_response>,
}

impl BootloaderInfoResponse {
    pub const fn as_raw(&self) -> &sys::limine_bootloader_info_response {
        unsafe { self.raw.as_ref() }
    }

    pub const fn revision(&self) -> u64 {
        self.as_raw().revision
    }

    pub const fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_raw().name) }
    }

    pub const fn version(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_raw().version) }
    }
}
