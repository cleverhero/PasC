use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

#[derive(Clone)]
pub struct DeclVarListNode {
    pub name: String,

    pub fields: Vec<Rc<Node>>,
}

impl DeclVarListNode {
    pub fn new(name: String, fields: Vec<Rc<Node>>) -> DeclVarListNode {
        DeclVarListNode { name, fields }
    }
}

impl Display for DeclVarListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for DeclVarListNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        let mut ans: Vec<&PrintableNode> = vec![];
        for field in &self.fields {
            ans.push(field.as_printable())
        }
        ans
    }
    fn get_caption(&self) -> String {
        self.name.to_string()
    }
}

impl Node for DeclVarListNode {
    fn as_printable(&self) -> &PrintableNode {
        self
    }

    fn generate(&self, generator: &mut Generator) {
        for child in &self.fields {
            child.generate(generator);
        }
    }
}
