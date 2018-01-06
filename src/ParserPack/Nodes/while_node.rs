use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use support::*;

#[derive(Clone)]
pub struct WhileNode {
	pub cond: Rc<Node>,

	pub block: Rc<Node>,
}

impl WhileNode {
	pub fn new(cond: Rc< Node >, block: Rc< Node >) -> Result< WhileNode, SemanticErrors > {
		if !cond.get_type().unwrap().as_enum( "boolean".to_string() ).is_none() {
			Ok( WhileNode { cond, block } )
		}
		else {
			Err( SemanticErrors::OtherError{msg: "Ожидалось логическое выражение".to_string()} )
		}
	}
}


impl Display for WhileNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for WhileNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![self.cond.as_printable(), self.block.as_printable()] }
	fn get_caption(&self) -> String { "While statement".to_string() }
}


impl Node for WhileNode {
	fn get_name(&self) -> String { "".to_string() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}