mod csv_parser;
mod file_reader;
mod tokenizer;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f32;
use std::i32;
use std::str::FromStr;
use csv_parser::Parser;
use tokenizer::Tokenizer;
use file_reader::FileReader;
use file_reader::FatChar;

fn main() {
	let file_csv = "TransitionTable.csv";
	let mut reader = FileReader::new(file_csv);

	while (true) {
		match reader.next_char() {
			FatChar::Char{ch} => {
				print!("{}", ch);
				if (ch == '_') { reader.push_back(ch); }
			}, 
			FatChar::Eof => break
		}
	}

   	// let t_table: Vec<Vec<i32>> = Parser::load_form_csv(file_csv);

   	// let mut tokenizer = Tokenizer::new();
   	// tokenizer.run("test.txt");
}
