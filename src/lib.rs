use std::ffi::{CStr};
use std::slice;
use std::os::raw::c_char;
use std::panic;

extern crate base64;
extern crate blake3;

#[no_mangle]
pub extern "C" fn hash(h: *const c_char, ptr: *mut u8, size: u32) -> i32 {
  panic::set_hook(Box::new(move |_| eprintln!("panic: blake3.hash()")));
  let b = unsafe { CStr::from_ptr(h).to_bytes() };
  let data = match base64::decode(&b){
    Ok(dr) => dr,
    Err(_) => return -1,
  };
  let hash = blake3::hash(&data);
  let hex = hash.to_hex();
  let bytes = hex.as_str().as_bytes();
  unsafe {
    let o = slice::from_raw_parts_mut(ptr, size as usize);
    o[..bytes.len()].copy_from_slice(bytes);
  }
  return 0;
}
