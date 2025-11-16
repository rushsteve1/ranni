use std::{ffi::OsStr, path::PathBuf};

use clap::Parser;

mod cli;
use cli::{Cli, Commands};

mod lsp;
use lsp::main as lsp_main;
use tokio::io::AsyncReadExt;

mod parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let cli = Cli::parse();

	match &cli.command {
		// https://doc.rust-lang.org/cargo/reference/features.html
		#[cfg(feature = "compiler")]
		Some(Commands::Lsp {}) => lsp_main().await,
		Some(Commands::Compile { file }) => compile_main(file).await,
		None => todo!(),
		_ => todo!(),
	}
}

async fn compile_main(path: &PathBuf) -> anyhow::Result<()> {
	let code = if path.as_os_str() == OsStr::new("-") {
		let mut s = String::new();
		tokio::io::stdin().read_to_string(&mut s).await?;
		s
	} else {
		tokio::fs::read_to_string(path).await?
	};

	match parser::parse(&code) {
		Ok(expr) => {
			println!("{:#?}", expr);
			Ok(())
		}
		Err(errs) => {
			for e in errs {
				eprintln!("{:#?}", e);
			}
			anyhow::bail!("Parse errors")
		}
	}
}

fn run_main() {}
