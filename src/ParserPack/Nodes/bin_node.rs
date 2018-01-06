use std::fmt;
use TokenizerPack::token::Token;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;
use TokenizerPack::*;
use support::*;

#[derive(Clone)]
pub struct BinNode {
	pub left: Rc< Node >,
	pub right: Rc< Node >,

	pub self_type: Rc< Type >,
	pub op: Token,
}

impl BinNode {
	pub fn new(op: Token, left: Rc< Node >, right: Rc< Node >) -> Result< BinNode, SemanticErrors > {
		let op_type = match op.token_type {
			TokenType::TPlus  => BinOperation::Plus,
			TokenType::TMinus => BinOperation::Minus,
			TokenType::TMul   => BinOperation::Mul,
			TokenType::TShare => BinOperation::Share,
			TokenType::TAnd   => BinOperation::And,
			TokenType::TOr    => BinOperation::Or,
			TokenType::TGe    => BinOperation::OGe,
			TokenType::TGt    => BinOperation::OGt,
			TokenType::TEq    => BinOperation::OEq,
			TokenType::TLe    => BinOperation::OLe,
			TokenType::TLt    => BinOperation::OLt,
			TokenType::TNe    => BinOperation::ONe,
			_ => { return Err( SemanticErrors::OtherError{msg: "Ожидалось * / + - or and или операторы сравнения".to_string()} )}
		};

		let self_type = try!( left.get_type().unwrap().bin_operation(right.get_type().unwrap(), op_type) );
		Ok( BinNode { op, left, right, self_type } )
	}
}

impl Display for BinNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for BinNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![self.left.as_printable(), self.right.as_printable()]}
	fn get_caption(&self) -> String { self.op.text.to_string() + " : " + &self.self_type.as_str() + " = " + &self.self_type.value_as_str()}
}


impl Node for BinNode {
	fn get_type(&self) -> Option< Rc< Type > > { Some( self.self_type.clone() ) }
	fn get_name(&self) -> String { self.op.text.to_string() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }
	fn as_printable(&self) -> &PrintableNode { self }
}