use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate blake3;
extern crate base64;

#[no_mangle]
pub extern "C" fn hash(h: *const c_char) -> *const c_char {
    let b = unsafe { CStr::from_ptr(h).to_bytes() };
    let hash = blake3::hash(&b);
    let c_str = CString::new(base64::encode(hash.as_bytes())).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return ptr
}

#[no_mangle]
pub extern "C" fn hex(h: *const c_char) -> *const c_char {
    let b = unsafe { CStr::from_ptr(h).to_bytes() };
    let hash = blake3::hash(&b);
    let c_str = CString::new(hash.to_hex().as_str()).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return ptr
}

