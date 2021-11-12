#![allow(unused_variables)]
use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;

#[derive(Clone)]
pub struct ConstNode {
    pub self_type: Rc<Type>,
}

impl ConstNode {
    pub fn new(self_type: Rc<Type>) -> ConstNode {
        ConstNode { self_type }
    }
}

impl Display for ConstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for ConstNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        vec![]
    }
    fn get_caption(&self) -> String {
        self.self_type.value_as_str() + ": " + &self.self_type.as_str()
    }
}

impl Node for ConstNode {
    fn get_type(&self) -> Option<Rc<Type>> {
        Some(self.self_type.clone())
    }
    fn get_name(&self) -> String {
        self.self_type.value_as_str()
    }
    fn get_kind(&self) -> KindIdentifier {
        KindIdentifier::Const
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }

    fn generate(&self, generator: &mut Generator) {
        match self.self_type.get_value() {
            ValueVariant::Int { v } => {
                let command = Command::create_push_const(v as i32);
                generator.push_to_text_section(command);
            }
            ValueVariant::Double { v } => {
                let command = Command::create_push_const((v as f32).to_bits() as i32);
                generator.push_to_text_section(command);
            }
            ValueVariant::Char { v } => {
                let command = Command::create_push_const(v as i32);
                generator.push_to_text_section(command);
            }
            ValueVariant::Enum { name, v } => {}
            ValueVariant::Other => {}
        }
    }
}
