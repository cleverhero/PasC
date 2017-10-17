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
	machine: FSMachine,

	pointer: Point,
	curr_token: usize,

	line_comment: bool,
	depth_comment: i32,

	reader: FileReader,
}

pub enum ErrorState {
	Critical{msg: String},
	EmptyToken,
	EndOfFile,
}

impl Tokenizer {
	pub fn new(file_name: String) -> Tokenizer {
		let mut reader = FileReader::new(&file_name);

		Tokenizer {
			machine: FSMachine::new(),

			pointer: Point { x: 0, y: 1 },
			curr_token: 0,

			line_comment: false,
			depth_comment: 0,

			reader: reader,
		}
	}

	fn check_comments(&mut self, ch: char) -> bool {
		if (self.line_comment) {
			if (ch == '\n') {
				self.line_comment = false;
			} 

			return true;
		}

		if (ch == '{') {
			self.depth_comment += 1;
		} 

		if (self.depth_comment > 0) {
			if (ch == '}') {
				self.depth_comment -= 1;
			} 
			return true;
		}

		return false;
	}

	fn move_pointer(&mut self, ch: char) {
		if (ch == '\n') {
			self.pointer.y += 1;
			self.pointer.x = 0;
		}
		else if (ch == '\t') { 
			self.pointer.x += 4 - ((self.pointer.x - 1) % 4); 
		}
		else { 
			self.pointer.x += 1; 
		}
	}

	fn next_token(&mut self) -> Result<Token, ErrorState> {
		let mut state = "start".to_string();
		let mut text = "".to_string();
		let mut token_coords = self.pointer.clone();
		let mut token_type_str = "".to_string();

		while (state != "end") {
			match self.reader.next_char() {
				FatChar::Char{ch} => {
					self.move_pointer(ch);
					if (state == "start") { token_coords = self.pointer.clone(); }
					if self.check_comments(ch) { continue }
					
					state = self.machine.step(ch);
					if (state == "lc") {
						self.line_comment = true;

						state = "end".to_string();
						self.machine.init();
						continue
					}
					if (state == "none") {
						let Error = ErrorState::Critical{msg: format!("Ошибка в ({}, {})", self.pointer.y, self.pointer.x)};
						return Err(Error);
					}
					else if (state != "end") {
						text += &(ch.to_string());
						token_type_str = state.clone();

						if (token_type_str == "fat_range") {
							text.pop();
							text.pop();
							let new_token = Token::new("int".to_string(), text, token_coords);
							
							self.pointer = Point{ x: self.pointer.x - 2, y: self.pointer.y };

							self.machine.init();
							self.reader.push_back('.');
							self.reader.push_back('.');

							return Ok(new_token);
						}
					}
					else if (text != "") {
						self.reader.push_back(ch);
						let new_token = Token::new(token_type_str, text, token_coords);

						text = "".to_string();
						token_type_str = "".to_string();
						token_coords = self.pointer.clone();

						return Ok(new_token);
					}
				}, 
				FatChar::Eof => { 
					if (text != "") {
						let new_token = Token::new(token_type_str, text, token_coords);

						text = "".to_string();
						token_type_str = "".to_string();
						token_coords = self.pointer.clone();

						return Ok(new_token);
					}

					return Err(ErrorState::EndOfFile);
				}
			}
		}

		return Err(ErrorState::EmptyToken);
	}
}


impl Iterator for Tokenizer {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Result<Token, String>> {
    	loop {
        	match self.next_token() {
				Ok(token) => {
					return Some(Ok(token));
				},
				Err(Error) => match Error {
					ErrorState::Critical{msg} => { return Some(Err(msg)); },
					ErrorState::EndOfFile => { return None },
					ErrorState::EmptyToken => {},
				}
			}
		}
    }
}