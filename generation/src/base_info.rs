use std::any::Any;

use repository::*;

pub enum GTypeUnion {
  FunctionInfo(*mut GIFunctionInfo),
  SignalInfo(*mut GISignalInfo),

  StructInfo(*mut GIStructInfo),

  ObjectInfo(*mut GIObjectInfo),
  InterfaceInfo(*mut GIInterfaceInfo),

  EnumInfo(*mut GIEnumInfo),
  UnionInfo(*mut GIUnionInfo),

  UnknownInfo(*mut GIBaseInfo)
}
struct InfoCache {
  function_info_type: u64,
  signal_info_type: u64,

  struct_info_type: u64,

  object_info_type: u64,
  interface_info_type: u64,

  enum_info_type: u64,
  union_info_type: u64,
}

fn generate_info_cache() -> InfoCache {
  return unsafe {
    InfoCache {
      function_info_type: gi_function_info_get_type(),
      signal_info_type: gi_signal_info_get_type(),
    
      struct_info_type: gi_struct_info_get_type(),
    
      object_info_type:     gi_object_info_get_type(),
      interface_info_type:  gi_interface_info_get_type(),
    
      enum_info_type:   gi_enum_info_get_type(),
      union_info_type:  gi_union_info_get_type(),
    }
  }
}

pub fn get_info_type(info: *mut GIBaseInfo) -> GTypeUnion {
  // pass this as an arg to avoid recomputing it all the time
  let info_cache = generate_info_cache();

  if unsafe { g_type_check_instance_is_a(info.cast(), info_cache.function_info_type) == 1 } {
    return GTypeUnion::FunctionInfo(info.cast());
  } else if unsafe { g_type_check_instance_is_a(info.cast(), info_cache.struct_info_type) == 1 } {
      return GTypeUnion::StructInfo(info.cast());
  } else if unsafe { g_type_check_instance_is_a(info.cast(), info_cache.object_info_type) == 1 } {
      return GTypeUnion::ObjectInfo(info.cast());
  } else if unsafe { g_type_check_instance_is_a(info.cast(), info_cache.signal_info_type) == 1 } {
      return GTypeUnion::SignalInfo(info.cast());
  } else {
    return GTypeUnion::UnknownInfo(info);
  }
}
