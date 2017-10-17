use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f32;
use std::i32;
use std::str::FromStr;

pub enum FatChar {
	Char{ch: char},
	Eof,
}

pub struct FileReader {
	curr_line: String,
	buffer: Vec<char>,
	pointer: i32,

	reader: BufReader<File>,
}

impl FileReader {
	pub fn new(file_name: &str) -> FileReader {
		let file = File::open(file_name).unwrap();
		let mut reader = BufReader::new(file);

		let mut line = String::new();
   		let len = reader.read_line(&mut line).unwrap();

		FileReader {
			reader: reader,
			curr_line: line,
			buffer: vec![],
			pointer: 0,
		}
	}

	pub fn next_char(&mut self) -> FatChar {
		if (self.buffer.len() != 0) {
			let ch = self.buffer.pop().unwrap();
			return FatChar::Char{ ch }
		}
		if (self.curr_line.len() == 0) {
			return FatChar::Eof;
		}

		let chs: Vec<char> = self.curr_line.chars().collect();
		let ch = chs[self.pointer as usize];
		if (self.pointer == (self.curr_line.len() - 1) as i32) {
			self.pointer = 0;
			self.curr_line.clear();

			let mut line = String::new();
			self.reader.read_line(&mut line).unwrap();
			self.curr_line = line;
		}
		else {
			self.pointer += 1;
		}

		FatChar::Char{ch: ch}
	}

	pub fn push_back(&mut self, ch: char) {
		self.buffer.insert(0, ch);
	}
}