use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
	

pub struct ConstNode<T> {
	pub value: T
}

impl<T> ConstNode<T> {
	pub fn new(value: T) -> ConstNode<T> {
		ConstNode {
			value: value
		}
	}
}

impl<T: fmt::Display> Display for ConstNode<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl<T: fmt::Display> Printable for ConstNode<T> {
	fn get_children(&self) -> Vec<Rc<Box<Node>>> { vec![] }
	fn get_name(&self) -> String { self.value.to_string() }
}
impl<T: fmt::Display> Node for ConstNode<T> {}