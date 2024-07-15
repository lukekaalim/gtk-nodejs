pub fn filter_name(input: &str) -> &str {
  match input {
    "switch" => "_switch",
    "match" => "_match",
    "import" => "_import",
    "type" => "_type",
    _ => input
  }
}

pub fn test_ignore_cname(input: &str) -> bool {
  match input {
    "g_variant_get_gtype" => true,
    "g_strv_get_type" => true,
    _ => false,
  }
}