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

	pub fn parse(&mut self) -> Tree {
		let e = Box::new(ConstNode::new(0));
		let mut t = match self.tokenizer.next() {
    		Some(res) => match res {
    			Ok(token) => token,
   				Err(msg) => { return Tree::new(e); }
   			}
   			None => { return Tree::new(e); }
    	};

		Tree::new(self.parseExpr())
	} 

	fn parseExpr(&mut self) -> Box<Node> {
		let mut e = self.parseTerm();
    	let mut t = self.tokenizer.current.clone();

    	while match t.token_type {
    		TokenType::TPlus | TokenType::TMinus => true,
    		_ => false
    	} {
    		self.tokenizer.next();
    		let right = self.parseTerm();
    	    e = Box::new(BinNode::new(e, right, t));
    	    t = self.tokenizer.current.clone();
    	}

		e
	}

	fn parseTerm(&mut self) -> Box<Node> {
		let mut e = self.parseFactor();
    	let mut t = self.tokenizer.current.clone();

    	while match t.token_type {
    		TokenType::TMul | TokenType::TShare => true,
    		_ => false
    	} {
    		self.tokenizer.next();
    		let right = self.parseFactor();
    	    e = Box::new(BinNode::new(e, right, t));
    	    t = self.tokenizer.current.clone();
    	}

		return e;
	}

	fn parseFactor(&mut self) -> Box<Node> {
		let mut t = self.tokenizer.current.clone();
		self.tokenizer.next();
    	match t.token_type {
    		TokenType::TDouble => {
    			let value = f64::from_str(&t.value).unwrap();
    			return Box::new(ConstNode::new(value))
    		},
    		TokenType::TInt => {
    			let value = i64::from_str(&t.value).unwrap();
    			return Box::new(ConstNode::new(value))
    		},
    		TokenType::TId => {
    			return Box::new(IdNode::new(t.value))
    		},
    		TokenType::TOp => {
           		let e = self.parseExpr();
            	self.tokenizer.next();
				return e;
			} 
    		_ => return Box::new(ConstNode::new(0))
    	}

    	//return Box::new(ConstNode::new(0));
	}
}