use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::*;

#[derive(Clone)]
pub struct IdNode {
	pub child: Rc< Node >
}

impl IdNode {
	pub fn new(child: Rc< Node >) -> IdNode {
		IdNode { child }
	}
}

impl Display for IdNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for IdNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![] }
	fn get_caption(&self) -> String { self.child.get_caption().to_string() }
}

impl Node for IdNode {
	fn get_type(&self) -> Option< Rc< Type > > { self.child.get_type().clone() }
	fn get_name(&self) -> String { self.child.get_name().clone() }
	fn get_kind(&self) -> KindIdentifier { self.child.get_kind() }
	fn as_printable(&self) -> &PrintableNode { self }
}