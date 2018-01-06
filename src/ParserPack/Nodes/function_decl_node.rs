use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;

#[derive(Clone)]
pub struct DeclFunctionNode {
	pub name: String,
	pub self_type: Rc< FunctionType >,
	pub block: Option< Rc< Node > >,
}

impl DeclFunctionNode {
	pub fn new(name: String, arg_list: Vec< Rc< Node > >, out_type: Rc< Type >, block: Option< Rc< Node > >) -> DeclFunctionNode {
		let arg_list_type = arg_list.clone().into_iter().map(|arg| arg.get_type().unwrap()).collect();

		let self_type = Rc::new( FunctionType::new(arg_list_type, out_type));
		DeclFunctionNode { name, self_type, block }
	}
}

impl Display for DeclFunctionNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for DeclFunctionNode {
	fn get_children(&self) -> Vec< &PrintableNode > { 
		let mut ans: Vec< &PrintableNode > = vec![];
		match self.block { Some(ref res) => ans.push(res.as_printable()), None => {} }
		ans
	}
	fn get_caption(&self) -> String { self.name.to_string() + &self.self_type.as_str() }
}

impl Node for DeclFunctionNode {
	fn get_type(&self) -> Option< Rc< Type > > { Some( self.self_type.clone() ) }
	fn get_name(&self) -> String { self.name.clone() }
	fn get_kind(&self) -> KindIdentifier { 
		match self.block {
			Some( ref _node ) => KindIdentifier::Function,
			None => KindIdentifier::ForwardFunction
		}
    }
	fn as_printable(&self) -> &PrintableNode { self }
}
