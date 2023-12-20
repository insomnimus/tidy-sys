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

const HEADER: &str = r#"
#include "tidy.h"
#include "tidybuffio.h"
#include "tidyplatform.h"
#include "tidyenum.h"
"#;

const fn onoff(on: bool) -> &'static str {
	if on {
		"on"
	} else {
		"off"
	}
}

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

const CMAKE_DEFINES: &[(&str, &str)] = &[
	("ENABLE_MEMORY_DEBUG", "off"),
	("ENABLE_DEBUG_LOG", "off"),
	("ENABLE_CRTDBG_MEMORY", "off"),
	("ENABLE_ALLOC_DEBUG", "off"),
	("BUILD_SHARED_LIB", "off"),
	("SUPPORT_CONSOLE_APP", "off"),
	(
		"SUPPORT_LOCALIZATIONS",
		onoff(cfg!(feature = "localization")),
	),
];

fn main() {
	let profile = env::var("PROFILE").unwrap();
	let opt_level = env::var("OPT_LEVEL")
		.ok()
		.filter(|s| s.len() == 1 && "0123szg".contains(s))
		.unwrap_or_else(|| {
			String::from(match profile.as_str() {
				"release" | "bench" => "2",
				"debug" => "0",
				_ => {
					eprintln!("warning: unrecognized cargo profile {profile}; using opt-level 2");
					"2"
				}
			})
		});
	let opt_flag = format!("-O{opt_level}");

	println!("cargo:rerun-if-changed=vendor/");

	for env in ["CMAKE_GENERATOR", "CC", "TIDY_SYS_CFLAGS", "CFLAGS"] {
		println!("cargo:rerun-if-env-changed={env}");
	}

	let mut cmake = cmake::Config::new("vendor/tidy.5.8.0");
	for (k, v) in CMAKE_DEFINES {
		cmake.define(k, v);
	}

	cmake
		.profile("Release") // Not setting this causes problems with msbuild
			.cflag(opt_flag)
			.cflag("-DNDEBUG");

	if let Ok(flags) = env::var("TIDY_SYS_CFLAGS") {
		eprintln!("info: using CFLAGS from the TIDY_SYS_CFLAGS environment variable with the value: {flags}");
		for s in flags.split_whitespace() {
			cmake.cflag(s);
		}
	}

	let p = cmake.build();

	println!("cargo:rustc-link-search=native={}/lib", p.display());
	if cfg!(windows) {
		println!("cargo:rustc-link-lib=static=tidy_static");
	} else {
		println!("cargo:rustc-link-lib=static=tidy");
	}

	// Generate bindings
	let mut builder = bindgen::Builder::default()
		.clang_arg(format!("-I{}/include", p.display()))
		.header_contents("wrapper.h", HEADER)
		.no_default("_?TidyDoc")
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
