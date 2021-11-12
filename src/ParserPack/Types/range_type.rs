use std::i32;
use std::rc::Rc;
use ParserPack::*;
use support::*;
use std::cell::Cell;

pub struct RangeType {
    left: i32,
    right: i32,

    value: Rc<Type>,
    pub kind: Cell<TypeKind>,
    pub is_unknown: Cell<bool>,
}

impl RangeType {
    pub fn new(left: i32, right: i32, value: Rc<Type>) -> Result<RangeType, SemanticErrors> {
        if right < left {
            return Err(SemanticErrors::OtherError {
                msg: "Неверно заданы границы range".to_string(),
            });
        }
        Ok(RangeType {
            left,
            right,
            value: value.get_clone(),
            kind: Cell::new(TypeKind::Var),
            is_unknown: Cell::new(false),
        })
    }
}

impl Type for RangeType {
    fn get_size(&self) -> i64 {
        32
    }
    fn as_str(&self) -> String {
        self.left.to_string() + ".." + &self.right.to_string()
    }
    fn value_as_str(&self) -> String {
        if self.is_unknown.get() {
            return "Unknown".to_string();
        }
        self.value.value_as_str()
    }

    fn is_enumerated(&self) -> bool {
        true
    }
    fn get_left(&self) -> i32 {
        self.left
    }
    fn get_right(&self) -> i32 {
        self.right
    }
    fn set_unknown(&self, is_unknown: bool) {
        self.is_unknown.set(is_unknown);
    }
    fn get_unknown(&self) -> bool {
        self.is_unknown.get()
    }

    fn parse_init_value(&self, parser: &mut Parser) -> Result<String, CompilerErrors> {
        let expr = try!(parser.parse_simple_expr());
        let value = expr.get_type().unwrap();
        try!(self.set_value(value));
        Ok("Ok".to_string())
    }

    fn set_value(&self, value: Rc<Type>) -> Result<String, SemanticErrors> {
        try!(self.value.set_value(value));
        Ok("Ok".to_string())
    }

    fn as_integer(&self) -> Option<i64> {
        self.value.as_integer()
    }
    fn as_double(&self) -> Option<f64> {
        self.value.as_double()
    }
    fn as_enum(&self, name: String) -> Option<i64> {
        self.value.as_enum(name)
    }

    fn get_clone(&self) -> Rc<Type> {
        let e = RangeType {
            left: self.left,
            right: self.right,
            value: self.value.get_clone(),
            kind: self.kind.clone(),
            is_unknown: self.is_unknown.clone(),
        };
        Rc::new(e)
    }

    fn set_kind(&self, kind: TypeKind) {
        self.kind.set(kind);
    }

    fn unar_operation(&self, op: UnarOperation) -> Result<Rc<Type>, SemanticErrors> {
        self.value.unar_operation(op)
    }
    fn bin_operation(&self, other: Rc<Type>, op: BinOperation) -> Result<Rc<Type>, SemanticErrors> {
        self.value.bin_operation(other, op)
    }
    fn cast_to(&self, other: Rc<Type>) -> Result<Rc<Type>, SemanticErrors> {
        self.value.cast_to(other)
    }
}
