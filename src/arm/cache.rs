#![allow(dead_code)]

use core::{arch::global_asm, ffi::c_void};

global_asm!(include_str!("cache.s"));

extern "C" {
    fn armDCacheFlush(addr:*mut c_void,size:usize);
    fn armDCacheClean(addr:*mut c_void,size:usize);
    fn armICacheInvalidate(addr:*mut c_void,size:usize);
    fn armDCacheZero(addr:*mut c_void,size:usize);
}