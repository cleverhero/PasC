use std::fmt;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct Tree {
	pub root: Rc<Box<Node>>
}

impl Tree {
	pub fn new(root: Box<Node>) -> Tree {
		Tree { root: Rc::new(root) }
	}
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "{}", self.root)
    }
}