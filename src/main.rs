#[macro_use]
extern crate lazy_static;

mod csv_parser;
mod file_reader;
mod tokenizer;
mod finite_state_machine;

use tokenizer::Tokenizer;


fn main() {
   	let mut tokenizer = Tokenizer::new();
   	tokenizer.run("tests/0008.txt");

   	println!("\t{:6} {:6} {:15} {:15} {:15}", "Line", "Col", "Type", "Value", "Text");
   	for i in &tokenizer.tokens {
   		println!("{}", i);
   	}
}
