use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use support::*;

#[derive(Clone)]
pub struct ForNode {
    pub id: Rc<Node>,

    pub start: Rc<Node>,
    pub finish: Rc<Node>,

    pub block: Rc<Node>,
}

impl ForNode {
    pub fn new(
        id: Rc<Node>,
        start: Rc<Node>,
        finish: Rc<Node>,
        block: Rc<Node>,
    ) -> Result<ForNode, SemanticErrors> {
        let id_type = id.get_type().unwrap();
        let start_type = start.get_type().unwrap();
        let finish_type = finish.get_type().unwrap();

        if !id_type.is_enumerated() {
            return Err(SemanticErrors::OtherError {
                msg: "Ожидался перечислимый тип".to_string(),
            });
        }
        if id_type.as_str() != start_type.as_str() {
            return Err(SemanticErrors::OtherError {
                msg: format!(
                    "Невозможно привести {} к {}",
                    start_type.as_str(),
                    id_type.as_str()
                ),
            });
        }
        if id_type.as_str() != finish_type.as_str() {
            return Err(SemanticErrors::OtherError {
                msg: format!(
                    "Невозможно привести {} к {}",
                    finish_type.as_str(),
                    id_type.as_str()
                ),
            });
        }

        Ok(ForNode {
            id,
            start,
            finish,
            block,
        })
    }
}

impl Display for ForNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for ForNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        vec![
            self.id.as_printable(),
            self.start.as_printable(),
            self.finish.as_printable(),
            self.block.as_printable(),
        ]
    }
    fn get_caption(&self) -> String {
        "For statement".to_string()
    }
}

impl Node for ForNode {
    fn get_name(&self) -> String {
        "".to_string()
    }
    fn get_kind(&self) -> KindIdentifier {
        KindIdentifier::Other
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }
}
