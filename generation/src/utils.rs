use std::ffi::CStr;

/* Very evilly just copy the string and don't bother releasing the c ptr */
pub fn copy_c_string(ptr: *const i8) -> String {
  let cstr = unsafe { CStr::from_ptr(ptr) };
  return cstr.to_str().unwrap().to_string();
}