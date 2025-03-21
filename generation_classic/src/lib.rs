use std::ffi::CString;
use std::ptr;

use introspection;

pub fn gather_infos() {
  unsafe {
    let repository = introspection::g_irepository_get_default();

    let namespace = CString::new("Hello").unwrap();
    let version = CString::new("World").unwrap();

    let namespace_ptr = namespace.as_ptr();
    let version_ptr: *const i8 = version.as_ptr();

    let error_ptr = ptr::null_mut();

    introspection::g_irepository_require(repository, namespace_ptr, version_ptr, 0, error_ptr);

    println!("Hallo!");
  }
}