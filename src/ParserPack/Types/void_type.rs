use std::i32;
use std::rc::Rc;
use ParserPack::*;

pub struct VoidType {}

impl VoidType {
	pub fn new() -> VoidType { VoidType{} }
}

impl Type for VoidType {
	fn get_size(&self) -> i64 { 0 }
	fn as_str(&self) -> String { "Void".to_string() }
	fn value_as_str(&self) -> String { "Void".to_string() }

	fn is_enumerated(&self) -> bool { false }
	fn get_left(&self) -> i32 { 0 }
	fn get_right(&self) -> i32 { 0 }

	fn get_clone(&self) -> Rc< Type > { 
		let e = VoidType::new();
		Rc::new( e ) 
	}
}