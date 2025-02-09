use core::{cell::UnsafeCell, ptr::NonNull};

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
