use std::fmt;
use TokenizerPack::token::Token;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct BinNode {
	pub childrens: Vec<Rc<Box<Node>>>,

	pub op: Token,
}

impl BinNode {
	pub fn new(op: Token) -> BinNode {
		BinNode {
			childrens: vec![],
			op: op,
		}
	}
}

impl Display for BinNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl Printable for BinNode {
	fn get_children(&self) -> Vec<Rc<Box<Node>>> { self.childrens.clone() }
	fn get_name(&self) -> String { self.op.value.to_string() }
}


impl Node for BinNode {
	fn add_child(&mut self, new_node: Box<Node>) {
		self.childrens.push(Rc::new(new_node));
	}
}