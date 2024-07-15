use napi_derive::napi;

#[napi]
pub struct VoidPointer {
  ptr: *mut ()
}

impl VoidPointer {
  pub fn read(&self) -> *mut () {
    return self.ptr;
  }
}

