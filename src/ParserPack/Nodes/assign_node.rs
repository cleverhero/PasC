use std::fmt;
use TokenizerPack::token::Token;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::Types::support::*;
use TokenizerPack::*;
use support::*;

#[derive(Clone)]
pub struct AssignNode {
	pub target: Rc< Node >,
	pub object: Rc< Node >,

	pub op: Token,
}

impl AssignNode {
	pub fn new(op: Token, target: Rc< Node >, object: Rc< Node >) -> Result< AssignNode, SemanticErrors > {
		match op.token_type {
			TokenType::TAssign      =>  {
				try!( target.get_type().unwrap().set_value(object.get_type().unwrap()) );
			}
			TokenType::TPlsAssign   => {
				let new_object = try!( target.get_type().unwrap().bin_operation(object.get_type().unwrap(), BinOperation::Plus) );
				try!( target.get_type().unwrap().set_value(new_object) );
			}
			TokenType::TMinAssign   => {
				let new_object = try!( target.get_type().unwrap().bin_operation(object.get_type().unwrap(), BinOperation::Minus) );
				try!( target.get_type().unwrap().set_value(new_object) );
			}
			TokenType::TMulAssign   => {
				let new_object = try!( target.get_type().unwrap().bin_operation(object.get_type().unwrap(), BinOperation::Mul) );
				try!( target.get_type().unwrap().set_value(new_object) );
			}
			TokenType::TShareAssign => {
				let new_object = try!( target.get_type().unwrap().bin_operation(object.get_type().unwrap(), BinOperation::Share) );
				try!( target.get_type().unwrap().set_value(new_object) );
			}
			_ => { return Err( SemanticErrors::OtherError{msg: "Ожидалось += -= := *= /=".to_string()} )}
		};

		Ok( AssignNode { op, target, object } )
	}
}


impl Display for AssignNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for AssignNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![self.target.as_printable(), self.object.as_printable()]}
	fn get_caption(&self) -> String { self.op.text.to_string() }
}


impl Node for AssignNode {
	fn get_name(&self) -> String { self.op.text.to_string() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}