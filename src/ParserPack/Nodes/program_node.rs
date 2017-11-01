use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct ProgramNode {
	pub name: String,
	pub childrens: Vec<Rc<Box<Node>>>,
}

impl ProgramNode {
	pub fn new(name: String) -> ProgramNode {
		ProgramNode {
			name: name,
			childrens: vec![]
		}
	}
}

impl Display for ProgramNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl Printable for ProgramNode {
	fn get_children(&self) -> Vec<Rc<Box<Node>>> { self.childrens.clone() }
	fn get_name(&self) -> String { self.name.to_string() }
}
impl Node for ProgramNode {
	fn add_child(&mut self, new_node: Box<Node>) {
		self.childrens.push(Rc::new(new_node));
	}
}