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
use support::*;

pub struct Tokenizer {
	pub tokens: Vec<Token>,
	machine: FSMachine,

	pointer: Point,
	curr_token: usize,

	line_comment: bool,
	depth_comment: i32,
}

impl Tokenizer {
	pub fn new() -> Tokenizer {
		Tokenizer {
			tokens: vec![],
			machine: FSMachine::new(),

			pointer: Point { x: 1, y: 1 },
			curr_token: 0,

			line_comment: false,
			depth_comment: 0,
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

	pub fn run(&mut self, file_name: String) {
		let mut reader = FileReader::new(&file_name);
		self.pointer = Point { x: 1, y: 1 };
		
		'one: loop {
			let mut state = "start".to_string();
			let mut text = "".to_string();
			let mut token_coords = self.pointer.clone();
			let mut token_type_str = "".to_string();

			while (state != "end") {
				match reader.next_char() {
					FatChar::Char{ch} => {
						if (self.line_comment) {
							if (ch == '\n') {
								self.line_comment = false;
							} 

							continue;
						}

						if (ch == '{') {
							self.depth_comment += 1;
						} 

						if (self.depth_comment > 0) {
							if (ch == '}') {
								self.depth_comment -= 1;
							} 
							continue
						}

						self.move_pointer(ch);
						state = self.machine.step(ch);
						if (state == "lc") {
							self.line_comment = true;
							reader.push_back(ch);
							continue 'one;
						}
						if (state == "none") {
							println!("Ошибка в ({}, {})", self.pointer.y, self.pointer.x);
							return;
						}
						else if (state != "end") {
							text += &(ch.to_string());
							token_type_str = state.clone();

							if (state == "fat_range") {
								text.pop();
								text.pop();
								let new_token = Token::new("int".to_string(), text.clone(), token_coords.clone());
								self.tokens.push(new_token);

								token_type_str = "range".to_string();
								text = "..".to_string();
								token_coords = Point{ x: self.pointer.x - 2, y: self.pointer.y }
							}
						}
						else if (text != "") {
							reader.push_back(ch);

							let new_token = Token::new(token_type_str.clone(), text.clone(), token_coords.clone());
							self.tokens.push(new_token);
						}
					}, 
					FatChar::Eof => { 
						if (text != "") {
							let new_token = Token::new(token_type_str.clone(), text.clone(), token_coords.clone());
							self.tokens.push(new_token);
						}

						break 'one;
					}
				}
			}
		}
	}

	pub fn next(&mut self) -> Token {
		let res = self.tokens[self.curr_token].clone();

		if (self.curr_token != (self.tokens.len() - 1)) {
			self.curr_token += 1;
		}

		res
	}
}