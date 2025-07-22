#![no_std]

pub mod magic;
pub mod requests;
#[allow(nonstandard_style)]
pub mod sys;

use core::cell::UnsafeCell;

#[repr(C)]
pub struct BaseRevision {
    _id: [u64; 2],
    revision: UnsafeCell<u64>,
}

unsafe impl Send for BaseRevision {}
unsafe impl Sync for BaseRevision {}

impl BaseRevision {
    pub const LATEST: Self = Self::new(3);

    pub const fn new(revision: u64) -> Self {
        Self {
            _id: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
            revision: UnsafeCell::new(revision),
        }
    }

    pub fn revision(&self) -> u64 {
        unsafe { self.revision.get().read_volatile() }
    }
}
