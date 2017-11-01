use std::i32;
use TokenizerPack::file_reader::*;
use TokenizerPack::finite_state_machine::FSMachine; 
use TokenizerPack::support::*;
use TokenizerPack::token::Token;
use support::*;

pub struct Tokenizer {
	machine: FSMachine,

	pointer: Point,

	line_comment: bool,
	depth_comment: i32,

	reader: FileReader,
	pub current: Token,
	pub after:   Token,
}

pub enum ErrorState {
	Critical{err: TokenizerErrors},
	EmptyToken,
	EndOfFile,
}

impl Tokenizer {
	pub fn new(file_name: String) -> Tokenizer {
		let reader = FileReader::new(&file_name);

		Tokenizer {
			machine: FSMachine::new(),

			pointer: Point { x: 0, y: 1 },

			line_comment: false,
			depth_comment: 0,

			reader:  reader,
			current: Token::default(),
			after:   Token::default(),
		}
	}

	fn check_comments(&mut self, ch: char) -> bool {
		if self.line_comment {
			if ch == '\n' {
				self.line_comment = false;
			} 

			return true;
		}

		if ch == '{' {
			self.depth_comment += 1;
		} 

		if self.depth_comment > 0 {
			if ch == '}' {
				self.depth_comment -= 1;
			} 
			return true;
		}

		return false;
	}

	fn move_pointer(&mut self, ch: char) {
		if ch == '\n' {
			self.pointer.y += 1;
			self.pointer.x = 0;
		}
		else if ch == '\t' { 
			self.pointer.x += 4 - (self.pointer.x % 4); 
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

		while state != "end" {
			match self.reader.next_char() {
				FatChar::Char{ch} => {
					self.move_pointer(ch);
					if state == "start" { token_coords = self.pointer.clone(); }
					if self.check_comments(ch) { continue }
					
					state = self.machine.step(ch);
					if state == "lc" {
						self.line_comment = true;

						state = "end".to_string();
						self.machine.init();
						continue
					}

					if state == "none" {
						let error = ErrorState::Critical{err: TokenizerErrors::SimpleError{ x: self.pointer.y, y: self.pointer.x}};
						return Err(error);
					}
					else if state != "end" {
						text += &(ch.to_string());
						token_type_str = state.clone();

						if token_type_str == "fat_range" {
							text.pop();
							text.pop();

							match Token::new("int".to_string(), text, token_coords) {
								Ok(token) => {
									self.pointer = Point{ x: self.pointer.x - 2, y: self.pointer.y };

									self.machine.init();
									self.reader.push_back('.');
									self.reader.push_back('.');

									return Ok(token);
								},
								Err(err) => {
									let error = ErrorState::Critical{err};
									return Err(error);
								},
							}
						}
					}
					else if text != "" {
						self.reader.push_back(ch);
						self.pointer = Point{ x: self.pointer.x - 1, y: self.pointer.y };
						match Token::new(token_type_str, text, token_coords) {
							Ok(token) => return Ok(token),
							Err(err) => {
								let error = ErrorState::Critical{err};
								return Err(error);
							},
						}
					}
				}, 
				FatChar::Eof => { 
					if text != "" {
						match Token::new(token_type_str, text, token_coords) {
							Ok(token) => return Ok(token),
							Err(err) => {
								let error = ErrorState::Critical{err};
								return Err(error);
							},
						}
					}

					return Err(ErrorState::EndOfFile);
				}
			}
		}

		return Err(ErrorState::EmptyToken);
	}

	pub fn my_next(&mut self) -> Result<Token, CompilerErrors> {
    	loop {
        	match self.next_token() {
				Ok(token) => {
					self.current = self.after.clone();
					self.after = token.clone();
					return Ok(token);
				},
				Err(error) => match error {
					ErrorState::Critical{err} => { return Err(CompilerErrors::TokenizerError{err}); },
					ErrorState::EndOfFile => { 
						let token = Token {
							token_type:     TokenType::TEof,
							value:          Value::Str{ v: "0".to_string() },
							text:           "".to_string(),
							coords:         self.pointer.clone(),
						};

						self.current = self.after.clone();
						self.after = token.clone();
						return Ok(token); 
					},
					ErrorState::EmptyToken => {},
				}
			}
		}
    }
}



impl Iterator for Tokenizer {
    type Item = Result<Token, TokenizerErrors>;

    fn next(&mut self) -> Option<Result<Token, TokenizerErrors>> {
    	loop {
        	match self.next_token() {
				Ok(token) => {
					self.current = self.after.clone();
					self.after = token.clone();
					return Some(Ok(token));
				},
				Err(error) => match error {
					ErrorState::Critical{err} => { return Some(Err(err)); },
					ErrorState::EndOfFile => { 
						self.current = self.after.clone();
						self.after = Token {
							token_type:     TokenType::TEof,
							value:          Value::Str{ v: "0".to_string() },
							text:           "".to_string(),
							coords:         self.pointer.clone(),
						};
						return None; 
					},
					ErrorState::EmptyToken => {},
				}
			}
		}
    }
}