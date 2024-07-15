#[allow(non_upper_case_globals)]

use repository::*;

use crate::{base_info::{get_info_type, GTypeUnion}, names::filter_name, namespace::NamespaceConfig, utils::copy_c_string};

pub fn generate_type_path(ns_conf: &NamespaceConfig, type_info: *mut GITypeInfo) -> Result<String, String> {
  let tag = unsafe { gi_type_info_get_tag(type_info) };

  match tag {
    | GITypeTag_GI_TYPE_TAG_INTERFACE => {
      let interface = unsafe { gi_type_info_get_interface(type_info) };
      match get_interface_rs_reference_path(&ns_conf, interface) {
        Ok(path) => Ok(path),
        Err(error) => Err(format!("Can\'t handle interface type: {}", error))
      }
    }
    
    GITypeTag_GI_TYPE_TAG_BOOLEAN => Ok(format!("bool")),

    | GITypeTag_GI_TYPE_TAG_UTF8
    | GITypeTag_GI_TYPE_TAG_FILENAME
      => Ok("std::string::String".to_string()),

    GITypeTag_GI_TYPE_TAG_FLOAT => Ok("f32".to_string()),
    GITypeTag_GI_TYPE_TAG_DOUBLE => Ok("f64".to_string()),

    GITypeTag_GI_TYPE_TAG_UINT8 => Ok("u8".to_string()),
    GITypeTag_GI_TYPE_TAG_UINT16 => Ok("u16".to_string()),
    GITypeTag_GI_TYPE_TAG_UINT32 => Ok("u32".to_string()),
    //GITypeTag_GI_TYPE_TAG_UINT64 => Ok(codegen::Type::new("u64")),

    GITypeTag_GI_TYPE_TAG_INT8 => Ok("i8".to_string()),
    GITypeTag_GI_TYPE_TAG_INT16 => Ok("i16".to_string()),
    GITypeTag_GI_TYPE_TAG_INT32 => Ok("i32".to_string()),
    GITypeTag_GI_TYPE_TAG_INT64 => Ok("i64".to_string()),

    GITypeTag_GI_TYPE_TAG_VOID => Ok("napi::bindgen_prelude::Undefined".to_string()),
    
    _ => {
      let type_tag_name = copy_c_string(unsafe { gi_type_tag_to_string(tag) });
      Err(format!("Unsupported type tag: \"{type_tag_name}\"", ))
    }
  }
}

fn get_interface_rs_reference_path(ns_conf: &NamespaceConfig, type_info: *mut GIBaseInfo) -> Result<String, String> {
  let g_name = copy_c_string(unsafe { gi_base_info_get_name(type_info) });
  let rs_name = filter_name(&g_name);
  match get_info_type(type_info) {
    GTypeUnion::ObjectInfo(_) => Ok(format!("{}::objects::{}", ns_conf.namespace_path, rs_name)),
    GTypeUnion::StructInfo(_) => Ok(format!("{}::structs::{}", ns_conf.namespace_path, rs_name)),
    _ => Err("Unimplemented Interface Type".to_string()),
  }
}
