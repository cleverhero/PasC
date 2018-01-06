use std::fmt;
use TokenizerPack::token::Token;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;
use TokenizerPack::*;
use support::*;


#[derive(Clone)]
pub struct UnaryOpNode {
	pub op: Token,
	pub children: Rc< Node >,
	pub self_type: Rc< Type >
}

impl UnaryOpNode {
	pub fn new(op: Token, children: Rc< Node >) -> Result< UnaryOpNode, SemanticErrors > {
		let op_type = match op.token_type {
			TokenType::TPlus => UnarOperation::Plus,
			TokenType::TMinus => UnarOperation::Minus,
			TokenType::TNot => UnarOperation::Not,
			_ => { return Err( SemanticErrors::OtherError{msg: "Ожидалось + - not".to_string()} )}
		};

		let self_type = try!( children.get_type().unwrap().unar_operation(op_type) );
		Ok( UnaryOpNode{ op, children, self_type: self_type.clone() } )
	}
}

impl Display for UnaryOpNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for UnaryOpNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![ self.children.as_printable() ] }
	fn get_caption(&self) -> String { self.op.text.to_string() + " : " + &self.self_type.as_str() + " = " + &self.self_type.value_as_str()}
}


impl Node for UnaryOpNode {
	fn get_type(&self) -> Option< Rc< Type > > { Some( self.self_type.clone() ) 	}
	fn get_name(&self) -> String { self.op.value.to_string() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}
