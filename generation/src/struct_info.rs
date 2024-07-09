use repository::*;

use crate::{names::filter_name, namespace::NamespaceConfig, utils::copy_c_string};

pub struct StructOutput {
  pub g_name: String,
  pub c_name: String,
  pub rs_name: String,

  pub fields: Vec<(String, codegen::Type)>,
  pub scope: codegen::Scope,
}

pub fn generate_struct(ns_config: &NamespaceConfig, struct_info: *mut GIStructInfo) -> Result<StructOutput, String> {
  let g_name = copy_c_string(unsafe { gi_base_info_get_name(struct_info.cast()) });
  let c_name = format!("{}{}", ns_config.c_prefix, g_name);
  let rs_name = filter_name(&g_name).to_string();

  let fields = Vec::new();
  let mut scope = codegen::Scope::new();

  scope.import("napi_derive", "napi");

  scope.new_struct(&rs_name)
    .vis("pub")
    .attr("napi");
  
  return Ok(StructOutput {
    g_name,
    c_name,
    rs_name,

    fields,
    scope
  })
}