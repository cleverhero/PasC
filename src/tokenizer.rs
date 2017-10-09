use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f32;
use std::i32;
use std::str::FromStr;
use file_reader::FileReader;
use file_reader::FatChar;
use finite_state_machine::FSMachine;
use std::collections::HashMap;

pub enum TokenType {
	t_int,
}

lazy_static! {
    static ref token_by_state: HashMap<String, TokenType> = {
    	let mut m = HashMap::new();
        m.insert("int".to_string(), TokenType::t_int);
        m
    };
}

pub struct Point { pub x: i32, pub y: i32 }

pub struct Token {
	pub token_type: TokenType,
	pub value: String,
	pub text: String,
	pub coords: Point
}

impl Token {
	fn new() -> Token {
		Token {
			token_type: TokenType::t_int,
			value: "".to_string(),
			text: "".to_string(),
			coords: Point { x: 0, y: 0 },
		}
	}
}

pub struct Tokenizer {
	tokens: Vec<Token>,
	machine: FSMachine,
}

impl Tokenizer {
	pub fn new() -> Tokenizer {
		Tokenizer {
			tokens: vec![],
			machine: FSMachine::new(),
		}
	}

	pub fn run(&mut self, file_name: &str) {
		let mut reader = FileReader::new(file_name);
		let mut x = 1;
		let mut y = 1;
		
		'one: loop {
			let mut state = "start".to_string();
			let mut text = "".to_string();
			let mut new_token = Token::new();
			new_token.coords = Point{ x: x, y: y };

			while (state != "end") {
				match reader.next_char() {
					FatChar::Char{ch} => {
						if (ch == '\n') {
							y += 1;
							x = 1;
						}
						else if (ch == '\t') { 
							x += 4 - ((x - 1) % 4); 
						}
						else { 
							x += 1; 
						}
						state = self.machine.step(ch);
						if (state != "end") {
							text += &(ch.to_string());
						}
					}, 
					FatChar::Eof => break 'one
				}
			}
			if (text != "") {
				new_token.text = text;
				println!("line: {}, col: {}, text: {}", new_token.coords.y, new_token.coords.x, new_token.text);
			}
		}
	}
}