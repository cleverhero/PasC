#![allow(unused_variables)]
use GeneratorPack::*;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use ParserPack::Nodes::support::*;
use support::*;
use ParserPack::*;

#[derive(Clone)]
pub struct WritelnNode {
    pub args: Vec<Rc<Node>>,
}

impl WritelnNode {
    pub fn new(args: Vec<Rc<Node>>) -> Result<WritelnNode, SemanticErrors> {
        Ok(WritelnNode { args })
    }
}

impl Display for WritelnNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ans = self.as_str("".to_string(), true);
        write!(f, "{}", ans)
    }
}

impl PrintableNode for WritelnNode {
    fn get_children(&self) -> Vec<&PrintableNode> {
        let mut ans: Vec<&PrintableNode> = vec![];
        for child in &self.args {
            ans.push(child.as_printable())
        }
        ans
    }
    fn get_caption(&self) -> String {
        "Writeln".to_string()
    }
}

impl Node for WritelnNode {
    fn get_name(&self) -> String {
        "".to_string()
    }
    fn get_kind(&self) -> KindIdentifier {
        KindIdentifier::Other
    }
    fn as_printable(&self) -> &PrintableNode {
        self
    }

    fn generate(&self, generator: &mut Generator) {
        let mut format = "".to_string();
        let mut fcount = 0;

        for arg in &self.args {
            let ttype = arg.get_type().unwrap();
            match ttype.get_value() {
                ValueVariant::Int { v } => {
                    format += "%d";
                }
                ValueVariant::Double { v } => {
                    format += "%f";
                }
                ValueVariant::Char { v } => {
                    format += "%c";
                }
                ValueVariant::Enum { name, v } => {
                    format += "%d";
                }
                ValueVariant::Other => {}
            }
        }

        for arg in self.args.clone().iter().rev() {
            arg.generate(generator);

            match arg.get_type().unwrap().get_value() {
                ValueVariant::Double { v } => {
                    fcount += 1;
                    generator.push_to_text_section(Command::create_float_to_double());
                }
                _ => {}
            }
        }

        let format_name = generator.next_format();

        let command = Command::create_format_string(format_name.clone(), format);
        generator.push_to_data_section(command);

        let command = Command::create_push_global_var(format_name);
        generator.push_to_text_section(command);

        let command = Command::create_call_func("_printf".to_string());
        generator.push_to_text_section(command);

        let command = Command::create_clear_stack(4 * ((self.args.len() + fcount + 1) as i32));
        generator.push_to_text_section(command);
    }
}
