use std::rc::Rc;
use ParserPack::*;
use support::*;


pub struct ProcedureType {
	arg_list: Vec< Rc< Type > >
}
impl Type for ProcedureType {
	fn bin_operation(&self, other: Rc< Type >, op: BinOperation) -> Result<Rc< Type >, SemanticErrors> { other.bin_operation_procedure_type(self, op) }
	fn cast_to(&self, other: Rc< Type> ) -> Result< Rc< Type >, SemanticErrors > { Err( SemanticErrors::CastError{ this: other.as_str(), other: self.as_str()} ) }
}