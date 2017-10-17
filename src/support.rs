use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::f64;
use std::i64;
use std::str::FromStr;
use file_reader::FileReader;
use file_reader::FatChar;
use finite_state_machine::FSMachine;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, name_by_field)]
pub enum TokenType {
	t_int,	     t_double,
	t_hex, 	     t_minus,
	t_plus,      t_id,
	t_point,     t_colom,
	t_semicolom, t_comma, 
	t_assign,    t_range,  
    t_mul,       t_share,  
    t_literal,   t_string,

	t_op, 	     t_cp,
	t_obr, 	     t_cbr,
     
	t_ge, 	     t_le,
	t_gt, 	     t_lt,
	t_eq,        t_ne,
     
    t_exp,       t_keyword,
}

lazy_static! {
    pub static ref type_by_state: HashMap<String, TokenType> = {
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
        m.insert("comma".to_string(),      TokenType::t_comma);
        m.insert("range".to_string(),      TokenType::t_range);
        m.insert("mul".to_string(),        TokenType::t_mul);
        m.insert("share".to_string(),      TokenType::t_share);
        m.insert("literal".to_string(),    TokenType::t_literal);
        m.insert("string".to_string(),     TokenType::t_string);

        m.insert("op".to_string(),         TokenType::t_op);
        m.insert("cp".to_string(),         TokenType::t_cp);
        m.insert("obr".to_string(),        TokenType::t_obr);
        m.insert("cbr".to_string(),        TokenType::t_cbr);

        m.insert("ge".to_string(),         TokenType::t_ge);
        m.insert("gt".to_string(),         TokenType::t_gt);
        m.insert("eq".to_string(),         TokenType::t_eq);
        m.insert("le".to_string(),         TokenType::t_le);
        m.insert("lt".to_string(),         TokenType::t_lt);
        m.insert("ne".to_string(),         TokenType::t_ne);

        m.insert("exp".to_string(),        TokenType::t_exp);
        m.insert("key_word".to_string(),   TokenType::t_keyword);
        m
    };
}

pub static key_words: &'static [&'static str] = &["and", "array", "begin", 
                                        "case", "const", "div", "do", 
                                        "downto", "else", "end", "file", 
                                        "for", "function", "goto", "if", 
                                        "in", "label", "mod", "nil", "not", 
                                        "of", "or", "packed", "procedure", 
                                        "program", "record", "repeat", "set", 
                                        "then", "to", "type", "until", "var", 
                                        "while", "with"];

#[derive(Clone)]
pub struct Point { pub x: i32, pub y: i32 }

#[derive(Clone)]
pub struct Token {
	token_type: TokenType,

	pub value: String,
	pub text: String,
	pub coords: Point
}

impl Token {
	pub fn new(machine_state: String, text: String, coords: Point) -> Token {
		let mut token_type_str = machine_state;
        let mut value = text.clone();
		if (token_type_str == "id") {
			for i in key_words {
				if (i.to_string() == text) {
					token_type_str = "key_word".to_string();
                    break;
				}
			}
		}

        if (token_type_str == "string") {
            value = text[1..text.len() - 1].to_string();
        }
        if (token_type_str == "int") {
            match i64::from_str(&text) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => panic!("{} -> {}", text, e),
            }
        }
        if (token_type_str == "hex") {
            let hstr = &(text[2..text.len()].to_string());

            match i64::from_str_radix(&hstr, 16) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => panic!("{} -> {}", text, e),
            }
        }
        if (token_type_str == "double") {
            value = f64::from_str(&text).unwrap().to_string();
        }
        if (token_type_str == "exp") {
            value = f64::from_str(&text).unwrap().to_string();
        }
		Token {
			token_type:     (*type_by_state.get(&token_type_str).unwrap()).clone(),
			value:          value,
			text:           text,
			coords:         coords,
		}
	}
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "\t{:6} {:6} {:15} {:25} {:25}",  self.coords.y.to_string(), self.coords.x.to_string(), self.token_type.fields_name(), self.value, self.text)
    }
}