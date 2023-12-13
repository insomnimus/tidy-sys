use std::{
	env,
	path::PathBuf,
};

fn main() {
	println!("cargo:rerun-if-changed=vendor/");
	for env in ["CMAKE_GENERATOR", "CC"] {
		println!("cargo:rerun-if-env-changed={env}");
	}

	let p = cmake::Config::new("vendor/tidy.5.8.0")
		.profile("release")
		.cflag("-O2")
		.cxxflag("-O2")
		.build();

	println!("cargo:rustc-link-search=native={}/lib", p.display());
	println!("cargo:rustc-link-lib=static=tidy_static");

	// Generate bindings
	// let header = p.join("include/tidy.h");
	let bindings = bindgen::Builder::default()
		.clang_arg(format!("-I{}/include", p.display()))
		.header_contents("wrapper.h", r#"#include "tidy.h""#)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.allowlist_file(r".*[/\\]tidy.*")
		.generate()
		.expect("failed to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.unwrap();
}
