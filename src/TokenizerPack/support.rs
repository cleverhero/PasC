use std::f64;
use std::i64;
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
pub enum Value {
    Int{ v: i64},
    Double{ v: f64},
    Str{ v: String}
}

impl Value {
    pub fn as_string(&self) -> String {
        match self {
            &Value::Str{ref v} => v.clone(),
            _ => "".to_string(),
        }
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            Value::Int{v} => v,
            _ => 0,
        }
    }

    pub fn as_double(&self) -> f64 {
        match *self {
            Value::Double{v} => v,
            _ => 0.0,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int{ref v} => write!(f, "{}", v.clone()),
            &Value::Double{ref v} => write!(f, "{}", v.clone()),
            &Value::Str{ref v} => write!(f, "{}", v.clone()),
        }
    }
}

