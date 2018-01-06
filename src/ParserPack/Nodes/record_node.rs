use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;
use std::collections::HashMap;


#[derive(Clone)]
pub struct RecordNode {
	pub name: String,
	pub self_type: Rc<Type>,

	pub fields: Vec< Rc<Node> >
}

impl RecordNode {
	pub fn new(name: String, fields: Vec< Rc<Node> >) -> RecordNode {
		let mut res = RecordNode{ name, self_type: Rc::new( IntegerType::new(0) ), fields };
		res.culc_type();

		res
	}

	fn culc_type(&mut self) {
		let mut field_map: HashMap< String, Rc< Type > > = HashMap::new();
		let mut field_list: Vec< String > = vec![];
		for field in &self.fields {
			field_map.insert(field.get_name(), field.get_type().unwrap());
			field_list.push(field.get_name());
		}
		self.self_type = Rc::new( RecordType::new(self.name.clone(), field_map, field_list) );
	}
}

impl Display for RecordNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}

impl PrintableNode for RecordNode {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![] }
	fn get_caption(&self) -> String { self.name.to_string() + "(" + &self.self_type.as_str() + ")" }
}

impl Node for RecordNode {
	fn get_type(&self) -> Option< Rc<Type> > { Some( self.self_type.clone() ) }
	fn get_name(&self) -> String { self.name.clone() }
	fn as_printable(&self) -> &PrintableNode { self }
}