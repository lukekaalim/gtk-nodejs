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

  let c_type_path = format!("{}::{}", ns_config.project.implementation_path, c_name);

  scope.import("napi_derive", "napi");

  let generated_struct = scope.new_struct(&rs_name)
    .vis("pub")
    .attr("napi");

  let c_type = codegen::Type::new(format!("*mut {c_type_path}"));
  let rs_type = codegen::Type::new(&rs_name);
  
  generated_struct.new_field("ptr", c_type.clone());

  let inner_impl = scope.new_impl(&rs_name);
  let inner_impl_constructor = inner_impl.new_fn("new")
    .arg("ptr", c_type.clone())
    .vis("pub")
    .ret(rs_type)
    .line(format!("return {rs_name} {{ ptr }};"));

  let inner_impl_reader = inner_impl.new_fn("read")
    .arg_ref_self()
    .vis("pub")
    .ret(c_type)
    .line(format!("return self.ptr;"));
  
  return Ok(StructOutput {
    g_name,
    c_name,
    rs_name,

    fields,
    scope
  })
}