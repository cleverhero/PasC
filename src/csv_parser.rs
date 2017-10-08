use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f32;
use std::i32;
use std::str::FromStr;

pub struct Parser;

impl Parser {
	pub fn load_form_csv(file_name: &str) -> Vec<Vec<i32>> {
		let file = File::open(file_name).unwrap();
    	let mut reader = BufReader::new(file);
	
    	let mut t_table: Vec<Vec<i32>> = vec![];
    	
    	let mut line = String::new();
    	reader.read_line(&mut line).unwrap();
    	let cols: Vec<&str> = line.trim().split(",").collect();
	
		let mut len = 1;
    	while (len != 0) {
    		let mut row = String::new();
    		len = reader.read_line(&mut row).unwrap();
    		let mut new_state: Vec<i32> = vec![-1; 255];
	
    		let cells: Vec<&str> = row.trim().split(",").collect();
    		for i in (1 .. cells.len()) {
    			let range = cols[i];
	
    			if (range.len() == 1) {
    				new_state[range.as_bytes()[0] as usize] = i32::from_str(cells[i]).unwrap();
    			}
    			else {
    				let segments: Vec<&str> = range.split(" ").collect();
    				for segment in segments {
    					for j in (segment.as_bytes()[0] as usize .. (segment.as_bytes()[2] + 1) as usize) {
    						new_state[j] = i32::from_str(cells[i]).unwrap();
    					} 
    				}
    			}
    		}
    		t_table.push(new_state);
    	}

    	t_table
	}
}