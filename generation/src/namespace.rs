use std::{collections::HashMap, ffi::CString};
use repository::*;

use crate::{
  base_info::{get_info_type, GTypeUnion},
  function_info::{generate_function, FunctionOutput},
  project::ProjectConfig,
  struct_info::{generate_struct, StructOutput},
  utils::copy_c_string
};
use convert_case::{Case, Casing};

#[derive(Clone)]
pub struct NamespaceConfig {
  pub namespace: String,
  pub namespace_cstr: CString,

  pub c_prefix: String,
  pub namespace_path: String,
  pub project: ProjectConfig,
}
pub struct NamespaceOutput {
  pub ns_conf: NamespaceConfig,

  pub functions: Vec<FunctionOutput>,
  pub objects: Vec<String>,
  pub structs: Vec<StructOutput>,

  pub files: HashMap<String, codegen::Scope>,

  pub errors: Vec<String>,
}

fn get_infos_from_repo(repo: *mut GIRepository, namespace: &str) -> Vec<*mut GIBaseInfo> {
  let namespace_cstr = CString::new(namespace).unwrap();

  let count = unsafe { gi_repository_get_n_infos(repo, namespace_cstr.as_ptr()) };

  let mut infos = Vec::new();
  for i in 0..count {
    let info = unsafe { gi_repository_get_info(repo, namespace_cstr.as_ptr(), i) };
    infos.push(info);
  }
  return infos;
}

pub fn generate_namespace_types(project: &ProjectConfig, namespace: &str) -> NamespaceOutput {
  let namespace_cstr = CString::new(namespace).unwrap();
  let c_prefix = copy_c_string(unsafe {
    gi_repository_get_c_prefix(project.repo, namespace_cstr.as_ptr())
  });
  
  let ns_conf = NamespaceConfig {
    namespace: namespace.to_string(),
    namespace_cstr,
    namespace_path: format!("{}::{}", project.generated_path, namespace.to_case(Case::Snake)),

    c_prefix,
    project: project.clone(),
  };
  let mut ns_output = NamespaceOutput {
    ns_conf: ns_conf.clone(),
    objects: Vec::new(),
    structs: Vec::new(),
    files: HashMap::new(),
    functions: Vec::new(),
    errors: Vec::new(),
  };

  let infos = get_infos_from_repo(ns_conf.project.repo, namespace);
  for info in infos {
    let name = copy_c_string(unsafe { gi_base_info_get_name(info) });
    match get_info_type(info) {
      GTypeUnion::FunctionInfo(func_info) =>
        match generate_function(&ns_conf, func_info) {
          Ok(func_output) => { ns_output.functions.push(func_output); },
          Err(err) => { ns_output.errors.push(format!("{name}: {err}")) },
        },
      GTypeUnion::StructInfo(struct_info) =>
      match generate_struct(&ns_conf, struct_info) {
        Ok(struct_output) => { ns_output.structs.push(struct_output); },
        Err(err) => { ns_output.errors.push(err) },
      },
      _ => {}
    }
  }

  create_function_files(&ns_conf, &mut ns_output);
  create_struct_files(&ns_conf, &mut ns_output);

  return ns_output;
}

pub fn create_function_files(ns_conf: &NamespaceConfig, ns_output: &mut NamespaceOutput) {
  let mut root_scope = codegen::Scope::new();
  root_scope.raw("#![allow(warnings)]");
  
  for func_output in &ns_output.functions {
    let file_name = func_output.rs_name.to_case(Case::Snake);
    let file_path = format!("functions/{}.rs", file_name);
    ns_output.files.insert(file_path, func_output.scope.clone());

    root_scope.raw(format!("mod {};", &file_name));
    root_scope.raw(format!("pub use {}::*;", &file_name));

  }

  ns_output.files.insert("functions.rs".to_string(), root_scope);
}

pub fn create_struct_files(ns_conf: &NamespaceConfig, ns_output: &mut NamespaceOutput) {
  let mut root_scope = codegen::Scope::new();
  root_scope.raw("#![allow(warnings)]");

  for struct_output in &ns_output.structs {
    let file_name = struct_output.rs_name.to_case(Case::Snake);
    let name = format!("structs/{}.rs", file_name);
    ns_output.files.insert(name, struct_output.scope.clone());

    root_scope.raw(format!("mod {};", &file_name));
    root_scope.raw(format!("pub use {}::*;", &file_name));
  }

  ns_output.files.insert("structs.rs".to_string(), root_scope);
}
