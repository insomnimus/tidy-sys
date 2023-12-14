#![allow(
	non_camel_case_types,
	non_upper_case_globals,
	non_snake_case,
	rustdoc::broken_intra_doc_links
)]
#![doc = include_str!("../readme.md")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs",));
