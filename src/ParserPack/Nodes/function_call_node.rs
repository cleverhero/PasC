use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use ParserPack::*;
use support::*;

#[derive(Clone)]
pub struct FunctionCallNode {
    pub func_name: String,
    pub parent: Rc<Type>,
    pub self_type: Rc<Type>,
}

impl FunctionCallNode {
    pub fn new(func_name: String, parent: Rc<Type>) -> Result<FunctionCallNode, SemanticErrors> {
        let self_type = try!(parent.call_by_args());
        Ok(FunctionCallNode {
            func_name,
            parent,
            self_type,
        })
    }
}

impl Display for FunctionCallNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for FunctionCallNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        vec![]
    }
    fn get_caption(&self) -> String {
        self.get_name() + ": " + &self.self_type.as_str() + " = " + &self.self_type.value_as_str()
    }
}

impl Node for FunctionCallNode {
    fn get_type(&self) -> Option<Rc<Type>> {
        Some(self.self_type.clone())
    }
    fn get_name(&self) -> String {
        self.func_name.clone() + &self.parent.value_as_str()
    }
    fn get_kind(&self) -> KindIdentifier {
        KindIdentifier::Other
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }
}
