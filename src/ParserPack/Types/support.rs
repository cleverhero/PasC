use std::i32;
use std::collections::HashMap;
use std::rc::Rc;
use ParserPack::*;
use support::*;
use TokenizerPack::token::*;

pub const MAX_SIZE: i64 = 4294967296; // 2^32

pub enum ValueVariant {
    Int { v: i64 },
    Double { v: f64 },
    Char { v: u8 },
    Enum { name: String, v: i64 },
    Other,
}

#[derive(Debug)]
pub enum UnarOperation {
    Plus,
    Minus,
    Not,
}

#[derive(Debug)]
pub enum BinOperation {
    Plus,
    Minus,
    Mul,
    Share,
    And,
    Or,
    OGe,
    OGt,
    OEq,
    OLe,
    OLt,
    ONe,
}

#[derive(Debug, Clone, Copy)]
pub enum TypeKind {
    Var,
    Const,
    RValue,
    VarArg,
    ConstArg,
    Arg,
}

pub fn create_boolean(res: bool) -> Rc<Type> {
    let var_list = vec!["false".to_string(), "true".to_string()];
    let value_map: HashMap<String, i64> = HashMap::new();
    match res {
        false => Rc::new(EnumType::new(
            var_list.clone(),
            value_map.clone(),
            "boolean".to_string(),
            0,
        )) as Rc<Type>,
        true => Rc::new(EnumType::new(
            var_list.clone(),
            value_map.clone(),
            "boolean".to_string(),
            1,
        )) as Rc<Type>,
    }
}

pub fn kind_cast(kind1: &TypeKind, kind2: &TypeKind) -> bool {
    match *kind2 {
        TypeKind::VarArg => match *kind1 {
            TypeKind::Var => true,
            TypeKind::Const => false,
            TypeKind::RValue => false,
            TypeKind::VarArg => true,
            TypeKind::ConstArg => false,
            TypeKind::Arg => true,
        },
        _ => true,
    }
}

pub fn is_mutable_kind(kind: &TypeKind) -> bool {
    match *kind {
        TypeKind::Var => true,
        TypeKind::Const => false,
        TypeKind::RValue => false,
        TypeKind::VarArg => true,
        TypeKind::ConstArg => false,
        TypeKind::Arg => true,
    }
}

pub trait Type {
    fn get_size(&self) -> i64 {
        0
    }

    fn set_unknown(&self, _is_unknown: bool) {}
    fn get_unknown(&self) -> bool {
        true
    }

    fn as_str(&self) -> String {
        "".to_string()
    }
    fn value_as_str(&self) -> String {
        "".to_string()
    }

    fn err_in_parse(&self, t: &Token) -> CompilerErrors {
        let x = t.coords.clone().x;
        let y = t.coords.clone().y;
        let token = "идентификатор".to_string();
        CompilerErrors::ParserError {
            err: ParserErrors::ExpectedToken { x, y, token },
        }
    }
    fn parse_init_value(&self, parser: &mut Parser) -> Result<String, CompilerErrors> {
        let t = parser.tokenizer.current.clone();
        Err(self.err_in_parse(&t))
    }

    fn get_by_index(&self, _index: Rc<Type>) -> Result<Rc<Type>, SemanticErrors> {
        Err(self.create_err(format!(
            "Нельзя обратиться по индексу к {}",
            self.as_str()
        )))
    }
    fn get_by_field(&self, _field_name: String) -> Result<Rc<Type>, SemanticErrors> {
        Err(self.create_err(format!(
            "Нельзя обратиться к полям {}",
            self.as_str()
        )))
    }
    fn call_by_args(&self) -> Result<Rc<Type>, SemanticErrors> {
        Err(self.create_err(format!(
            "Нельзя вызвать {} как функцию",
            self.as_str()
        )))
    }

    fn is_enumerated(&self) -> bool {
        false
    }
    fn get_left(&self) -> i32 {
        0
    }
    fn get_right(&self) -> i32 {
        0
    }

    fn get_clone(&self) -> Rc<Type> {
        Rc::new(IntegerType::new(0))
    }
    fn create_err(&self, msg: String) -> SemanticErrors {
        SemanticErrors::OtherError { msg }
    }

    fn set_kind(&self, _kind: TypeKind) {}
    fn get_value(&self) -> ValueVariant {
        ValueVariant::Other
    }
    fn set_value(&self, value: Rc<Type>) -> Result<String, SemanticErrors> {
        Err(self.create_err(format!(
            "Невозможно привести {} к {}",
            value.as_str(),
            self.as_str()
        )))
    }

    fn as_integer(&self) -> Option<i64> {
        None
    }
    fn as_char(&self) -> Option<u8> {
        None
    }
    fn as_double(&self) -> Option<f64> {
        None
    }
    fn as_enum(&self, _name: String) -> Option<i64> {
        None
    }
    fn as_enum_without_name(&self) -> Option<i64> {
        None
    }

    fn unar_operation(&self, op: UnarOperation) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInUnarOperation {
            name: self.as_str(),
            op,
        })
    }
    fn bin_operation(
        &self,
        other: Rc<Type>,
        _op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }

    fn bin_operation_integer_type(
        &self,
        other: &IntegerType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    fn bin_operation_char_type(
        &self,
        other: &CharType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    fn bin_operation_double_type(
        &self,
        other: &DoubleType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    fn bin_operation_range_type(
        &self,
        other: &RangeType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    fn bin_operation_enum_type(
        &self,
        other: &EnumType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    fn bin_operation_record_type(
        &self,
        other: &RecordType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    //fn bin_operation_procedure_type(&self, other: &ProcedureType, op: BinOperation) -> Result<Rc< Type >, SemanticErrors> { Err( SemanticErrors::ErrorInBinOperation{ left: other.as_str(), right: self.as_str(), op} ) }
    fn bin_operation_function_type(
        &self,
        other: &FunctionType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }
    fn bin_operation_array_type(
        &self,
        other: &ArrayType,
        op: BinOperation,
    ) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::ErrorInBinOperation {
            left: other.as_str(),
            right: self.as_str(),
            op,
        })
    }

    fn cast_to(&self, other: Rc<Type>) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }

    fn cast_from_integer(&self, other: &IntegerType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_double(&self, other: &DoubleType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_char(&self, other: &CharType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_range(&self, other: &RangeType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_enum(&self, other: &EnumType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_array(&self, other: &ArrayType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_record(&self, other: &RecordType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_function(&self, other: &FunctionType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
    fn cast_from_void(&self, other: &VoidType) -> Result<Rc<Type>, SemanticErrors> {
        Err(SemanticErrors::CastError {
            this: other.as_str(),
            other: self.as_str(),
        })
    }
}
