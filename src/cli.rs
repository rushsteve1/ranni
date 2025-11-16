use std::path::PathBuf;

use clap::{Parser, Subcommand};

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[cfg(feature = "compiler")]
#[derive(Subcommand)]
pub enum Commands {
	Lsp {},
	// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#positionals
	Ui { file: PathBuf },
	Compile { file: PathBuf },
	Run {},
}

// https://doc.rust-lang.org/reference/conditional-compilation.html#r-cfg.attr
#[cfg(not(feature = "compiler"))]
pub enum Commands {
	Run {},
}
