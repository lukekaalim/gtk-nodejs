use std::ptr::null;

use codegen::Function;
use repository::*;

use crate::{arg_info::{generate_arg, ArgOutput}, names::{filter_name, test_ignore_cname}, namespace::NamespaceConfig, type_info::generate_type_path, utils::copy_c_string};

pub struct FunctionOutput {
  pub func_info: *mut GIFunctionInfo,

  pub g_name: String,
  pub c_name: String,
  pub rs_name: String,

  pub assembly: FunctionAssembly,

  pub scope: codegen::Scope,
}

pub struct FunctionAssembly {
  imports: Vec<(String, String)>,

  rs_name: String,
  rs_arguments: Vec<(String, String)>,
  rs_return_type: String,
  rs_return_identifier: String,

  prelude: Vec<String>,

  c_arguments: Vec<String>,
  c_end_arguments: Vec<String>,
  c_implementation_path: String,

  conclude: Vec<String>,
}

impl FunctionAssembly {
  fn new() -> FunctionAssembly {
    FunctionAssembly {
      imports: Vec::new(),

      rs_name: String::new(),
      rs_arguments: Vec::new(),
      rs_return_type: String::new(),
      rs_return_identifier: "return_c_value".to_string(),
    
      prelude: Vec::new(),
    
      c_arguments: Vec::new(),
      c_end_arguments: Vec::new(),
      c_implementation_path: String::new(),
    
      conclude: Vec::new(),
    }
  }
  fn assemble(&mut self, scope: &mut codegen::Scope) {
    let mut func = codegen::Function::new(&self.rs_name);

    func.vis("pub");
    func.attr("napi");

    func.ret(&self.rs_return_type);
    
    for (arg_name, arg_type) in &self.rs_arguments {
      func.arg(&format!("mut {arg_name}"), arg_type);
    }

    for prelude_line in &self.prelude {
      func.line(prelude_line);
    }
    self.c_arguments.append(&mut self.c_end_arguments);

    let c_argument_list = self.c_arguments.join(",\n    ");
    let call_expression = format!("unsafe {{\n  {}(\n    {}\n  )\n}}", self.c_implementation_path, c_argument_list);

    func.line(format!("let return_c_value = {};", call_expression));

    for conclude_line in &self.conclude {
      func.line(conclude_line);
    }

    func.line(format!("return {};", self.rs_return_identifier));

    for (import_path, import_type) in &self.imports {
      scope.import(import_path, import_type);
    }
    scope.push_fn(func);
  }
}

pub fn generate_function(ns_conf: &NamespaceConfig, func_info: *mut GIFunctionInfo) -> Result<FunctionOutput, String> {
  let g_name = copy_c_string(unsafe { gi_base_info_get_name(func_info.cast()) });
  let c_name = copy_c_string(unsafe { gi_function_info_get_symbol(func_info.cast()) });
  let rs_name = filter_name(&g_name).to_string();

  let is_depreciated = unsafe { gi_base_info_is_deprecated(func_info.cast()) == 1 };
  let is_ignored = test_ignore_cname(&c_name);

  let is_error_throwing = unsafe { gi_callable_info_can_throw_gerror(func_info.cast()) == 1 };

  if is_ignored {
    return Err("Is Ignored".to_string());
  }
  if is_depreciated {
    return Err("Is Depreciated".to_string());
  }

  let mut assembly = FunctionAssembly::new();

  assembly.rs_name = rs_name.to_string();
  assembly.imports.push((format!("napi_derive"), format!("napi")));
  assembly.c_implementation_path = format!("{}::{}", ns_conf.project.implementation_path, c_name);

  let mut scope = codegen::Scope::new();

  if is_error_throwing {
    generate_error_handler(&mut assembly);
  }

  let arg_count = unsafe { gi_callable_info_get_n_args(func_info.cast()) };
  for i in 0..arg_count {
    let arg_info = unsafe { gi_callable_info_get_arg(func_info.cast(), i) };
    let arg_output = generate_arg(ns_conf, arg_info)?;

    assembly.rs_arguments.push((arg_output.name, arg_output.type_expression));
    assembly.c_arguments.push(arg_output.marshalled_identifier);

    for arg_marshal_expression in arg_output.marshal_expressions {
      assembly.prelude.push(arg_marshal_expression);
    }
  }
  generate_return(ns_conf, func_info, &mut assembly)?;

  assembly.assemble(&mut scope);

  return Ok(FunctionOutput {
    func_info,

    g_name,
    c_name,
    rs_name,

    assembly,
    scope,
  });
}

fn generate_error_handler(assembly: &mut FunctionAssembly) {
  assembly.imports.push(("napi".to_string(), "Env".to_string()));
  assembly.rs_arguments.push(("env".to_string(), "Env".to_string()));

  assembly.prelude.push(format!("let error_ptr = std::ptr::null_mut();"));

  assembly.c_end_arguments.push(format!("error_ptr"));
  
  assembly.conclude.push(format!("if error_ptr.is_null() {{"));
  assembly.conclude.push(format!("\t env.throw_error(\"Ooops\", None);"));
  assembly.conclude.push(format!("}}"));
}

fn generate_return(ns_conf: &NamespaceConfig, func_info: *mut GIFunctionInfo, assembly: &mut FunctionAssembly) -> Result<(), String> {
  let return_type = generate_return_type(ns_conf, func_info)?;
  assembly.rs_return_type = return_type;

  let return_type_info = unsafe { gi_callable_info_get_return_type(func_info.cast()) };
  let return_type_tag = unsafe { gi_type_info_get_tag(return_type_info) };

  let is_pointer = unsafe { gi_type_info_is_pointer(return_type_info) } == 1;

  match return_type_tag {
    | GITypeTag_GI_TYPE_TAG_UTF8
    | GITypeTag_GI_TYPE_TAG_FILENAME
      => {
        assembly.conclude.push(format!("let return_cstr = unsafe {{ std::ffi::CStr::from_ptr(return_c_value) }};", ));
        assembly.conclude.push(format!("let return_rs = return_cstr.to_str().unwrap().to_string();", ));
        assembly.rs_return_identifier = format!("return_rs");
      }
    | GITypeTag_GI_TYPE_TAG_BOOLEAN
     => {
      assembly.conclude.push(format!("let return_rs = return_c_value == 1;", ));
      assembly.rs_return_identifier = format!("return_rs");
     }
    | GITypeTag_GI_TYPE_TAG_VOID
    | GITypeTag_GI_TYPE_TAG_UINT8
    | GITypeTag_GI_TYPE_TAG_UINT16
    | GITypeTag_GI_TYPE_TAG_UINT32
    //| GITypeTag_GI_TYPE_TAG_UINT64
    | GITypeTag_GI_TYPE_TAG_INT8
    | GITypeTag_GI_TYPE_TAG_INT16
    | GITypeTag_GI_TYPE_TAG_INT32
    | GITypeTag_GI_TYPE_TAG_INT64
     => {
      if is_pointer {
        return Err(format!("Dont support pointers yet"));
      } else {
        assembly.rs_return_identifier = format!("return_c_value");
      }
      //assembly.rs_return_expression = format!();
     }
     _ => {
      return Err("Unsupported marshall".to_string());
     }
  }

  return Ok(());
}

fn generate_return_type(ns_conf: &NamespaceConfig, func_info: *mut GIFunctionInfo) -> Result<String, String> {
  let return_type_info = unsafe { gi_callable_info_get_return_type(func_info.cast()) };

  let type_path = generate_type_path(ns_conf, return_type_info)?;
  let type_tag = unsafe { gi_type_info_get_tag(return_type_info) };

  let is_pointer = unsafe { gi_type_info_is_pointer(return_type_info) } == 1;
  let is_void = type_tag == GITypeTag_GI_TYPE_TAG_VOID;

  let type_expression = {
    if is_void && is_pointer {
      format!("napi::bindgen_prelude::BigInt")
    } else {
      format!("{type_path}")
    }
  };

  return Ok(type_expression);
}