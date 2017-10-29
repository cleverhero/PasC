use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct IdNode {
	pub value: String
}

impl IdNode {
	pub fn new(value: String) -> IdNode {
		IdNode {
			value: value
		}
	}
}

impl Display for IdNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl Printable for IdNode {
	fn get_children(&self) -> Vec<Rc<Box<Node>>> { vec![] }
	fn get_name(&self) -> String { self.value.to_string() }
}
impl Node for IdNode {}