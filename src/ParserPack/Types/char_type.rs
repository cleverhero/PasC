use ParserPack::*;
use std::rc::Rc;
use std::cell::Cell;
use support::*;

pub struct CharType {
	value: Cell<u8>,
	pub kind: Cell< TypeKind >,
}

impl CharType {
	pub fn new(value: u8) -> CharType {
		CharType{ value: Cell::new(value), kind: Cell::new( TypeKind::Var ), }
	}
} 

impl Type for CharType {
	fn get_size(&self) -> i64 { 8 }
	fn as_str(&self) -> String { "Char".to_string() }
	fn value_as_str(&self) -> String { "#".to_string() + &self.value.get().to_string() }

	fn parse_init_value(&self, parser: &mut Parser) -> Result< String, CompilerErrors > {
		let expr = try!(parser.parse_simple_expr());
		let value = expr.get_type().unwrap();
		try!( self.set_value(value) );
		Ok( self.value.get().to_string() )
	}

	fn as_char(&self) -> Option< u8 > { Some( self.value.get() ) }

	fn get_value(&self) -> ValueVariant { ValueVariant::Char{ v: self.value.get() } }
	fn set_value(&self, value: Rc< Type >) -> Result< String, SemanticErrors > {
		if !is_mutable_kind(&self.kind.get()) {
			return Err( self.create_err(format!("Невозможно привести {} к {}", value.as_str(), self.as_str())) )
		}

		let new_value = match value.as_char() {
			Some(res) => res,
			None => return Err( self.create_err(format!("Невозможно привести {} к {}", value.as_str(), self.as_str())) )
		};;
		self.value.set(new_value);
		Ok( "Ok".to_string() )
	}

	fn is_enumerated(&self) -> bool { true }
	fn get_left(&self) -> i32 { 0 }
	fn get_right(&self) -> i32 { 127 }
	fn get_clone(&self) -> Rc< Type > { Rc::new( CharType::new(self.value.get()) ) }

	fn set_kind(&self, kind: TypeKind) { self.kind.set(kind); }

	fn bin_operation(&self, other: Rc< Type >, op: BinOperation) -> Result<Rc< Type >, SemanticErrors> { other.bin_operation_char_type(self, op) }

	fn cast_to(&self, other: Rc< Type> ) -> Result< Rc< Type >, SemanticErrors > { other.cast_from_char(self) }
	fn cast_from_char(&self, other: &CharType ) -> Result< Rc< Type >, SemanticErrors > { 
		if !kind_cast(&other.kind.get(), &self.kind.get()) {
			return Err( self.create_err(format!("Невозможно привести {} к {}", other.as_str(), self.as_str())) ) 
		}
		Ok( Rc::new(CharType::new(other.value.get())) ) 
	}
}