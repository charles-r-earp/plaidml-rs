use std::env;
use cpp_build;

fn main() {
  println!("cargo:rustc-link-lib=plaidml");
  let home = env::var("HOME").unwrap();
  cpp_build::Config::new()
    .include(home + "/plaidml/include")
    .flag("-w")
    .build("src/lib.rs");
}
