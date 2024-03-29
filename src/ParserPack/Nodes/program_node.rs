use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

#[derive(Clone)]
pub struct ProgramNode {
    pub name: String,
    pub childrens: Vec<Rc<Node>>,
}

impl ProgramNode {
    pub fn new(name: String) -> ProgramNode {
        ProgramNode {
            name: name,
            childrens: vec![],
        }
    }
}

impl Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for ProgramNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        let mut ans: Vec<&PrintableNode> = vec![];
        for child in &self.childrens {
            ans.push(child.as_printable())
        }
        ans
    }
    fn get_caption(&self) -> String {
        self.name.to_string()
    }
}
impl Node for ProgramNode {
    fn add_child(&mut self, new_node: Rc<Node>) {
        self.childrens.push(new_node);
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }

    fn generate(&self, generator: &mut Generator) {
        for child in &self.childrens {
            child.generate(generator);
        }
    }
}
