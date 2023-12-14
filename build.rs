use std::{
	env,
	path::PathBuf,
};

use bindgen::callbacks::{
	EnumVariantValue,
	ParseCallbacks,
};

const ENUMS: &[(&str, &str)] = &[
	("TidyAttrId", "TidyAttr_"),
	("TidyAttrSortStrategy", "TidySortAttr"),
	("TidyConfigCategory", "Tidy"),
	("TidyDoctypeModes", "TidyDoctype"),
	("TidyDupAttrModes", "Tidy"),
	("TidyEncodingOptions", "TidyEnc"),
	("TidyFormatParameterType", "tidyFormatType_"),
	("TidyLineEnding", "Tidy"),
	("TidyNodeType", "TidyNode_"),
	("TidyOptionId", "Tidy"),
	("TidyOptionType", "Tidy"),
	("TidyReportLevel", "Tidy"),
	("tidyStrings", "-"),
	("TidyTagId", "TidyTag_"),
	("TidyTriState", "Tidy"),
	("TidyUppercase", "TidyUppercase"),
	("TidyUseCustomTagsState", "TidyCustom"),
];

#[derive(Debug)]
struct ParseCallback;

impl ParseCallbacks for ParseCallback {
	fn enum_variant_name(
		&self,
		enum_name: Option<&str>,
		variant: &str,
		_val: EnumVariantValue,
	) -> Option<String> {
		let enum_name = enum_name?;
		for &(name, prefix) in ENUMS {
			if name == enum_name {
				return variant.strip_prefix(prefix).map(str::to_string);
			}
		}

		None
	}
}

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
	if cfg!(windows) {
		println!("cargo:rustc-link-lib=static=tidy_static");
	} else {
		println!("cargo:rustc-link-lib=static=tidy");
	}

	// Generate bindings
	// let header = p.join("include/tidy.h");
	let mut builder = bindgen::Builder::default()
		.clang_arg(format!("-I{}/include", p.display()))
		.header_contents("wrapper.h", r#"#include "tidy.h""#)
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.parse_callbacks(Box::new(ParseCallback))
		.allowlist_file(r".*[/\\]tidy.*")
		.enable_function_attribute_detection();

	for (name, _) in ENUMS {
		builder = builder.rustified_enum(*name);
	}

	let bindings = builder.generate().expect("failed to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.unwrap();
}
