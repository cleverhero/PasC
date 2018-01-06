use std::rc::Rc;
use ParserPack::*;
use std::collections::HashMap;
use support::*;
use TokenizerPack::support::*;
use std::cell::Cell;

pub struct RecordType {
	name: String,

	fields: HashMap< String, Rc< Type > >,
	field_order: Vec< String >,

	pub kind: Cell< TypeKind >,
}

impl RecordType {
	pub fn new(name: String, fields: HashMap< String, Rc< Type > >, field_order: Vec< String >) -> RecordType {
		RecordType{ name, fields: fields.clone(), field_order, kind: Cell::new( TypeKind::Var ) }
	}
}

impl Type for RecordType {
	fn get_size(&self) -> i64 { 
		let mut size: i64 = 0;
		for name in &self.field_order { size += self.get_by_field(*name.clone()).unwrap().get_size(); } 
		size
	}
	fn as_str(&self) -> String {	
		let mut ans = self.name.clone() + ": ";
		for name in &self.field_order { ans += &(self.get_by_field(name.to_string()).unwrap().as_str() + ", "); } 

		ans[..ans.len() - 2].to_string()
	}

	fn value_as_str(&self) -> String {	
		let mut ans = "(".to_string();
		for name in &self.field_order { ans += &(name.clone() + ": " + &self.get_by_field(name.to_string()).unwrap().value_as_str() + ", "); } 
		ans[..ans.len() - 2].to_string() + ")"
	}

	fn get_by_field(&self, field_name: String) -> Result< Rc< Type >, SemanticErrors > {
		match self.fields.get(&field_name) {
			Some(res) => return Ok( res.clone() ),
			None => Err( self.create_err(format!("Полe {} у {} не существует", field_name, self.as_str())) ) 
		}
	}

	fn get_clone(&self) -> Rc< Type > { 
		let mut field_map = HashMap::new();
		for (name, self_type) in &self.fields { field_map.insert(name.clone(), self_type.get_clone()); }
		Rc::new( RecordType{ name: self.name.clone(), fields: field_map, field_order: self.field_order.clone(), kind: self.kind.clone() } ) 
	}

	fn set_kind(&self, kind: TypeKind) { 
		for field in &self.field_order { self.get_by_field( field.to_string() ).unwrap().set_kind( kind.clone() ); }
		self.kind.set(kind); 
	}

	fn parse_init_value(&self, parser: &mut Parser) -> Result< String, CompilerErrors > {
		check_token!(parser, TokenType::TOp);
		for i in 0..self.field_order.len() {
			let name = self.field_order[i].clone();
			let t = try!( parser.tokenizer.get_and_next() );
			if t.text != name { return Err( self.err_in_parse(&t) ) } 
			check_token!(parser, TokenType::TColon);
			try!( self.get_by_field(name.to_string()).unwrap().parse_init_value(parser) );
			if i < self.field_order.len() - 1 { check_token!(parser, TokenType::TComma); } 
		}
		check_token!(parser, TokenType::TCp);
		Ok( "Record".to_string() )
	}

	fn bin_operation(&self, other: Rc< Type >, op: BinOperation) -> Result<Rc< Type >, SemanticErrors> { other.bin_operation_record_type(self, op) } 

	fn cast_to(&self, other: Rc< Type> ) -> Result< Rc< Type >, SemanticErrors > { other.cast_from_record(self) }

	fn cast_from_record(&self, other: &RecordType ) -> Result< Rc< Type >, SemanticErrors > { 
		if !kind_cast(&other.kind.get(), &self.kind.get()) {
			return Err( self.create_err(format!("Невозможно привести {} к {}", other.as_str(), self.as_str())) ) 
		}
		
		if other.name != self.name { return Err( SemanticErrors::CastError{ this: other.as_str(), other: self.as_str()} ) }
		Ok( other.get_clone() ) 
	}
}