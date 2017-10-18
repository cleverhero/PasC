#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate NameByField;

use std::env;
use std::i32;
use std::str::FromStr;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;

mod TokenizerPack;
mod ParserPack;


use TokenizerPack::tokenizer::Tokenizer;
use ParserPack::nodes::Tree;
use ParserPack::nodes::IntNode;
use ParserPack::nodes::BinNode;

fn main() {
	// let tree = Tree{root: Box::new( BinNode {
	// 	left:  Box::new( BinNode {
	// 		left:  Box::new( IntNode { value: 11 } ),
	// 		right: Box::new( IntNode { value: 22 } ),
	// 		op:    "*".to_string(),
 //    	}),
	// 	right: Box::new(  IntNode { value: 33 } ),
	// 	op:    "*".to_string(),
 //    } )};
	// println!("{}", tree);


	let mut tokenizer_mode = false;
	let mut parser_mode = false;
	let mut infile_mode = false;
	let mut file = "".to_string();
	if env::args().len() == 1 {
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
		if (arg == "-f") {
			infile_mode = true;
		}

		if (arg == "-l" && !parser_mode) {
			tokenizer_mode = true;
		}

		if (arg == "-p" && !tokenizer_mode) {
			parser_mode = true;
		}
	}
	
	if (tokenizer_mode) {
    	let mut tokenizer = Tokenizer::new(file.clone());
	
		let mstr = "tests/results/".to_string() + &(file);

		if (infile_mode) {
			let mut file = File::create(mstr).unwrap();
    		file.write_fmt(format_args!("\t{:6} {:6} {:15} {:25} {:25}\n", "Line", "Col", "Type", "Value", "Text"));
    		for res in tokenizer {
   				match res {
   					Ok(token) => {
   						file.write_fmt(format_args!("{}", token));
   					}
   					Err(msg) => {
   						file.write_fmt(format_args!("{}", msg));
   						break;
   					}
   				}
			}
		} 
		else {
    		println!("\t{:6} {:6} {:15} {:25} {:25}", "Line", "Col", "Type", "Value", "Text");
   			for res in tokenizer {
   				match res {
   					Ok(token) => {
   						println!("{}", token);
   					}
   					Err(msg) => {
   						println!("{}", msg);
   						break;
   					}
   				}
			}
		}
    }

    if (parser_mode) {
    	let mut tokenizer = Tokenizer::new(file.clone());
    }
}
