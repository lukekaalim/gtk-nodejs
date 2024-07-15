use std::path::PathBuf;

use generation::project::generate_project;

fn main() {
  println!("Yoo");
  let output_dir = PathBuf::from("output/src");
  generate_project(
    "GLib",
    "2.0",
    "crate::glib_native",
    "crate",
    &output_dir,
  );
}