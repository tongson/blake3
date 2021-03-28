use std::os::raw::{c_uint, c_int, c_uchar};
use std::panic;
use std::slice;

extern crate base64;
extern crate blake3;

#[no_mangle]
pub extern "C" fn hash(h: *const c_uchar, l: c_uint, ptr: *mut c_uchar) -> c_int {
  panic::set_hook(Box::new(move |_| eprintln!("panic: blake3.hash()")));
  let b = unsafe { slice::from_raw_parts(h, l as usize) };
  let hash = blake3::hash(&b);
  let hex = hash.to_hex();
  let bytes = hex.as_str().as_bytes();
  unsafe {
    let o = slice::from_raw_parts_mut(ptr, 64);
    o[..64].copy_from_slice(bytes);
  }
  return 0;
}
