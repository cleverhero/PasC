mod csv_parser;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f32;
use std::i32;
use std::str::FromStr;
use csv_parser::Parser;


fn main() {
	let file_name = "TransitionTable.csv";

   	let t_table: Vec<Vec<i32>> = Parser::load_form_csv(file_name);
}
