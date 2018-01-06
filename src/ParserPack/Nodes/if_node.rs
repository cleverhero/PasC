use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use support::*;

#[derive(Clone)]
pub struct IfNode {
	pub cond: Rc<Node>,

	pub block: Rc< Node >,
	pub else_block: Option< Rc<Node> >,
}

impl IfNode {
	pub fn new(cond: Rc< Node >, block: Rc< Node >, else_block: Option< Rc<Node> >) -> Result< IfNode, SemanticErrors > {
		if !cond.get_type().unwrap().as_enum( "boolean".to_string() ).is_none() {
			Ok( IfNode { cond, block, else_block } )
		}
		else {
			Err( SemanticErrors::OtherError{msg: "Ожидалось логическое выражение".to_string()} )
		}
		
	}
}


impl Display for IfNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for IfNode {
	fn get_children(&self) -> Vec< &PrintableNode > { 
		match self.else_block { 
			Some(ref block) => { vec![self.cond.as_printable(), self.block.as_printable(), block.as_printable()] },
			None => { vec![self.cond.as_printable(), self.block.as_printable()] }
		}
	}
	fn get_caption(&self) -> String { "If statement".to_string() }
}


impl Node for IfNode {
	fn get_name(&self) -> String { "".to_string() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}