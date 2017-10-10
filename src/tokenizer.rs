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
use std::fmt;

#[derive(Clone)]
pub enum TokenType {
	t_int,	     t_double,
	t_hex, 	     t_minus,
	t_plus,      t_id,
	t_point,     t_colom,
	t_semicolom, t_eq,
	t_assign,    t_range,  

	t_op, 	t_cp,
	t_obr, 	t_cbr,

}

lazy_static! {
    static ref token_by_state: HashMap<String, TokenType> = {
    	let mut m = HashMap::new();
        m.insert("int".to_string(),        TokenType::t_int);
        m.insert("double".to_string(),     TokenType::t_double);
        m.insert("hex".to_string(),        TokenType::t_hex);
        m.insert("minus".to_string(),      TokenType::t_minus);
        m.insert("plus".to_string(),       TokenType::t_plus);
        m.insert("id".to_string(),         TokenType::t_id);
        m.insert("point".to_string(),      TokenType::t_point);
        m.insert("colon".to_string(),      TokenType::t_colom);
        m.insert("semicolon".to_string(),  TokenType::t_semicolom);
        m.insert("eq".to_string(),         TokenType::t_eq);
        m.insert("assign".to_string(),     TokenType::t_assign);
        m.insert("op".to_string(),         TokenType::t_op);
        m.insert("cp".to_string(),         TokenType::t_cp);
        m.insert("obr".to_string(),        TokenType::t_obr);
        m.insert("cbr".to_string(),        TokenType::t_cbr);
        m.insert("range".to_string(),      TokenType::t_range);
        m
    };
}

#[derive(Clone)]
pub struct Point { pub x: i32, pub y: i32 }

#[derive(Clone)]
pub struct Token {
	token_type: TokenType,

	pub token_type_str: String,
	pub value: String,
	pub text: String,
	pub coords: Point
}

impl Token {
	fn new() -> Token {
		Token {
			token_type: TokenType::t_int,
			token_type_str: "".to_string(),
			value: "".to_string(),
			text: "".to_string(),
			coords: Point { x: 0, y: 0 },
		}
	}
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "\t{:6} {:6} {:15} {:15} {:15}",  self.coords.y.to_string(), self.coords.x.to_string(), self.token_type_str, self.value, self.text)
    }
}

pub struct Tokenizer {
	pub tokens: Vec<Token>,
	machine: FSMachine,

	pointer: Point
}

impl Tokenizer {
	pub fn new() -> Tokenizer {
		Tokenizer {
			tokens: vec![],
			machine: FSMachine::new(),

			pointer: Point { x: 1, y: 1 },
		}
	}

	pub fn move_pointer(&mut self, ch: char) {
		if (ch == '\n') {
			self.pointer.y += 1;
			self.pointer.x = 1;
		}
		else if (ch == '\t') { 
			self.pointer.x += 4 - ((self.pointer.x - 1) % 4); 
		}
		else { 
			self.pointer.x += 1; 
		}
	}

	pub fn run(&mut self, file_name: &str) {
		let mut reader = FileReader::new(file_name);
		self.pointer = Point { x: 1, y: 1 };
		
		'one: loop {
			let mut state = "start".to_string();
			let mut text = "".to_string();
			let mut token_coords = self.pointer.clone();
			let mut token_type = TokenType::t_int;
			let mut token_type_str = "".to_string();

			while (state != "end") {
				match reader.next_char() {
					FatChar::Char{ch} => {
						self.move_pointer(ch);
						state = self.machine.step(ch);

						if (state != "end") {
							text += &(ch.to_string());
							token_type = (*token_by_state.get(&state).unwrap()).clone();
							token_type_str = state.clone();
						}
						else if (text != "") {
							reader.push_back(ch);

							if (token_type_str == "int") {
								if (self.tokens.len() > 2) {
									if (self.tokens[self.tokens.len() - 1].token_type_str == "point" && self.tokens[self.tokens.len() - 2].token_type_str == "int") {
										text = self.tokens[self.tokens.len() - 2].text.clone() + &self.tokens[self.tokens.len() - 1].text.clone() + &text;
										token_type_str = "double".to_string();
										self.tokens.pop();
										self.tokens.pop();
									}
								}
							}
							let new_token = Token {
								token_type: token_type.clone(),
								token_type_str: token_type_str.clone(),
								value: text.clone(),
								text: text.clone(),
								coords: token_coords.clone(),
							};

							self.tokens.push(new_token);
						}
					}, 
					FatChar::Eof => { 
						if (text != "") {
							let new_token = Token {
								token_type: token_type.clone(),
								token_type_str: token_type_str.clone(),
								value: text.clone(),
								text: text.clone(),
								coords: token_coords.clone(),
							};

							self.tokens.push(new_token);
						}

						break 'one;
					}
				}
			}
		}
	}
}