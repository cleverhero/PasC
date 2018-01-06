use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;

#[derive(Clone)]
pub struct DeclVarNode {
	pub name: String,
	pub self_type: Rc< Type >
}

impl DeclVarNode {
	pub fn new(name: String, self_type: Rc< Type >) -> DeclVarNode {
		DeclVarNode { name, self_type }
	}
}

impl Display for DeclVarNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for DeclVarNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![] }
	fn get_caption(&self) -> String { self.name.to_string() + " : " + &self.self_type.as_str() + " = " + &self.self_type.value_as_str() }
}

impl Node for DeclVarNode {
	fn get_type(&self) -> Option< Rc<Type> > { Some( self.self_type.clone() ) 	}
	fn get_name(&self) -> String { self.name.clone() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Var }

	fn as_printable(&self) -> &PrintableNode { self }
}
