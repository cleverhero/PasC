use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;
use support::*;

#[derive(Clone)]
pub struct RecordFieldNode {
	pub field_name: String,
	pub parent: Rc< Node >,
	pub self_type: Rc< Type >
}

impl RecordFieldNode {
	pub fn new(parent: Rc< Node >, field_name: String) -> Result< RecordFieldNode, SemanticErrors > {
		let self_type = try!( parent.get_type().unwrap().get_by_field(field_name.clone()) );
		Ok( RecordFieldNode { parent, field_name, self_type } )
	}
}

impl Display for RecordFieldNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for RecordFieldNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![self.parent.as_printable()] }
	fn get_caption(&self) -> String { self.get_name() + " : " + &self.self_type.as_str() + " = " + &self.self_type.value_as_str() }
}

impl Node for RecordFieldNode {
	fn get_type(&self) -> Option< Rc< Type > > { Some( self.self_type.clone() ) 	}
	fn get_name(&self) -> String { self.parent.get_name() + "." + &self.field_name }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}
