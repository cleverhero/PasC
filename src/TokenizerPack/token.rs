use TokenizerPack::support::*;
use std::str::FromStr;
use std::fmt;

#[derive(Clone)]
pub struct Token {
	pub token_type: TokenType,

	pub value: Value,
	pub text: String,
	pub coords: Point
}



impl Token {
    pub fn default() -> Token {
        Token {
            token_type: TokenType::TId,

            value: Value::Str{ v: "0".to_string() },
            text: "0".to_string(),
            coords: Point { x:0, y: 0},
        }
    }

	pub fn new(machine_state: String, text: String, coords: Point) -> Result<Token, String> {
		let mut token_type_str = machine_state;
        let mut value = Value::Str{ v: text.clone() };
		if token_type_str == "id" {
			for i in KEY_WORDS {
				if i.to_string() == text {
					token_type_str = "key_word".to_string();
                    break;
				}
			}
		}
        

        if token_type_str == "string" {
            value = Value::Str{ v: text[1..text.len() - 1].to_string() };
        }
        if token_type_str == "int" {
            match i64::from_str(&text) {
                Ok(x) => {
                    value = Value::Int{ v: x };
                },
                Err(e) => return Err(format!("{} => Error in format of Integer", e)),
            }
        }
        if token_type_str == "hex" {
            let hstr = &(text[1..text.len()].to_string());

            match i64::from_str_radix(&hstr, 16) {
                Ok(x) => {
                    value = Value::Int{ v: x };
                },
                Err(e) => return Err(format!("{} => Error in format of Hex", e)),
            }
        }
        if token_type_str == "bin" {
            let hstr = &(text[1..text.len()].to_string());

            match i64::from_str_radix(&hstr, 2) {
                Ok(x) => {
                    value = Value::Int{ v: x };
                },
                Err(e) => return Err(format!("{} => Error in format of Bin", e)),
            }
        }
        if token_type_str == "octal" {
            let hstr = &(text[1..text.len()].to_string());

            match i64::from_str_radix(&hstr, 8) {
                Ok(x) => {
                    value = Value::Int{ v: x };
                },
                Err(e) => return  Err(format!("{} => Error in format of Octal", e)),
            }
        }
        if token_type_str == "double" {
            match f64::from_str(&text) {
                Ok(x) => {
                    value = Value::Double{ v: x };
                },
                Err(e) => return  Err(format!("{} => Error in format of Double", e)),
            }
        }
        if token_type_str == "exp" {
            match f64::from_str(&text) {
                Ok(x) => {
                    value = Value::Double{ v: x };
                },
                Err(e) => return  Err(format!("{} => Error in format of Exp", e)),
            }
        }
        if token_type_str == "char" {
            let hstr = &(text[1..text.len()].to_string());
            let i = match i64::from_str(&hstr) {
                Ok(x) => { x },
                Err(e) => return Err(format!("{} => Error in format of Integer", e)),
            };

            if i <= 127 && i >= 0 {
                value = Value::Str{ v: ((i as u8) as char).to_string() };
            }
            else {
                return Err(format!("Unknown character code {}", text[1..text.len()].to_string()));
            }
        }
        if token_type_str == "hex_char" {
            token_type_str = "char".to_string();
            let hstr = &(text[2..text.len()].to_string());
            let i = match i64::from_str_radix(&hstr, 16) {
                Ok(x) => { x },
                Err(e) => return Err(format!("{} => Error in format of Hex", e)),
            };

            if i <= 127 && i >= 0 {
                value = Value::Str{ v: ((i as u8) as char).to_string() };
            }
            else {
                return Err(format!("Unknown character code {}", text[1..text.len()].to_string()));
            }
        } 

        if token_type_str == "bin_char" {
            token_type_str = "char".to_string();
            let hstr = &(text[2..text.len()].to_string());
            let i = match i64::from_str_radix(&hstr, 2) {
                Ok(x) => { x },
                Err(e) => return Err(format_args!("{} => Error in format of Bin", e).to_string()),
            };

            if i <= 127 && i >= 0 {
                value = Value::Str{ v: ((i as u8) as char).to_string() };
            }
            else {
                return Err(format_args!("Unknown character code {}", text[1..text.len()].to_string()).to_string());
            }
        } 

        if token_type_str == "octal_char" {
            token_type_str = "char".to_string();
            let hstr = &(text[2..text.len()].to_string());
            let i = match i64::from_str_radix(&hstr, 8) {
                Ok(x) => { x },
                Err(e) => return Err(format_args!("{} => Error in format of Bin", e).to_string()),
            };

            if i <= 127 && i >= 0 {
                value = Value::Str{ v: ((i as u8) as char).to_string() };
            }
            else {
                return Err(format_args!("Unknown character code {}", text[1..text.len()].to_string()).to_string());
            }
        }
		Ok(Token {
			token_type:     (*TYPE_BY_STATE.get(&token_type_str).unwrap()).clone(),
			value:          value,
			text:           text,
			coords:         coords,
		})
	}
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "\t{:6} {:6} {:15}{}{:25} {:25}",  self.coords.y.to_string(), 
                                                    self.coords.x.to_string(), 
                                                    self.token_type.fields_name(), 
                                                    self.value, "", self.text)
    }
}