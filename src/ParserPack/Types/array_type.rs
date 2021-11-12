use support::*;
use std::rc::Rc;
use ParserPack::*;
use TokenizerPack::support::*;
use std::cell::Cell;

pub struct ArrayType {
    elements: Vec<Rc<Type>>,

    index_type: Rc<Type>,
    out_type: Rc<Type>,

    pub kind: Cell<TypeKind>,
    pub is_unknown: Cell<bool>,
}

impl ArrayType {
    pub fn new(index_type: Rc<Type>, out_type: Rc<Type>) -> Result<ArrayType, CompilerErrors> {
        if !index_type.is_enumerated() {
            return Err(CompilerErrors::from(SemanticErrors::OtherError {
                msg: "Ожидался перечислимый тип".to_string(),
            }));
        }
        if (index_type.get_right() as i64 - index_type.get_left() as i64) * out_type.get_size()
            > MAX_SIZE
        {
            return Err(CompilerErrors::from(SemanticErrors::OtherError {
                msg: "Слишком много элементов массива".to_string(),
            }));
        }

        let count = index_type.get_right() as i64 - index_type.get_left() as i64;
        let mut elements: Vec<Rc<Type>> = vec![];
        for _i in 0..count + 1 {
            elements.push(out_type.get_clone());
        }

        Ok(ArrayType {
            index_type,
            out_type,
            elements,
            kind: Cell::new(TypeKind::Var),
            is_unknown: Cell::new(false),
        })
    }
}

impl Type for ArrayType {
    fn get_size(&self) -> i64 {
        self.elements.len() as i64 * self.out_type.get_size()
    }
    fn as_str(&self) -> String {
        let ans =
            "Array[".to_string() + &self.index_type.as_str() + "] of " + &self.out_type.as_str();
        ans
    }

    fn set_unknown(&self, is_unknown: bool) {
        self.is_unknown.set(is_unknown);
    }
    fn get_unknown(&self) -> bool {
        self.is_unknown.get()
    }

    fn value_as_str(&self) -> String {
        if self.is_unknown.get() {
            return "Unknown".to_string();
        }
        let mut ans = "[".to_string();
        for element in &self.elements {
            ans += &(element.value_as_str() + ", ");
        }
        (ans[..ans.len() - 2].to_string() + "]")
    }

    fn get_by_index(&self, index: Rc<Type>) -> Result<Rc<Type>, SemanticErrors> {
        let ind = match index.get_value() {
            ValueVariant::Int { v } => match self.index_type.as_integer() {
                Some(_res) => v,
                None => {
                    return Err(self.create_err(format!(
                        "Неверный тип индекса {}, ожидался {}",
                        index.as_str(),
                        self.index_type.as_str()
                    )))
                }
            },
            ValueVariant::Enum { name, v } => match self.index_type.as_enum(name) {
                Some(_res) => v,
                None => {
                    return Err(self.create_err(format!(
                        "Неверный тип индекса {}, ожидался {}",
                        index.as_str(),
                        self.index_type.as_str()
                    )))
                }
            },
            _ => {
                return Err(self.create_err(format!(
                    "Неверный тип индекса {}",
                    index.as_str()
                )))
            }
        };
        Ok(self.elements[ind as usize].clone())
    }

    fn parse_init_value(&self, parser: &mut Parser) -> Result<String, CompilerErrors> {
        check_token!(parser, TokenType::TObr);
        for i in 0..self.elements.len() {
            try!(self.elements[i].parse_init_value(parser));
            if i < self.elements.len() - 1 {
                check_token!(parser, TokenType::TComma);
            }
        }
        check_token!(parser, TokenType::TCbr);
        Ok("Array".to_string())
    }

    fn set_kind(&self, kind: TypeKind) {
        for elem in &self.elements {
            elem.set_kind(kind.clone());
        }
        self.kind.set(kind);
    }
    fn get_clone(&self) -> Rc<Type> {
        let clone = match ArrayType::new(self.index_type.clone(), self.out_type.clone()) {
            Ok(res) => res,
            Err(_err) => panic!(),
        };
        Rc::new(clone)
    }

    fn bin_operation(&self, other: Rc<Type>, op: BinOperation) -> Result<Rc<Type>, SemanticErrors> {
        other.bin_operation_array_type(self, op)
    }

    fn cast_to(&self, other: Rc<Type>) -> Result<Rc<Type>, SemanticErrors> {
        other.cast_from_array(self)
    }
}
