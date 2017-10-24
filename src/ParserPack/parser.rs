use ParserPack::nodes::*;
use TokenizerPack::tokenizer::Tokenizer;
use TokenizerPack::support::*;
use std::i64;
use std::f64;
use std::str::FromStr;

pub struct Parser {
	tokenizer: Tokenizer,
}

impl Parser {
	pub fn new(tokenizer: Tokenizer) -> Parser {
		Parser {
			tokenizer
		}
	} 

	pub fn parse(&mut self) -> Result<Tree, String> {
		match self.tokenizer.next() {
    		Some(res) => match res {
    			Ok(token) => token,
   				Err(msg) => { return Err(msg); }
   			}
   			None => { return Err(String::from("Error: Not found tokens")); }
    	};

    	match self.parse_expr() {
    		Ok(node) => return Ok(Tree::new(node)),
    		Err(msg) => return Err(msg)
    	}
		
	} 

	fn parse_expr(&mut self) -> Result<Box<Node>, String> {
		let mut e = match self.parse_term() {
    		Ok(val) => val,
    		Err(msg) => return Err(msg)
    	};
    	let mut t = self.tokenizer.current.clone();

    	while match t.token_type {
    		TokenType::TPlus | TokenType::TMinus => true,
    		_ => false
    	} {
    		match self.tokenizer.next() {
    			Some(res) => match res {
    				Ok(_token) => {},
   					Err(msg) => return Err(msg)
   				}
   				None => { }
    		};
    		let right = match self.parse_term() {
    			Ok(val) => val,
    			Err(msg) => return Err(msg)
    		};
    	    e = Box::new(BinNode::new(e, right, t));
    	    t = self.tokenizer.current.clone();
    	}

		Ok(e)
	}

	fn parse_term(&mut self) -> Result<Box<Node>, String> {
		let mut e = match self.parse_factor() {
    		Ok(val) => val,
    		Err(msg) => return Err(msg)
    	};
    	let mut t = self.tokenizer.current.clone();

    	while match t.token_type {
    		TokenType::TMul | TokenType::TShare => true,
    		_ => false
    	} {
    		match self.tokenizer.next() {
    			Some(res) => match res {
    				Ok(_token) => {},
   					Err(msg) => return Err(msg)
   				}
   				None => { }
    		};
    		let right = match self.parse_factor() {
    			Ok(val) => val,
    			Err(msg) => return Err(msg)
    		};
    	    e = Box::new(BinNode::new(e, right, t));
    	    t = self.tokenizer.current.clone();
    	}

		Ok(e)
	}

	fn parse_factor(&mut self) -> Result<Box<Node>, String> {
		let t = self.tokenizer.current.clone();
		match self.tokenizer.next() {
    		Some(res) => match res {
    			Ok(_token) => {},
   				Err(msg) => return Err(msg)
   			}
   			None => { }
    	};
    	match t.token_type {
    		TokenType::TDouble => {
    			let value = f64::from_str(&t.value).unwrap();
    			return Ok(Box::new(ConstNode::new(value)))
    		},
    		TokenType::TInt => {
    			let value = i64::from_str(&t.value).unwrap();
    			return Ok(Box::new(ConstNode::new(value)))
    		},
    		TokenType::TId => {
    			return Ok(Box::new(IdNode::new(t.value)))
    		},
    		TokenType::TOp => {
           		let e = self.parse_expr();

            	let t = self.tokenizer.current.clone();
            	match t.token_type {
            		TokenType::TCp => {},
            		_ => return Err(String::from("Error: Missing closing bracket"))
            	}

            	match self.tokenizer.next() {
    				Some(res) => match res {
    					Ok(_token) => {},
   						Err(msg) => return Err(msg)
   					}
   					None => { }
    			};

				return e;
			} 
    		_ => return Err(String::from("Error: Missing operand"))
    	}
	}
}