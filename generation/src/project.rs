use std::{ffi::{CStr, CString}, fs::{create_dir_all, File}, io::Write, path::PathBuf, ptr::null_mut};

use convert_case::Casing;
use repository::{gi_repository_new, gi_repository_require, GIRepository, GIRepositoryLoadFlags_GI_REPOSITORY_LOAD_FLAG_NONE, GITypelib};

use crate::{namespace::generate_namespace_types, utils::copy_c_string};

#[derive(Clone)]
pub struct ProjectConfig {
  pub repo: *mut GIRepository,
  pub root_namespace: String,

  pub loaded_namespaces: Vec<String>,

  pub implementation_path: String
}

pub fn generate_project(
  namespace: &str,
  version: &str,
  implementation_path: &str, // The rust type path for the bindgen code (where all the types and functions are)
  output_directory: &PathBuf
) {
  /*
   * [namespace]/objects/[object].rs
   * [namespace]/objects.rs
   * [namespace]/structs/[struct].rs
   * [namespace]/structs.rs
   * [namespace]/functions.rs
   * [namespace].rs
   */
  let namespace_dir = output_directory.join(namespace.to_case(convert_case::Case::Snake));
  let repo = unsafe { gi_repository_new() };

  require_namespace(repo, namespace, version).unwrap();

  let config = ProjectConfig {
    repo,
    root_namespace: namespace.to_string(),
    loaded_namespaces: Vec::new(),
    implementation_path: implementation_path.to_string()
  };

  create_dir_all(namespace_dir.clone()).unwrap();
  let mut namespace_lib_scope = codegen::Scope::new();
  namespace_lib_scope.raw("mod structs;");
  namespace_lib_scope.raw("mod functions;");
  namespace_lib_scope.raw("pub use functions::*;");
  namespace_lib_scope.raw("pub use structs::*;");
  let namespace_filename = format!("{}.rs", namespace.to_case(convert_case::Case::Snake));
  let mut namespace_file = File::create(output_directory.join(namespace_filename)).unwrap();
  namespace_file.write_all(namespace_lib_scope.to_string().as_bytes()).unwrap();

  let ns_output = generate_namespace_types(&config, namespace);
  for (filename, scope) in ns_output.files {
    let path = namespace_dir.join(filename);
    let parent = path.parent();
    if parent.is_some() {
      let parent_path = parent.unwrap();
      create_dir_all(parent_path).unwrap();
    }
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(scope.to_string().as_bytes()).unwrap();
  }
}

fn require_namespace(repo: *mut GIRepository, namespace: &str, version: &str) -> Result<*mut GITypelib, String> {
  let namespace_cstr = CString::new(namespace).unwrap();
  let version_cstr = CString::new(version).unwrap();
  let flags = GIRepositoryLoadFlags_GI_REPOSITORY_LOAD_FLAG_NONE;
  let error_ptr = null_mut();
  let lib = unsafe {
    gi_repository_require(repo, namespace_cstr.as_ptr(), version_cstr.as_ptr(), flags, error_ptr)
  };
  if error_ptr.is_null() {
    return Ok(lib);
  } else {
    let error = unsafe { error_ptr.read().read().clone() };
    let message = copy_c_string(error.message);
    return Err(message);
  }
}