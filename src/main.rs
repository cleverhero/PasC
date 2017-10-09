#[macro_use]
extern crate lazy_static;

mod csv_parser;
mod file_reader;
mod tokenizer;
mod finite_state_machine;

use tokenizer::Tokenizer;


fn main() {
   	let mut tokenizer = Tokenizer::new();
   	tokenizer.run("tests/0001.txt");
}
