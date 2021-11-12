use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;

#[derive(Clone)]
pub struct MainNode {
    pub name: String,
    pub childrens: Vec<Rc<Node>>,
}

impl MainNode {
    pub fn new(name: String) -> MainNode {
        MainNode {
            name: name,
            childrens: vec![],
        }
    }
}

impl Display for MainNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for MainNode {
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
impl Node for MainNode {
    fn add_child(&mut self, new_node: Rc<Node>) {
        self.childrens.push(new_node);
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }

    fn generate(&self, generator: &mut Generator) {
        let command = Command::create_decl_function("main".to_string());
        generator.push_to_text_section(command);

        for child in &self.childrens {
            child.generate(generator);
        }

        let command = Command::create_ret(0);
        generator.push_to_text_section(command);
    }
}
