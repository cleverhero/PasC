use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

pub struct Parser;

impl Parser {
	pub fn load_form_csv(file_name: &str) -> HashMap<String, Vec<String>> {
		let file = File::open(file_name).unwrap();
    	let mut reader = BufReader::new(file);
	
    	let mut t_table: HashMap<String, Vec<String>> = HashMap::new();
    	
    	let mut line = String::new();
    	reader.read_line(&mut line).unwrap();
    	let cols: Vec<&str> = line.trim().split("|").collect();
	
		let mut len = 1;
    	'gl: while len != 0 {
    		let mut row = String::new();
    		len = reader.read_line(&mut row).unwrap();
    		let mut new_state: Vec<String> = vec!["end".to_string(); 255];


    		let cells: Vec<&str> = row.trim().split("|").collect();
    		for i in 1 .. cells.len() {
    			let mut range = cols[i];
	            if cells[0] == "literal" {
                    let mut new_state = vec!["literal".to_string(); 255];
                    new_state["'".as_bytes()[0] as usize] = "string".to_string();
                    t_table.insert(cells[0].to_string(), new_state);
                    continue 'gl;
                }
    			if range.len() == 1 {
                    if range == "\\" { range = "/"; }
    				new_state[range.as_bytes()[0] as usize] = cells[i].to_string();
    			}
    			else {
    				let segments: Vec<&str> = range.split(" ").collect();
    				for segment in segments {
    					for j in segment.as_bytes()[0] as usize .. (segment.as_bytes()[2] + 1) as usize {
    						new_state[j] = cells[i].to_string();
    					} 
    				}
    			}
    		}
    		t_table.insert(cells[0].to_string(), new_state);
    	}

    	t_table
	}
}