use rust_sitter::{Spanned, leaf};

#[rust_sitter::grammar("ranni")]
mod grammar {
	#[derive(Debug)]
	pub struct Ident(
		#[rust_sitter::word]
		#[rust_sitter::leaf(pattern = r"[$_a-zA-Z][_a-zA-Z0-9\d]+", transform = |v| v.to_string())]
		String,
	);

	#[derive(Debug)]
	pub enum LetQual {
		#[rust_sitter::leaf(text = "case")]
		Case,
		#[rust_sitter::leaf(text = "method")]
		Method,
	}

	#[derive(Debug)]
	pub enum Block {
		Arrow(#[rust_sitter::leaf(text = "=>")] (), Box<Expr>),
		Body(
			#[rust_sitter::leaf(text = "{")] (),
			Vec<Expr>,
			#[rust_sitter::leaf(text = "}")] (),
		),
	}

	#[derive(Debug)]
	pub struct Hint {
		#[rust_sitter::leaf(text = ":")]
		_colon: (),
		value: Box<Expr>,
	}

	#[derive(Debug)]
	struct AssignValue {
		#[rust_sitter::leaf(text = "=")]
		_equal: (),
		value: Box<Expr>,
	}

	#[derive(Debug)]
	pub struct Assign {
		name: Ident,
		hint: Option<Hint>,
		value: Option<AssignValue>,
	}

	#[derive(Debug)]
	pub struct Record {
		#[rust_sitter::leaf(text = "(")]
		_open: (),
		#[rust_sitter::repeat(non_empty = true)]
		pos: Vec<Expr>,
		named: Vec<Assign>,
		#[rust_sitter::leaf(text = ")")]
		_close: (),
	}

	#[derive(Debug)]
	pub struct Func {
		#[rust_sitter::leaf(text = "fn")]
		_fn: (),
		args: Option<Record>,
		ret: Option<Box<Expr>>,
		body: Block,
	}

	#[derive(Debug)]
	pub struct Let {
		#[rust_sitter::leaf(text = "let")]
		_let: (),
		qualifier: Option<LetQual>,
		assign: Assign,
		rest: Option<Box<Expr>>, // Let is a statement
	}

	#[derive(Debug)]
	pub struct Pragma {
		#[rust_sitter::leaf(text = "pragma")]
		_pragma: (),
		assign: Assign,
		rest: Option<Box<Expr>>, // Pragma is a statement
	}

	#[derive(Debug)]
	pub enum Literal {
		Int(#[rust_sitter::leaf(pattern = r"-?\d[_\d]*", transform = |v| v.parse().unwrap())] u64),
		Float(
			#[rust_sitter::leaf(pattern = r"-?\d[_\d]*\.\d[_\d]*(e\d+)?", transform = |v| v.parse().unwrap())]
			 f64,
		),
	}

	#[derive(Debug)]
	#[rust_sitter::language]
	pub enum Expr {
		// Literals
		Literal(Literal),

		// Variables
		Let(Let),
		Lookup(Ident),

		// Control
		Block(Block),
		Pragma(Pragma),
		Func(Func),
		FunCall(Ident, Record),

		// Compunds
		Record(Record),
		Struct(#[rust_sitter::leaf(text = "struct")] (), Block),

		// Arithmetic
		#[rust_sitter::prec_left(1)]
		Add(Box<Expr>, #[rust_sitter::leaf(text = "+")] (), Box<Expr>),
		#[rust_sitter::prec_left(1)]
		Sub(Box<Expr>, #[rust_sitter::leaf(text = "+")] (), Box<Expr>),
		#[rust_sitter::prec_left(2)]
		Mul(Box<Expr>, #[rust_sitter::leaf(text = "*")] (), Box<Expr>),
		#[rust_sitter::prec_left(2)]
		Div(Box<Expr>, #[rust_sitter::leaf(text = "/")] (), Box<Expr>),
		#[rust_sitter::prec_left(2)]
		Mod(Box<Expr>, #[rust_sitter::leaf(text = "%")] (), Box<Expr>),
		#[rust_sitter::prec_left(3)]
		Exp(Box<Expr>, #[rust_sitter::leaf(text = "^")] (), Box<Expr>),
	}

	#[rust_sitter::extra]
	struct Whitespace {
		#[rust_sitter::leaf(pattern = r"[\s,]")]
		_whitespace: (),
	}
}

pub use grammar::{Expr, parse};

#[cfg(text)]
mod test {
	use std::fs::read_to_string;

	// Use the entire standard library as our test input for the parser
	#[test]
	fn std_lib() {
		let files = glob::glob("lib/*.rni");
		for f in files {
			let code = read_to_string(path);
			match parse(&code) {
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
	}
}
