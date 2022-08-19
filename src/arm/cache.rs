use core::{
    arch::{asm, global_asm},
    ffi::c_void,
};

use crate::kernel::svc::{CpuRegister, FpuRegister};

global_asm!(include_str!("cache.s"));

#[allow(dead_code)]
extern "C" {
    fn armDCacheFlush(addr: *mut c_void, size: usize);
    fn armDCacheClean(addr: *mut c_void, size: usize);
    fn armICacheInvalidate(addr: *mut c_void, size: usize);
    fn armDCacheZero(addr: *mut c_void, size: usize);
}

impl CpuRegister {
    #[inline]
    pub const fn get_x(&self) -> u64 {
        unsafe { self.x }
    }

    #[inline]
    pub fn set_x(&mut self, x: u64) {
        self.x = x;
    }

    #[inline]
    pub const fn get_w(&self) -> u32 {
        unsafe { self.w }
    }

    #[inline]
    pub fn set_w(&mut self, w: u32) {
        self.w = w;
    }

    #[inline]
    pub const fn get_r(&self) -> u32 {
        unsafe { self.r }
    }

    #[inline]
    pub fn set_r(&mut self, r: u32) {
        self.r = r;
    }
}

impl FpuRegister {
    #[inline]
    pub const fn get_v(&self) -> u128 {
        unsafe { self.v }
    }

    #[inline]
    pub fn set_v(&mut self, v: u128) {
        self.v = v;
    }

    #[inline]
    pub const fn get_d(&self) -> f64 {
        unsafe { self.d }
    }

    #[inline]
    pub fn set_d(&mut self, d: f64) {
        self.d = d;
    }

    #[inline]
    pub const fn get_s(&self) -> f32 {
        unsafe { self.s }
    }

    #[inline]
    pub fn set_s(&mut self, s: f32) {
        self.s = s;
    }
}

#[inline(always)]
pub fn cache_flush(address: *mut c_void, size: usize) {
    unsafe {
        armDCacheFlush(address, size);
    }
}

/// Gets the system tick
#[inline(always)]
pub fn get_system_tick() -> u64 {
    let tick: u64;
    unsafe {
        asm!(
            "mrs {}, cntpct_el0",
            out(reg) tick
        );
    }
    tick
}

/// Gets the system tick frequency
#[inline(always)]
pub fn get_system_tick_frequency() -> u64 {
    let tick_freq: u64;
    unsafe {
        asm!(
            "mrs {}, cntfrq_el0",
            out(reg) tick_freq
        );
    }
    tick_freq
}
