use std::env;
use std::path::PathBuf;

fn main() {
  let dep_hash = system_deps::Config::new().probe().unwrap();

  let include_paths: Vec<PathBuf> = dep_hash
    .values()
    .flat_map(|lib| lib.include_paths.clone())
    .collect();

  let clang_args: Vec<String> = include_paths
    .iter()
    .map(|p| format!("-I{}", p.as_os_str().to_str().unwrap()))
    .collect();

  let bindings = bindgen::Builder::default()
      // The input header we would like to generate
      // bindings for.
      .clang_args(clang_args)
      .header("src/wrapper.h")
      // Finish the builder and generate the bindings.
      .generate()
      // Unwrap the Result and panic on failure.
      .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
}
