use repository::*;

use crate::{names::filter_name, namespace::NamespaceConfig, type_info::{generate_type_path}, utils::copy_c_string};

pub struct ArgOutput {
  pub name: String,

  pub marshal_expressions: Vec<String>,
  pub marshalled_identifier: String,

  pub type_expression: String,

  pub arg_info: *mut GIArgInfo,
}

pub fn generate_arg(ns_conf: &NamespaceConfig, arg_info: *mut GIArgInfo) -> Result<ArgOutput, String> {
  let type_info = unsafe { gi_arg_info_get_type_info(arg_info) };
  let type_path = generate_type_path(ns_conf, type_info)?;

  let type_tag = unsafe { gi_type_info_get_tag(type_info) };
  let tag_name = copy_c_string(unsafe { gi_type_tag_to_string(type_tag) });

  let is_return_value = unsafe { gi_arg_info_is_return_value(arg_info) } == 1;
  let direction = unsafe { gi_arg_info_get_direction(arg_info) };
  let is_optional = unsafe { gi_arg_info_is_optional(arg_info) } == 1;

  let is_pointer = unsafe { gi_type_info_is_pointer(type_info) } == 1;
  let is_interface = type_tag == GITypeTag_GI_TYPE_TAG_INTERFACE;
  let is_void = type_tag == GITypeTag_GI_TYPE_TAG_VOID;
  let is_filename = type_tag == GITypeTag_GI_TYPE_TAG_FILENAME;
  let is_string = type_tag == GITypeTag_GI_TYPE_TAG_UTF8 || is_filename;

  let c_name = copy_c_string(unsafe { gi_base_info_get_name(arg_info.cast()) });
  let name = filter_name(&c_name).to_string();

  if direction == GIDirection_GI_DIRECTION_OUT || direction == GIDirection_GI_DIRECTION_INOUT {
    return Err(format!("Don't support out values yet"));
  }

  if is_optional {
    return Err(format!("Don't support optional values yet"));
  }

  if is_return_value {
    return Err(format!("Don't support return values yet"));
  }

  let marshalled_identifier = format!("{name}_marshalled");

  let type_expression = {
    if is_void && is_pointer {
      format!("napi::bindgen_prelude::BigInt")
    } else if is_interface {
      format!("&{type_path}")
    } else if is_string {
      format!("{type_path}")
    } else if is_pointer {
      format!("napi::bindgen_prelude::External<{type_path}>")
    } else {
      format!("{type_path}")
    }
  };

  let mut marshal_expressions = Vec::new();
  if is_void && is_pointer {
    marshal_expressions.push(format!("let {name}_ptr = {name}.get_u64().1 as *mut std::ffi::c_void;"));
    marshal_expressions.push(format!("let {marshalled_identifier} = {name}_ptr.cast();"));
  } else if is_interface {
    marshal_expressions.push(format!("let {marshalled_identifier} = {name}.read();"));
    if is_pointer {
      return Err(format!("Too much indirection for now"));
    }
  } else {
    match type_tag {
      | GITypeTag_GI_TYPE_TAG_UTF8
      | GITypeTag_GI_TYPE_TAG_FILENAME
        => {
          if is_pointer {
            return Err(format!("Too much indirection for now"));
          }
          marshal_expressions.push(
            format!("let {name}_cstring = std::ffi::CString::new({name}).unwrap();")
          );
          marshal_expressions.push(
            format!("let {marshalled_identifier} = {name}_cstring.as_ptr() as *mut std::ffi::c_char;")
          );
        }
      | GITypeTag_GI_TYPE_TAG_BOOLEAN
       => {
        marshal_expressions.push(
          format!("let {marshalled_identifier} = if {name} {{ 1 }} else {{ 0 }};")
        );
       }
      | GITypeTag_GI_TYPE_TAG_FLOAT
      | GITypeTag_GI_TYPE_TAG_DOUBLE


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
            marshal_expressions.push(format!("let {name}_ptr = {name}.as_mut() as *mut {type_path};"));
            marshal_expressions.push(format!("let {marshalled_identifier} = {name}_ptr.cast();"));
          } else {
            marshal_expressions.push(format!("let {marshalled_identifier} = {name};"));
          }
        }
      _ => {
        return Err(format!("Cant marshal to type tag \"{tag_name}\""))
      }
    }
  }
  return Ok(ArgOutput {
    name,

    marshalled_identifier,
    marshal_expressions,
    type_expression,

    arg_info,
  });
}
