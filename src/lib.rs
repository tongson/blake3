use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate blake3;

#[no_mangle]
pub extern "C" fn bytes(h: *const c_char) -> *const c_char {
    let b = unsafe { CStr::from_ptr(h).to_string_lossy().into_owned() };
    let c_str = CString::new(blake3::as_bytes(&b)).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return ptr
}

#[no_mangle]
pub extern "C" fn hex(h: *const c_char) -> *const c_char {
    let b = unsafe { CStr::from_ptr(h).to_string_lossy().into_owned() };
    let c_str = CString::new(blake3::to_hex(&b)).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return ptr
}
