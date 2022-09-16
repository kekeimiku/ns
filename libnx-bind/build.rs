extern crate bindgen;

use std::env;

pub fn main() {
    let devkitpro_path = env::var("DEVKITPRO").unwrap();

    println!("cargo:rustc-link-lib=static=nx");
    println!(
        "cargo:rustc-link-search=native={}/libnx/lib",
        devkitpro_path
    );

    let bindings = bindgen::Builder::default()    
        .trust_clang_mangling(false)
        .use_core()
        .ctypes_prefix("lang_items")
        .header("wrapper.h")
        .clang_arg(format!("-I{}/libnx/include", devkitpro_path))
        .clang_arg(format!(
            "-I{}/devkitA64/aarch64-none-elf/include",
            devkitpro_path
        ))
        .bitfield_enum("HidMouseButton")
        .bitfield_enum("HidKeyboardModifier")
        .rustified_enum("HidKeyboardScancode")
        .bitfield_enum("HidControllerType")
        .rustified_enum("HidControllerLayoutType")
        .bitfield_enum("HidControllerColorDescription")
        .bitfield_enum("HidControllerKeys")
        .rustified_enum("HidControllerJoystick")
        .bitfield_enum("HidControllerConnectionState")
        .rustified_enum("HidControllerID")
        .generate_inline_functions(true)
        .blocklist_type("u8")
        .blocklist_type("u16")
        .blocklist_type("u32")
        .blocklist_type("u64")
        .generate()
        .expect("Unable to generate bindings");    
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
