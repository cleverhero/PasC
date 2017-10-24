use std::io::prelude::*;
use std::f64;
use std::i64;
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, name_by_field)]
pub enum TokenType {
	TColom,	     TDouble,
    THex, 	     TMinus,
	TPlus,       TId,
	TPoint,      TInt,
	TSemicolom,  TComma, 
	TAssign,     TRange,  
    TMul,        TShare,  
    TLiteral,    TString,

	TOp, 	     TCp,
	TObr, 	     TCbr,
     
	TGe, 	     TLe,
	TGt, 	     TLt,
	TEq,         TNe,

    TExp,        TKeyword,
     
    TDog,        TLid,
    TOctal,      TGrill,
    TChar,       THexChar,
    TOctalChar,
    TBin,        TBinChar,
    TEof,
}

lazy_static! {
    pub static ref TYPE_BY_STATE: HashMap<String, TokenType> = {
    	let mut m = HashMap::new();
        m.insert("int".to_string(),        TokenType::TInt);
        m.insert("double".to_string(),     TokenType::TDouble);
        m.insert("hex".to_string(),        TokenType::THex);
        m.insert("bin".to_string(),        TokenType::TBin);
        m.insert("minus".to_string(),      TokenType::TMinus);
        m.insert("plus".to_string(),       TokenType::TPlus);
        m.insert("id".to_string(),         TokenType::TId);
        m.insert("point".to_string(),      TokenType::TPoint);
        m.insert("colon".to_string(),      TokenType::TColom);
        m.insert("semicolon".to_string(),  TokenType::TSemicolom);
        m.insert("eq".to_string(),         TokenType::TEq);
        m.insert("assign".to_string(),     TokenType::TAssign);
        m.insert("comma".to_string(),      TokenType::TComma);
        m.insert("range".to_string(),      TokenType::TRange);
        m.insert("mul".to_string(),        TokenType::TMul);
        m.insert("share".to_string(),      TokenType::TShare);
        m.insert("literal".to_string(),    TokenType::TLiteral);
        m.insert("string".to_string(),     TokenType::TString);

        m.insert("op".to_string(),         TokenType::TOp);
        m.insert("cp".to_string(),         TokenType::TCp);
        m.insert("obr".to_string(),        TokenType::TObr);
        m.insert("cbr".to_string(),        TokenType::TCbr);

        m.insert("ge".to_string(),         TokenType::TGe);
        m.insert("gt".to_string(),         TokenType::TGt);
        m.insert("eq".to_string(),         TokenType::TEq);
        m.insert("le".to_string(),         TokenType::TLe);
        m.insert("lt".to_string(),         TokenType::TLt);
        m.insert("ne".to_string(),         TokenType::TNe);

        m.insert("exp".to_string(),        TokenType::TExp);
        m.insert("key_word".to_string(),   TokenType::TKeyword);

        m.insert("octal".to_string(),      TokenType::TOctal);
        m.insert("lid".to_string(),        TokenType::TLid);
        m.insert("dog".to_string(),        TokenType::TDog);
        m.insert("grill".to_string(),      TokenType::TGrill);
        m.insert("char".to_string(),       TokenType::TChar);
        m.insert("hex_char".to_string(),   TokenType::THexChar);
        m.insert("octal_char".to_string(), TokenType::TOctalChar);
        m.insert("bin_char".to_string(),   TokenType::TBinChar);

        m
    };
}


pub static KEY_WORDS: &'static [&'static str] = &["and", "array", "begin", 
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
	pub token_type: TokenType,

	pub value: String,
	pub text: String,
	pub coords: Point
}

impl Token {
    pub fn default() -> Token {
        Token {
            token_type: TokenType::TId,

            value: "0".to_string(),
            text: "0".to_string(),
            coords: Point { x:0, y: 0},
        }
    }

	pub fn new(machine_state: String, text: String, coords: Point) -> Result<Token, String> {
		let mut token_type_str = machine_state;
        let mut value = text.clone();
		if token_type_str == "id" {
			for i in KEY_WORDS {
				if i.to_string() == text {
					token_type_str = "key_word".to_string();
                    break;
				}
			}
		}
        

        if token_type_str == "string" {
            value = text[1..text.len() - 1].to_string();
        }
        if token_type_str == "int" {
            match i64::from_str(&text) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => return Err(format!("{} => Error in format of Integer", e)),
            }
        }
        if token_type_str == "hex" {
            let hstr = &(text[1..text.len()].to_string());

            match i64::from_str_radix(&hstr, 16) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => return Err(format!("{} => Error in format of Hex", e)),
            }
        }
        if token_type_str == "bin" {
            let hstr = &(text[1..text.len()].to_string());

            match i64::from_str_radix(&hstr, 2) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => return Err(format!("{} => Error in format of Bin", e)),
            }
        }
        if token_type_str == "octal" {
            let hstr = &(text[1..text.len()].to_string());

            match i64::from_str_radix(&hstr, 8) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => return  Err(format!("{} => Error in format of Octal", e)),
            }
        }
        if token_type_str == "double" {
            match f64::from_str(&text) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => return  Err(format!("{} => Error in format of Double", e)),
            }
        }
        if token_type_str == "exp" {
            match f64::from_str(&text) {
                Ok(x) => {
                    value = x.to_string();
                },
                Err(e) => return  Err(format!("{} => Error in format of Exp", e)),
            }
        }
        if token_type_str == "char" {
            let hstr = &(text[1..text.len()].to_string());
            let mut i = 0;

            match i64::from_str(&text){
                Ok(x) => {
                    i = x;
                },
                Err(e) => return Err(format!("{} => Error in format of Integer", e)),
            }

            if i <= 127 && i >= 0 {
                unsafe {
                    value = ((i as u8) as char).to_string();
                }
            }
            else {
                return Err(format!("Unknown character code {}", text[1..text.len()].to_string()));
            }
        }
        if token_type_str == "hex_char" {
            token_type_str = "char".to_string();
            let hstr = &(text[2..text.len()].to_string());
            let mut i = 0;
            match i64::from_str_radix(&hstr, 16) {
                Ok(x) => {
                    i = x;
                },
                Err(e) => return Err(format!("{} => Error in format of Hex", e)),
            }

            if i <= 127 && i >= 0 {
                unsafe {
                    value = ((i as u8) as char).to_string();
                }
            }
            else {
                return Err(format!("Unknown character code {}", text[1..text.len()].to_string()));
            }
        } 

        if token_type_str == "bin_char" {
            token_type_str = "char".to_string();
            let hstr = &(text[2..text.len()].to_string());
            let mut i = 0;
            match i64::from_str_radix(&hstr, 2) {
                Ok(x) => {
                    i = x;
                },
                Err(e) => return Err(format_args!("{} => Error in format of Bin", e).to_string()),
            }

            if i <= 127 && i >= 0 {
                unsafe {
                    value = ((i as u8) as char).to_string();
                }
            }
            else {
                return Err(format_args!("Unknown character code {}", text[1..text.len()].to_string()).to_string());
            }
        } 

        if token_type_str == "octal_char" {
            token_type_str = "char".to_string();
            let hstr = &(text[2..text.len()].to_string());
            let mut i = 0;
            match i64::from_str_radix(&hstr, 8) {
                Ok(x) => {
                    i = x;
                },
                Err(e) => return Err(format_args!("{} => Error in format of Bin", e).to_string()),
            }

            if i <= 127 && i >= 0 {
                value = ((i as u8) as char).to_string();
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
    	write!(f, "\t{:6} {:6} {:15} {:25} {:25}",  self.coords.y.to_string(), self.coords.x.to_string(), self.token_type.fields_name(), self.value, self.text)
    }
}