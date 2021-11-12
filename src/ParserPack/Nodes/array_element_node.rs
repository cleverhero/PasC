use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;
use support::*;

#[derive(Clone)]
pub struct ArrayElementNode {
    pub index: Rc<Node>,
    pub parent: Rc<Node>,
    pub self_type: Rc<Type>,
}

impl ArrayElementNode {
    pub fn new(parent: Rc<Node>, index: Rc<Node>) -> Result<ArrayElementNode, SemanticErrors> {
        let self_type = try!(
            parent
                .get_type()
                .unwrap()
                .get_by_index(index.get_type().unwrap())
        );
        Ok(ArrayElementNode {
            parent,
            index,
            self_type,
        })
    }
}

impl Display for ArrayElementNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for ArrayElementNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        vec![self.index.as_printable(), self.parent.as_printable()]
    }
    fn get_caption(&self) -> String {
        self.get_name() + " : " + &self.self_type.as_str() + " = " + &self.self_type.value_as_str()
    }
}

impl Node for ArrayElementNode {
    fn get_type(&self) -> Option<Rc<Type>> {
        Some(self.self_type.clone())
    }
    fn get_name(&self) -> String {
        self.parent.get_name() + "[" + &self.index.get_type().unwrap().value_as_str() + "]"
    }
    fn get_kind(&self) -> KindIdentifier {
        KindIdentifier::Other
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }
}
