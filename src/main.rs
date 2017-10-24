#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate NameByField;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod TokenizerPack;
mod ParserPack;


use TokenizerPack::tokenizer::Tokenizer;
use ParserPack::parser::Parser;

fn main() {
	let mut tokenizer_mode = false;
	let mut parser_mode = false;
	let mut infile_mode = false;
	let mut file = "".to_string();
	if env::args().len() == 1 {
		println!("Приходько Олег. 2017 год.");
	}

	for arg in env::args() {
		if arg[0..1].to_string() != "-" {
			file = arg.to_string();
		}
		if arg == "-h" {
			println!("Приходько Олег. 2017 год.");
			println!("-h -> help");
			println!("-l file -> run tokenizer in file");
			return;
		}
		if arg == "-f" {
			infile_mode = true;
		}

		if arg == "-l" && !parser_mode {
			tokenizer_mode = true;
		}

		if arg == "-p" && !tokenizer_mode {
			parser_mode = true;
		}
	}

	

	if tokenizer_mode {
		let mut tokenizer = Tokenizer::new(file.clone());
		let mstr = file[0..file.len() - 4].to_string() + ".res";

		if infile_mode {
			let mut file = File::create(mstr).unwrap();
    		file.write_fmt(format_args!("\t{:6} {:6} {:15} {:25} {:25}\n", "Line", "Col", "Type", "Value", "Text"));
    		for res in tokenizer {
   				match res {
   					Ok(token) => {
   						file.write_fmt(format_args!("{}\n", token));
   					}
   					Err(msg) => {
   						file.write_fmt(format_args!("{}\n", msg));
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
    else if parser_mode {
    	let tokenizer = Tokenizer::new(file.clone());
    	let mut parser = Parser::new(tokenizer);

		let mstr = file[0..file.len() - 4].to_string() + ".res";
    	let mut file = File::create(mstr).unwrap();

    	let tree = match parser.parse() {
    		Ok(val) => val,
    		Err(msg) => {
    			if infile_mode {
    				file.write_fmt(format_args!("{}", msg));
    			}
    			else {
    				println!("{}", msg);
    			}
    			return;
    		}
    	};
    	
    	if infile_mode {
    		file.write_fmt(format_args!("{}", tree));
		} 
		else {
    		println!("{}", tree);
		}
    }
}
