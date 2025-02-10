use core::{cell::UnsafeCell, ffi::c_void, ptr::NonNull};

use crate::sys;

#[repr(C)]
pub struct FramebufferRequest {
    id: [u64; 4],
    revision: u64,
    response: UnsafeCell<Option<FramebufferResponse>>,
}

#[repr(transparent)]
pub struct FramebufferResponse {
    raw: NonNull<sys::limine_framebuffer_response>,
}

#[repr(transparent)]
pub struct FrameBuffer {
    raw: NonNull<sys::limine_framebuffer>,
}

impl FrameBuffer {
    pub const fn as_raw(&self) -> &sys::limine_framebuffer {
        unsafe { self.raw.as_ref() }
    }

    pub const fn address(&self) -> *mut c_void {
        self.as_raw().address
    }

    pub const fn width(&self) -> u64 {
        self.as_raw().width
    }

    pub const fn height(&self) -> u64 {
        self.as_raw().height
    }

    pub const fn pitch(&self) -> u64 {
        self.as_raw().pitch
    }

    pub const fn red_mask_size(&self) -> u8 {
        self.as_raw().red_mask_size
    }

    pub const fn red_mask_shift(&self) -> u8 {
        self.as_raw().red_mask_shift
    }

    pub const fn green_mask_size(&self) -> u8 {
        self.as_raw().green_mask_size
    }

    pub const fn green_mask_shift(&self) -> u8 {
        self.as_raw().green_mask_shift
    }

    pub const fn blue_mask_size(&self) -> u8 {
        self.as_raw().blue_mask_size
    }

    pub const fn blue_mask_shift(&self) -> u8 {
        self.as_raw().blue_mask_shift
    }
}

impl FramebufferResponse {
    pub const fn as_raw(&self) -> &sys::limine_framebuffer_response {
        unsafe { self.raw.as_ref() }
    }

    pub const fn revision(&self) -> u64 {
        self.as_raw().revision
    }

    pub const fn framebuffers(&self) -> &[FrameBuffer] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_raw().framebuffers.cast(),
                self.as_raw().framebuffer_count as _,
            )
        }
    }
}

impl FramebufferRequest {
    pub const fn new() -> Self {
        Self {
            id: [
                0xc7b1dd30df4c8b88,
                0x0a82e883a194f07b,
                0x9d5827dcd881dd75,
                0xa3148604f6fab11b,
            ],
            revision: 1,
            response: UnsafeCell::new(None),
        }
    }

    pub fn get_response(&self) -> Option<FramebufferResponse> {
        unsafe { self.response.get().read_volatile() }
    }
}

unsafe impl Sync for FramebufferRequest {}
