use std::rc::Rc;
use support::*;
use ParserPack::*;


pub struct FunctionType {
	pub arg_list: Vec< Rc< Type > >,
	pub out_type: Rc< Type >,
}

impl FunctionType {
	pub fn new(arg_list: Vec< Rc< Type > >, out_type: Rc< Type >) -> FunctionType {
		out_type.set_kind( TypeKind::RValue );

		FunctionType{ arg_list, out_type }
	}
}


impl Type for FunctionType {
	fn get_size(&self) -> i64 { 0 }
	fn as_str(&self) -> String {	
		let mut ans = "(".to_string();
		for arg in &self.arg_list {
			ans += &arg.as_str();
			ans += ", ";
		}

		ans = if ans.len() < 2 { "(Void): ".to_string() }
			else { ans[0..ans.len() - 2].to_string() + "): " };
			
		ans += &self.out_type.as_str();
		ans
	}

	fn get_clone(&self) -> Rc< Type > { Rc::new( FunctionType{ arg_list: self.arg_list.clone(), out_type: self.out_type.clone() } ) }
	fn value_as_str(&self) -> String { 
		let mut ans = "(".to_string();
		for arg in &self.arg_list {
			ans += &arg.value_as_str();
			ans += ", ";
		}

		ans = if self.arg_list.len() == 0 { "(void)".to_string() }
			else { ans[0..ans.len() - 2].to_string() + ")" };
		ans
	}
	fn bin_operation(&self, other: Rc< Type >, op: BinOperation) -> Result< Rc< Type >, SemanticErrors > { other.bin_operation_function_type(self, op) }

	fn cast_to(&self, other: Rc< Type> ) -> Result< Rc< Type >, SemanticErrors > { other.cast_from_function(self) }
	fn cast_from_function(&self, other: &FunctionType ) -> Result< Rc< Type >, SemanticErrors > {
		if self.arg_list.len() != other.arg_list.len() { return Err( SemanticErrors::CastError{ this: other.as_str(), other: self.as_str()} ) }

		let mut args: Vec< Rc<Type> > = vec![];
		for i in 0..self.arg_list.len() { 
			args.push( try!(other.arg_list[i].cast_to(self.arg_list[i].clone())) );
		}

		Ok( Rc::new(FunctionType::new(args, self.out_type.get_clone() )) )
	}

	fn call_by_args(&self) -> Result< Rc< Type >, SemanticErrors > {
		Ok( self.out_type.clone() )
	}
}