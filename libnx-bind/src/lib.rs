#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

pub mod lang_items;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {
    todo!()
}
