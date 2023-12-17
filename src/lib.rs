#![allow(
	non_camel_case_types,
	non_upper_case_globals,
	non_snake_case,
	rustdoc::broken_intra_doc_links
)]
#![doc = include_str!("../readme.md")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs",));

pub const NoState: ulong = TidyTriState::NoState as ulong;
pub const YesState: ulong = TidyTriState::YesState as ulong;
pub const AutoState: ulong = TidyTriState::AutoState as ulong;
