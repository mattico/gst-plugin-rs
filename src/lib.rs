#![crate_type="dylib"]

extern crate libc;
extern crate url;

#[macro_use]
pub mod utils;
pub mod rssource;
pub mod rsfilesrc;
pub mod rsfilesink;

use utils::*;
use rssource::Source;
use rsfilesrc::FileSrc;

use std::os::raw::c_void;
use libc::{c_char};
use std::ffi::CString;

extern "C" {
    fn gst_rs_source_register(plugin: *const c_void,
        name: *const c_char,
        long_name: *const c_char,
        description: *const c_char,
        classification: *const c_char,
        author: *const c_char,
        rank: i32,
        create_instance: extern fn() -> *mut Box<Source>,
        protocols: *const c_char) -> GBoolean;
}

#[no_mangle]
pub extern "C" fn sources_register(plugin: *const c_void) -> GBoolean {

    unsafe {
        gst_rs_source_register(plugin,
            CString::new("rsfilesrc").unwrap().as_ptr(),
            CString::new("File Source").unwrap().as_ptr(),
            CString::new("Reads local files").unwrap().as_ptr(),
            CString::new("Source/File").unwrap().as_ptr(),
            CString::new("Sebastian Dröge <sebastian@centricular.com>").unwrap().as_ptr(),
            256 + 100,
            FileSrc::new_ptr,
            CString::new("file").unwrap().as_ptr());
    }
    return GBoolean::True;
}