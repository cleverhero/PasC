use std::i32;
use ParserPack::*;
use std::rc::Rc;
use std::cell::Cell;
use support::*;
use std::collections::HashMap;

pub struct EnumType {
	variants_list: Vec< String >,
	value_map: HashMap < String, i64 >,

	is_initialized: bool,
	name: String,
	value: Cell<i64>,

	pub kind: Cell< TypeKind >,
}


impl EnumType {
	pub fn new(variants_list: Vec< String >, value_map_init: HashMap < String, i64 >, name: String, value: i64) -> EnumType {
		let mut value_map: HashMap< String, i64 > = HashMap::new();
		let mut curr_val = 0;
		for name in &variants_list {
			match value_map_init.get(name) {
				Some(res) => {
					value_map.insert(name.clone(), *res);
					curr_val = *res + 1;
				},
				None => {
					value_map.insert(name.clone(), curr_val);
					curr_val += 1;
				}
			}
		}
		let is_initialized = if !value_map_init.is_empty() { true } else { false };

		EnumType{ variants_list, value_map, name, value: Cell::new(value), is_initialized, kind: Cell::new( TypeKind::Var )}
	}
}
impl Type for EnumType {
	fn get_size(&self) -> i64 { 32 }
	fn as_str(&self) -> String {	
		let mut caption = self.name.clone() + ":(";
		for name in &self.variants_list {
        	caption += name;
        	caption += " = "; 
        	caption += &match self.value_map.get(name) {
        		None => "".to_string(),
        		Some(res) => res.to_string()
        	}; 
        	caption += ", "
		} 
		caption[..caption.len() - 2].to_string() + ")"
	}

	fn value_as_str(&self) -> String { 
		self.variants_list[ self.value.get() as usize ].clone()
    }

	fn parse_init_value(&self, parser: &mut Parser) -> Result< String, CompilerErrors > {
		let expr = try!(parser.parse_simple_expr());
		let value = expr.get_type().unwrap();
		try!( self.set_value(value) );
		Ok( self.value.get().to_string() )
	}
	fn get_clone(&self) -> Rc< Type > { Rc::new( EnumType::new(self.variants_list.clone(), self.value_map.clone(), self.name.clone(), self.value.get()) ) }

	fn is_enumerated(&self) -> bool { !self.is_initialized }
	fn get_left(&self) -> i32 { 0 }
	fn get_right(&self) -> i32 { self.variants_list.len() as i32 - 1}


	fn get_value(&self) -> ValueVariant { ValueVariant::Enum{ name: self.name.clone(), v: self.value.get() } }
	fn set_value(&self, value: Rc< Type >) -> Result< String, SemanticErrors > {
		let new_value = match value.as_enum(self.name.clone()) {
			Some(res) => res,
			None => return Err( self.create_err(format!("Невозможно привести {} к {}", value.as_str(), self.as_str())) )
		};;
		self.value.set(new_value);
		Ok( "Ok".to_string() )
	}
	fn set_kind(&self, kind: TypeKind) { self.kind.set(kind); }

	fn as_enum(&self, name: String) -> Option< i64 > { 
		if name == self.name { return Some( self.value.get() ) }
		None
	}

	fn as_enum_without_name(&self) -> Option< i64 > { Some( self.value.get() ) }
	fn bin_operation(&self, other: Rc< Type >, op: BinOperation) -> Result<Rc< Type >, SemanticErrors> { other.bin_operation_enum_type(self, op) }

	fn bin_operation_enum_type(&self, other: &EnumType, op: BinOperation) -> Result< Rc< Type >, SemanticErrors > {
		let vars: Vec<bool> = vec![false, true];
		match op {
            BinOperation::And => { Ok( create_boolean( vars[ (other.value.get() & self.value.get()) as usize ] ) ) }, 
            BinOperation::Or  => { Ok( create_boolean( vars[ (other.value.get() | self.value.get()) as usize ] ) ) },
            BinOperation::OGe => { Ok( create_boolean( other.value.get() >= self.value.get() ) ) }, 
            BinOperation::OGt => { Ok( create_boolean( other.value.get() >  self.value.get() ) ) }, 
            BinOperation::OEq => { Ok( create_boolean( other.value.get() == self.value.get() ) ) },
            BinOperation::OLe => { Ok( create_boolean( other.value.get() <= self.value.get() ) ) }, 
            BinOperation::OLt => { Ok( create_boolean( other.value.get() <  self.value.get() ) ) }, 
            BinOperation::ONe => { Ok( create_boolean( other.value.get() != self.value.get() ) ) },
            _ => { Err( SemanticErrors::ErrorInBinOperation{ left: other.as_str(), right: self.as_str(), op} ) }
        }
	}

	fn cast_to(&self, other: Rc< Type> ) -> Result< Rc< Type >, SemanticErrors > { other.cast_from_enum(self) }
	fn cast_from_enum(&self, other: &EnumType ) -> Result< Rc< Type >, SemanticErrors > { 
		if !kind_cast(&other.kind.get(), &self.kind.get()) {
			return Err( self.create_err(format!("Невозможно привести {} к {}", other.as_str(), self.as_str())) ) 
		}
		
		if other.name != self.name { return Err( SemanticErrors::CastError{ this: other.as_str(), other: self.as_str()} ) }
		Ok( other.get_clone() ) 
	}
}