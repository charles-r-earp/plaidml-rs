use std::env;
use cpp_build;

fn main() {
  let virtual_env = env::var("VIRTUAL_ENV").unwrap();
  println!("cargo:rustc-link-lib=plaidml");
  println!("cargo:rustc-link-search={}/lib", &virtual_env);
  cpp_build::Config::new()
    .include(format!("{}/include", &virtual_env).as_str())
    .flag("-w")
    .flag(&format!("-Wl, -rpath={}/lib", &virtual_env).as_str())
    .build("src/lib.rs");
}
