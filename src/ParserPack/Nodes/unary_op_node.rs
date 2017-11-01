use std::fmt;
use TokenizerPack::token::Token;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct UnaryOpNode {
	pub op: Token,
	pub children: Rc<Box<Node>>,
}

impl UnaryOpNode {
	pub fn new(op: Token, children: Box<Node>) -> UnaryOpNode {
		UnaryOpNode {
			op: op, 
			children: Rc::new(children)
		}
	}
}

impl Display for UnaryOpNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl Printable for UnaryOpNode {
	fn get_children(&self) -> Vec<Rc<Box<Node>>> { vec![self.children.clone()] }
	fn get_name(&self) -> String { self.op.value.to_string() }
}
impl Node for UnaryOpNode {}