pub fn filter_name(input: &str) -> &str {
  match input {
    "switch" => "_switch",
    "match" => "_match",
    "import" => "_import",
    _ => input
  }
}