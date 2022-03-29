use std::ffi::{CString, CStr};

use libc::c_char;

#[no_mangle]
pub unsafe extern "C" fn setup() {
  println!("Hi from Rust :D!");
}

#[no_mangle]
pub unsafe extern "C" fn load_plugin(plugin: *const c_char) {
  println!("Loading plugin: {:?}", CStr::from_ptr(plugin));
}