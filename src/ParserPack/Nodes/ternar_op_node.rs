use std::fmt;
use TokenizerPack::token::Token;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct TernarOpNode {
	pub op: Token,
	pub children: Rc<Box<Node>>,
}

impl TernarOpNode {
	pub fn new(op: Token, children: Box<Node>) -> TernarOpNode {
		TernarOpNode {
			op: op, 
			children: Rc::new(children)
		}
	}
}

impl Display for TernarOpNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl Printable for TernarOpNode {
	fn get_children(&self) -> Vec<Rc<Box<Node>>> { vec![self.children.clone()] }
	fn get_name(&self) -> String { self.op.value.to_string() }
}
impl Node for TernarOpNode {}