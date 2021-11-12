use GeneratorPack::*;
use std::fmt;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

pub struct Tree {
    pub root: Rc<Node>,
}

impl Tree {
    pub fn new(root: Rc<Node>) -> Tree {
        Tree { root }
    }

    pub fn generate(&self, generator: &mut Generator) {
        self.root.generate(generator);
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.root)
    }
}
