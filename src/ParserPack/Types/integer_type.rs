use std::i32;
use std::cell::Cell;
use support::*;
use ParserPack::*;
use std::rc::Rc;

pub struct IntegerType {
	pub value: Cell<i64>,
	pub kind: Cell< TypeKind >
}

impl IntegerType {
	pub fn new(value: i64) -> IntegerType {
		IntegerType{value: Cell::new(value), kind: Cell::new( TypeKind::Var ) }
	}
}

impl Type for IntegerType {
	fn get_size(&self) -> i64 { 32 }
	fn as_str(&self) -> String { "Integer".to_string() }
	fn value_as_str(&self) -> String { self.value.get().to_string() }
	fn parse_init_value(&self, parser: &mut Parser) -> Result< String, CompilerErrors > {
		let expr = try!(parser.parse_simple_expr());
		let value = expr.get_type().unwrap();
		try!( self.set_value(value) );
		Ok( self.value.get().to_string() )
	}

	fn get_value(&self) -> ValueVariant { ValueVariant::Int{ v: self.value.get() } }
	fn as_integer(&self) -> Option< i64 > { Some( self.value.get() ) }
	fn as_double(&self) -> Option< f64 > { Some( self.value.get() as f64 ) }
	fn set_value(&self, value: Rc< Type >) -> Result< String, SemanticErrors > {
		if !is_mutable_kind(&self.kind.get()) {
			return Err( self.create_err(format!("Невозможно привести {} к {}", value.as_str(), self.as_str())) )
		}

		let new_value = match value.as_integer() {
			Some(res) => res,
			None => return Err( self.create_err(format!("Невозможно привести {} к {}", value.as_str(), self.as_str())) )
		};;
		self.value.set(new_value);
		Ok( "Ok".to_string() )
	}
	fn set_kind(&self, kind: TypeKind) { self.kind.set(kind); }

	fn is_enumerated(&self) -> bool { true }
	fn get_left(&self) -> i32 { i32::MIN }
	fn get_right(&self) -> i32 { i32::MAX }
	fn get_clone(&self) -> Rc< Type > { Rc::new( IntegerType::new(self.value.get()) ) }


	fn unar_operation(&self, op: UnarOperation) -> Result<Rc< Type >, SemanticErrors> {
		match op {
			UnarOperation::Plus  => Ok( Rc::new( IntegerType::new( self.value.get())) as Rc<Type>  ), 
			UnarOperation::Minus => Ok( Rc::new( IntegerType::new(-self.value.get())) as Rc<Type>  ), 
			UnarOperation::Not   => Ok( Rc::new( IntegerType::new(-self.value.get())) as Rc<Type>  )
		}
	}

	fn bin_operation(&self, other: Rc< Type >, op: BinOperation) -> Result<Rc< Type >, SemanticErrors> { other.bin_operation_integer_type(self, op) }

	fn bin_operation_integer_type(&self, other: &IntegerType, op: BinOperation) -> Result< Rc< Type >, SemanticErrors > {
		match op {
            BinOperation::Plus  => { Ok( Rc::new( IntegerType::new( self.value.get() + other.value.get() )) as Rc<Type> ) }, 
            BinOperation::Minus => { Ok( Rc::new( IntegerType::new( other.value.get() - self.value.get() )) as Rc<Type> ) }, 
            BinOperation::Mul   => { Ok( Rc::new( IntegerType::new( self.value.get() * other.value.get() )) as Rc<Type> ) },
            BinOperation::Share => { Ok( Rc::new( IntegerType::new( other.value.get() / self.value.get() )) as Rc<Type> ) }, 
            BinOperation::And   => { Ok( Rc::new( IntegerType::new( self.value.get() * other.value.get() )) as Rc<Type> ) }, 
            BinOperation::Or    => { Ok( Rc::new( IntegerType::new( other.value.get() / self.value.get() )) as Rc<Type> ) },
            BinOperation::OGe   => { Ok( create_boolean( other.value.get() >= self.value.get() ) ) }, 
            BinOperation::OGt   => { Ok( create_boolean( other.value.get() >  self.value.get() ) ) }, 
            BinOperation::OEq   => { Ok( create_boolean( other.value.get() == self.value.get() ) ) },
            BinOperation::OLe   => { Ok( create_boolean( other.value.get() <= self.value.get() ) ) }, 
            BinOperation::OLt   => { Ok( create_boolean( other.value.get() <  self.value.get() ) ) }, 
            BinOperation::ONe   => { Ok( create_boolean( other.value.get() != self.value.get() ) ) },
        }
		
	}
	fn bin_operation_double_type(&self, other: &DoubleType, op: BinOperation) -> Result<Rc<Type> , SemanticErrors > {
		match op {
            BinOperation::Plus  => { Ok( Rc::new( DoubleType::new( self.value.get() as f64 + other.value.get() )) as Rc<Type>  ) }, 
            BinOperation::Minus => { Ok( Rc::new( DoubleType::new( other.value.get() - self.value.get() as f64 )) as Rc<Type>  ) }, 
            BinOperation::Mul   => { Ok( Rc::new( DoubleType::new( self.value.get() as f64 * other.value.get() )) as Rc<Type>  ) },
            BinOperation::Share => { Ok( Rc::new( DoubleType::new( other.value.get() / self.value.get() as f64 )) as Rc<Type>  ) }, 
            BinOperation::And   => { Ok( Rc::new( DoubleType::new( self.value.get() as f64 * other.value.get() )) as Rc<Type>  ) }, 
            BinOperation::Or    => { Ok( Rc::new( DoubleType::new( other.value.get() / self.value.get() as f64 )) as Rc<Type>  ) },
            BinOperation::OGe   => { Ok( create_boolean( other.value.get() >= self.value.get() as f64 ) ) }, 
            BinOperation::OGt   => { Ok( create_boolean( other.value.get() >  self.value.get() as f64 ) ) }, 
            BinOperation::OEq   => { Ok( create_boolean( other.value.get() == self.value.get() as f64 ) ) },
            BinOperation::OLe   => { Ok( create_boolean( other.value.get() <= self.value.get() as f64 ) ) }, 
            BinOperation::OLt   => { Ok( create_boolean( other.value.get() <  self.value.get() as f64 ) ) }, 
            BinOperation::ONe   => { Ok( create_boolean( other.value.get() != self.value.get() as f64 ) ) },
        }
	}

	fn cast_to(&self, other: Rc< Type> ) -> Result< Rc< Type >, SemanticErrors > { other.cast_from_integer(self) }

	fn cast_from_integer(&self, other: &IntegerType ) -> Result< Rc< Type >, SemanticErrors > { 
		if !kind_cast(&other.kind.get(), &self.kind.get()) {
			return Err( self.create_err(format!("Невозможно привести {} к {}", other.as_str(), self.as_str())) ) 
		}
		Ok( Rc::new(IntegerType::new(other.value.get())) ) 
	}
}