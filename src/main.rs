#[macro_use]
extern crate lazy_static;
use std::env;

mod csv_parser;
mod file_reader;
mod tokenizer;
mod finite_state_machine;
mod support;

use tokenizer::Tokenizer;


fn main() {
	let mut tokenizer_mode = false;
	let mut file = "".to_string();
	if (env::args().len() == 1) {
		println!("Приходько Олег. 2017 год.");
	}

	for arg in env::args() {
		if (arg[0..1].to_string() != "-") {
			file = arg.to_string();
		}
		if (arg == "-h") {
			println!("Приходько Олег. 2017 год.");
			println!("-h -> help");
			println!("-l file -> run tokenizer in file");
			return;
		}

		if (arg == "-l") {
			tokenizer_mode = true;
		}
	}
	
	if (tokenizer_mode) {
   		let mut tokenizer = Tokenizer::new();
   		tokenizer.run(file);
	
    	println!("\t{:6} {:6} {:15} {:25} {:25}", "Line", "Col", "Type", "Value", "Text");
   		for i in &tokenizer.tokens {
   			println!("{}", i);
   		}
    }
}
