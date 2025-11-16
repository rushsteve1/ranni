use std::path::PathBuf;

// https://github.com/hydro-project/rust-sitter#installation
fn main() {
	println!("cargo:rerun-if-changed=src");
	rust_sitter_tool::build_parsers(&PathBuf::from("src/parser.rs"));
}
