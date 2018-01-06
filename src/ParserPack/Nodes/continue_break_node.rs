use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;

#[derive(Clone)]
pub struct ContinueBreakNode {
	pub name: String
}

impl ContinueBreakNode {
	pub fn new(name: String) -> ContinueBreakNode { ContinueBreakNode { name } 	}
}

impl Display for ContinueBreakNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for ContinueBreakNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![] }
	fn get_caption(&self) -> String { self.name.to_string() }
}

impl Node for ContinueBreakNode {
	fn get_name(&self) -> String { self.name.clone() }
	fn get_type(&self) -> Option< Rc< Type > > { None }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}