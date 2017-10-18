use std::collections::HashMap;
use TokenizerPack::csv_parser::Parser;


pub struct FSMachine {
	state: String,
	
	t_table: HashMap<String, Vec<String>>,
}

impl FSMachine {
	pub fn new() -> FSMachine {
		let file_csv = "TransitionTable.csv";
		let mut t_table = Parser::load_form_csv(file_csv);

		FSMachine {
			state: "start".to_string(),
			t_table: t_table,
		}
	}

	pub fn step(&mut self, ch: char) -> String {
		if (ch == '\n' || ch == '\r') {
			self.state = "start".to_string();
			return "end".to_string()
		}
		let mut ind: u8 = 0;
		unsafe { ind = ch as u8; }
		let new_state = self.t_table.get(&self.state).unwrap()[ind as usize].clone();

		if (new_state != "end") {
			self.state = new_state.clone();
		}
		else {
			self.state = "start".to_string();
		}
		new_state
	}

	pub fn init(&mut self) {
		self.state = "start".to_string();
	}
}