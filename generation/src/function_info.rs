use repository::*;

use crate::{names::filter_name, namespace::NamespaceConfig, utils::copy_c_string};

pub struct FunctionOutput {
  pub func_info: *mut GIFunctionInfo,
  pub func_codegen: codegen::Function,

  pub g_name: String,
  pub c_name: String,
  pub rs_name: String,

  pub arg_types: Vec<codegen::Type>,
  pub return_type: codegen::Type,

  pub scope: codegen::Scope,
}

pub fn generate_function(ns_conf: &NamespaceConfig, func_info: *mut GIFunctionInfo) -> Result<FunctionOutput, &str> {
  let g_name = copy_c_string(unsafe { gi_base_info_get_name(func_info.cast()) });
  let c_name = copy_c_string(unsafe { gi_function_info_get_symbol(func_info.cast()) });
  let rs_name = filter_name(&g_name).to_string();

  let is_depreciated = unsafe { gi_base_info_is_deprecated(func_info.cast()) == 1 };

  if is_depreciated {
    return Err("Is Depreciated");
  }

  let return_type = codegen::Type::new("()");
  let arg_types = Vec::new();

  let mut scope = codegen::Scope::new();
  let mut func_codegen = codegen::Function::new(&rs_name);

  func_codegen
    .vis("pub")
    .attr("napi");

  scope.import("napi_derive", "napi");
  scope.import(&ns_conf.project.implementation_path, &c_name);

  scope.push_fn(func_codegen.clone());
  return Ok(FunctionOutput {
    func_info,
    func_codegen,

    g_name,
    c_name,
    rs_name,

    arg_types,
    return_type,

    scope,
  });
}